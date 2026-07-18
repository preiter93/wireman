#![allow(clippy::module_name_repetitions)]
use super::{core_client::CoreClient, headers::HeadersModel, history::HistoryModel};
use crate::widgets::editor::{pretty_format_json, yank_to_clipboard, ErrorKind, TextEditor};
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::{self, stream::once, Stream, StreamExt};
use ratatui::prelude::Rect;
use std::{cell::RefCell, collections::HashMap, pin::Pin, rc::Rc};
use tokio::task::JoinHandle;
use wireman_core::{
    client::tls::TlsConfig,
    descriptor::{response::StreamingResponse, DynamicMessage, RequestMessage, ResponseMessage},
    MethodDescriptor,
};

/// Map from Method to request/response message
type MessagesCache = HashMap<String, (String, String)>;

pub struct MessagesModel {
    /// The request editor model
    pub request: RequestModel,

    /// The response text model
    pub response: ResponseModel,

    /// Cache previous request/response
    cache: MessagesCache,

    // The id/name of the currently active cache key
    loaded_cache_id: String,

    /// The currently selected method
    pub(crate) selected_method: Option<MethodDescriptor>,

    /// A reference to the headers model
    pub headers: Rc<RefCell<HeadersModel>>,

    /// The model for the request history.
    pub history: Rc<RefCell<HistoryModel>>,

    /// Whether a grpc request should be dispatched
    pub dispatch: bool,

    /// The task handler of the grpc request. Is None
    /// if no request is dispatched.
    pub handler: Option<JoinHandle<()>>,

    /// Sender for the outbound message stream of an open client- or bidirectional-streaming call.
    /// `None` when no stream session is active.
    pub stream_tx: Option<UnboundedSender<RequestMessage>>,

    /// Number of messages sent on the current client-side stream.
    pub stream_count: usize,
}

impl Default for MessagesModel {
    fn default() -> Self {
        Self::new(
            Rc::new(RefCell::new(CoreClient::default())),
            Rc::new(RefCell::new(HeadersModel::default())),
            Rc::new(RefCell::new(HistoryModel::default())),
        )
    }
}

impl MessagesModel {
    /// Instantiates a request and response model and returns
    /// the common messages model.
    pub fn new(
        core_client: Rc<RefCell<CoreClient>>,
        headers: Rc<RefCell<HeadersModel>>,
        history: Rc<RefCell<HistoryModel>>,
    ) -> Self {
        let request = RequestModel::new(core_client);
        let response = ResponseModel::new();
        Self {
            request,
            response,
            cache: HashMap::new(),
            loaded_cache_id: String::new(),
            selected_method: None,
            headers,
            history,
            dispatch: false,
            handler: None,
            stream_tx: None,
            stream_count: 0,
        }
    }

    /// Calls [`load_request_template`]. Does not load the proto message
    /// if the editor has already text in it
    pub fn load_method(&mut self, method: &MethodDescriptor) {
        // Change editor
        let id = method.name();
        if id != self.loaded_cache_id {
            self.change_method(id);
        }
        // Mark method as selected
        self.selected_method = Some(method.clone());
        // Load the request message
        let history = self.history.borrow().clone();
        let request_from_history = history.load(self);
        if self.request.editor.is_empty() && request_from_history.is_none() {
            self.request.load_template(method);
        }
        // Clear the error state
        self.request.editor.set_error(None);
    }

    /// Clear the loaded method
    pub fn clear_method(&mut self) {
        self.selected_method = None;
        self.loaded_cache_id.clear();
        self.request.editor.clear();
        self.response.editor.clear();
    }

    /// Dump text into the no method buffer
    pub fn set_no_method_error(&mut self) {
        self.change_method("NO_METHOD_CACHE_ID");
        self.request.set_text("Go back and select a method.");
    }

    /// Change method. Check if the response and request are in cache.
    /// Generate a new cache entry if the method has not been visited.
    fn change_method(&mut self, id: &str) {
        // Save the current editor
        let current_id = self.loaded_cache_id.clone();
        if let Some(value) = self.cache.get_mut(&current_id) {
            *value = (
                self.request.editor.get_text_raw(),
                self.response.editor.get_text_raw(),
            );
        }

        // Create a new cache entry if there is none for the given key
        if !self.cache.contains_key(id) {
            self.cache
                .insert(id.to_string(), (String::new(), String::new()));
        }

        // set the active editor
        self.loaded_cache_id = id.to_string();
        let (req, resp) = self.cache[id].clone();
        self.request.editor.set_text_raw(&req);
        self.response.editor.set_text_raw(&resp);
    }

    /// Returns the request as json string
    pub fn to_json(&self) -> String {
        if let Some(method) = &self.selected_method {
            let mut req = self.request.core_client.borrow().get_request(method);
            match req
                .message_mut()
                .from_json(&self.request.editor.get_text_raw())
            {
                Ok(()) => try_pretty_format_json(&req.to_json().unwrap()),
                Err(_) => String::new(),
            }
        } else {
            String::new()
        }
    }

    /// Marks a request to be dispatched on the next frame, or sends the next
    /// message when a client-side stream is already open.
    pub fn start_request(&mut self) {
        if self.stream_tx.is_some() {
            self.send_stream_message();
            return;
        }
        self.dispatch = true;
        self.response.editor.set_text_raw("Processing...");
        self.response.editor.set_error(None);
    }

    /// This method should be called to abort a grpc request.
    pub fn abort_request(&mut self) {
        self.clear_stream_session();
        if let Some(handler) = self.handler.take() {
            handler.abort();
            self.response.editor.set_text_raw("User cancelled");
            self.response.editor.set_error(None);
        }
    }

    /// Whether the selected method streams messages from the client.
    pub fn is_client_streaming(&self) -> bool {
        self.selected_method
            .as_ref()
            .is_some_and(MethodDescriptor::is_client_streaming)
    }

    /// Opens a client-side stream and returns the receiver feeding the
    /// outbound messages.
    pub fn open_stream(&mut self) -> UnboundedReceiver<RequestMessage> {
        let (tx, rx) = mpsc::unbounded();
        self.stream_tx = Some(tx);
        self.stream_count = 0;
        rx
    }

    /// Pushes a message onto the open stream and updates the status.
    pub fn push_stream_message(&mut self, req: RequestMessage) {
        let Some(tx) = &self.stream_tx else { return };
        if tx.unbounded_send(req).is_err() {
            self.stream_tx = None;
            return;
        }
        self.stream_count += 1;
        self.set_stream_status();
    }

    /// Finishes an open stream (half-close), prompting the server response.
    pub fn finish_stream(&mut self) {
        if self.stream_tx.take().is_some() && !self.is_bidi_streaming() {
            self.response
                .editor
                .set_text_raw("Finished sending. Waiting for response...");
            self.response.editor.set_error(None);
        }
    }

    /// Clears client-side stream session state.
    pub fn clear_stream_session(&mut self) {
        self.stream_tx = None;
        self.stream_count = 0;
    }

    /// Sends the current editor content as the next stream message.
    fn send_stream_message(&mut self) {
        match self.get_request() {
            Ok(req) => self.push_stream_message(req),
            Err(err) => {
                self.response.editor.set_error(Some(err.clone()));
                self.response.editor.set_text_raw(&err.string());
            }
        }
    }

    /// Shows the send count for client-streaming. Bidirectional streams render
    /// live responses instead.
    fn set_stream_status(&mut self) {
        if self.is_bidi_streaming() {
            return;
        }
        let text = format!(
            "Streaming: {} message(s) sent\nEnter: send  Ctrl+d: finish  Esc: cancel",
            self.stream_count
        );
        self.response.editor.set_error(None);
        self.response.editor.set_text_raw(&text);
    }

    fn is_bidi_streaming(&self) -> bool {
        self.selected_method
            .as_ref()
            .is_some_and(|m| m.is_client_streaming() && m.is_server_streaming())
    }

    // Builds the request message from the editor, including metadata and
    // address. For streaming methods this is the first ("head") message.
    pub fn get_request(&mut self) -> Result<RequestMessage, ErrorKind> {
        let Some(method) = self.selected_method.clone() else {
            let err = ErrorKind::default_error("Select a method!");
            self.response.editor.set_error(Some(err.clone()));
            self.response.editor.set_text_raw(&err.string());
            return Err(ErrorKind::default_error("No method selected"));
        };
        let mut req = self.request.core_client.borrow().get_request(&method);

        if let Err(err) = req
            .message_mut()
            .from_json(&self.request.editor.get_text_raw())
        {
            return Err(ErrorKind::default_error(err.to_string()));
        }

        let headers = self.headers.borrow();
        for (key, val) in headers.headers_expanded() {
            if !key.is_empty() {
                let _ = req.insert_metadata(&key, &val);
            }
        }

        req.set_address(&headers.address());
        Ok(req)
    }

    pub fn apply_template(&mut self) {
        if let Some(method) = &self.selected_method {
            self.request.load_template(method);
        }
    }

    /// Yanks the request message.
    pub fn yank_request(&mut self) {
        if self.selected_method.is_some() {
            let message = self.request.editor.get_text_raw();
            yank_to_clipboard(&message);
        }
    }

    /// Yanks the response message.
    pub fn yank_response(&mut self) {
        if self.selected_method.is_some() {
            let message = self.response.editor.get_text_raw();
            yank_to_clipboard(&message);
        }
    }

    /// Yanks the request message in grpcurl format
    pub fn yank_grpcurl(&mut self) {
        if let Some(method) = &self.selected_method {
            let address = self.headers.borrow().address();
            let message = self.request.editor.get_text_raw();
            let header = self.headers.borrow().headers();

            if let Ok(text) = self
                .request
                .core_client
                .borrow()
                .get_grpcurl(&message, method, &header, &address)
            {
                yank_to_clipboard(&text);
            }
        }
    }

    pub fn handle_history_reload(&mut self, index: usize) {
        self.history.borrow_mut().select(index);

        let history = self.history.clone();
        let _ = history.borrow_mut().load(self);
    }
}

pub(crate) async fn unary(req: RequestMessage, tls: Option<TlsConfig>) -> RequestResult {
    let resp: Result<ResponseMessage, ErrorKind> = CoreClient::call_unary_async(&req, tls).await;

    match resp {
        Ok(resp) => unmarshal_message(&resp.message),
        Err(err) => RequestResult::error(err),
    }
}

pub(crate) async fn server_streaming(
    req: RequestMessage,
    tls: Option<TlsConfig>,
) -> Pin<Box<dyn Stream<Item = RequestResult> + Send>> {
    let resp: Result<StreamingResponse, ErrorKind> =
        CoreClient::call_server_streaming(&req, tls).await;

    if let Err(err) = resp {
        return once(async { RequestResult::error(err) }).boxed();
    }

    let resp = resp.unwrap();

    let mapped_stream = resp.inner.map(|message| match message {
        Ok(message) => unmarshal_message(&message.message),
        Err(err) => {
            let kind = ErrorKind::streaming_error(format!("{err}"));
            RequestResult::error(kind)
        }
    });

    Box::pin(mapped_stream)
}

pub(crate) async fn client_streaming(
    head: RequestMessage,
    messages: impl Stream<Item = RequestMessage> + Send + 'static,
    tls: Option<TlsConfig>,
) -> RequestResult {
    let resp: Result<ResponseMessage, ErrorKind> =
        CoreClient::call_client_streaming(&head, messages, tls).await;

    match resp {
        Ok(resp) => unmarshal_message(&resp.message),
        Err(err) => RequestResult::error(err),
    }
}

pub(crate) async fn bidi_streaming(
    head: RequestMessage,
    messages: impl Stream<Item = RequestMessage> + Send + 'static,
    tls: Option<TlsConfig>,
) -> Pin<Box<dyn Stream<Item = RequestResult> + Send>> {
    let resp: Result<StreamingResponse, ErrorKind> =
        CoreClient::call_bidirectional_streaming(&head, messages, tls).await;

    if let Err(err) = resp {
        return once(async { RequestResult::error(err) }).boxed();
    }

    let resp = resp.unwrap();

    let mapped_stream = resp.inner.map(|message| match message {
        Ok(message) => unmarshal_message(&message.message),
        Err(err) => {
            let kind = ErrorKind::streaming_error(format!("{err}"));
            RequestResult::error(kind)
        }
    });

    Box::pin(mapped_stream)
}

#[derive(Default, Clone)]
pub struct RequestResult {
    data: Option<String>,
    error: Option<ErrorKind>,
}
unsafe impl Send for RequestResult {}

impl RequestResult {
    pub fn data(data: String) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }
    pub fn error(error: ErrorKind) -> Self {
        Self {
            data: None,
            error: Some(error),
        }
    }
    pub fn set(&self, editor: &mut TextEditor) {
        if let Some(text) = &self.data {
            editor.set_error(None);
            editor.set_text_raw(text);
        }
        if let Some(error) = &self.error {
            editor.set_error(Some(error.clone()));
            editor.set_text_raw(&error.string());
        }
    }
}

#[derive(Clone)]
pub struct RequestModel {
    /// The core client retrieves default proto message and making grpc calls.
    pub core_client: Rc<RefCell<CoreClient>>,

    /// The currently active editor
    pub editor: TextEditor,

    /// The metadata
    pub metadata: String,

    /// The window size in percentage.
    pub window_size: u16,

    /// Hit-test area of the request editor
    pub content_area: Option<Rect>,
}

impl RequestModel {
    /// Returns a request model. Requires the core client which retrieves the
    /// proto message and calls the grpc client.
    pub fn new(core_client: Rc<RefCell<CoreClient>>) -> Self {
        Self {
            core_client,
            editor: TextEditor::new(),
            metadata: String::new(),
            window_size: 50,
            content_area: None,
        }
    }

    /// Loads a new request message template into the editor.
    pub fn load_template(&mut self, method: &MethodDescriptor) {
        let req = self
            .core_client
            .borrow_mut()
            .get_request(method)
            .message()
            .to_json()
            .map_or("{}".to_string(), |r| try_pretty_format_json(&r));
        self.editor.set_text_raw(&req);
    }

    pub fn set_text(&mut self, text: &str) {
        self.editor.set_text_raw(text);
    }

    pub fn set_error(&mut self, error: ErrorKind) {
        self.editor.set_error(Some(error));
    }

    pub fn increase_window_size(&mut self) {
        self.window_size = (self.window_size + 10).min(90);
    }

    pub fn decrease_window_size(&mut self) {
        self.window_size = self.window_size.saturating_sub(10).max(10);
    }
}

#[derive(Clone)]
pub struct ResponseModel {
    // The response text field
    pub editor: TextEditor,

    /// Hit-test area of the response editor
    pub content_area: Option<Rect>,
}

impl ResponseModel {
    /// Returns a response model.
    pub fn new() -> Self {
        Self {
            editor: TextEditor::new(),
            content_area: None,
        }
    }

    /// Clears the response
    pub fn clear(&mut self) {
        self.editor = TextEditor::new();
    }

    pub fn set_text(&mut self, text: &str) {
        self.editor.set_text_raw(text);
    }

    pub fn set_error(&mut self, error: ErrorKind) {
        self.editor.set_error(Some(error));
    }
}

/// Convenienve method to retty format a json string and just return
/// the input if formatting fails.
fn try_pretty_format_json(input: &str) -> String {
    pretty_format_json(input).unwrap_or_else(|_| input.to_string())
}

fn unmarshal_message(message: &DynamicMessage) -> RequestResult {
    if let Ok(json) = message.to_json() {
        let formatted_json = try_pretty_format_json(&json);
        RequestResult::data(formatted_json)
    } else {
        let err = ErrorKind::format_error("failed to parse json".to_string());
        RequestResult::error(err)
    }
}
