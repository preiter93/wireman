#![allow(clippy::module_name_repetitions)]
use tokio::task::JoinHandle;

use super::{core_client::CoreClient, headers::HeadersModel, history::HistoryModel};
use crate::commons::editor::{pretty_format_json, yank_to_clipboard, ErrorKind, TextEditor};
use core::{descriptor::RequestMessage, MethodDescriptor};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    pub headers_model: Rc<RefCell<HeadersModel>>,

    /// The model for the request history.
    pub history_model: HistoryModel,

    /// Wheter a grpc request should be dispatched
    pub dispatch: bool,

    /// The task handler of the grpc request. Is None
    /// if no request is dispatched.
    pub handler: Option<JoinHandle<()>>,
}

impl Default for MessagesModel {
    fn default() -> Self {
        Self::new(
            Rc::new(RefCell::new(CoreClient::default())),
            Rc::new(RefCell::new(HeadersModel::default())),
            HistoryModel::default(),
        )
    }
}

impl MessagesModel {
    /// Instantiates a request and response model and returns
    /// the common messages model.
    pub fn new(
        core_client: Rc<RefCell<CoreClient>>,
        headers_model: Rc<RefCell<HeadersModel>>,
        history_model: HistoryModel,
    ) -> Self {
        let request = RequestModel::new(core_client);
        let response = ResponseModel::new();
        Self {
            request,
            response,
            cache: HashMap::new(),
            loaded_cache_id: String::new(),
            selected_method: None,
            headers_model,
            history_model,
            dispatch: false,
            handler: None,
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
        if self.request.editor.is_empty() && self.history_model.clone().load(self).is_none() {
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

    /// This method is called before `do_request` to give the ui an
    /// indication that a request is in process. The actual grpc
    /// request is done on the next frame.
    pub fn start_request(&mut self) {
        self.dispatch = true;
        self.response.editor.set_text_raw("Processing...");
        self.response.editor.set_error(None);
    }

    /// This method should be called to abort a grpc request.
    pub fn abort_request(&mut self) {
        if let Some(handler) = self.handler.take() {
            handler.abort();
            self.response.editor.set_text_raw("Cancelled");
            self.response.editor.set_error(None);
        }
    }

    // Collect the grpc request
    pub fn collect_request(&mut self) -> Result<RequestMessage, ErrorKind> {
        let Some(method) = self.selected_method.clone() else {
            let err = ErrorKind::default_error("Select a method!");
            self.response.editor.set_error(Some(err.clone()));
            self.response.editor.set_text_raw(&err.string());
            return Err(ErrorKind::default_error("No method selected"));
        };
        let mut req = self.request.core_client.borrow().get_request(&method);

        // Message
        if let Err(err) = req
            .message_mut()
            .from_json(&self.request.editor.get_text_raw())
        {
            return Err(ErrorKind::default_error(err.to_string()));
        }

        // Metadata
        let headers_model = self.headers_model.borrow();
        for (key, val) in headers_model.headers_expanded() {
            if !key.is_empty() {
                let _ = req.insert_metadata(&key, &val);
            }
        }

        // Address
        req.set_address(&headers_model.address());
        Ok(req)
    }

    pub fn apply_template(&mut self) {
        if let Some(method) = &self.selected_method {
            self.request.load_template(method);
        }
    }

    /// Yanks the request message in grpcurl format
    pub fn yank_grpcurl(&mut self) {
        if let Some(method) = &self.selected_method {
            let address = self.headers_model.borrow().address();
            let message = self.request.editor.get_text_raw();
            let header = self.headers_model.borrow().headers();

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
}

/// Make a grpc call and set response or error.
pub async fn do_request(req: RequestMessage) -> RequestResult {
    let resp = CoreClient::call_unary_async(&req).await;

    match resp {
        Ok(resp) => {
            if let Ok(json) = resp.message.to_json() {
                let formatted_json = try_pretty_format_json(&json);
                return RequestResult::data(formatted_json);
            } else {
                let err = ErrorKind::format_error("failed to parse json".to_string());
                return RequestResult::error(err);
            }
        }
        Err(err) => {
            return RequestResult::error(err);
        }
    }
}

#[derive(Default)]
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
            editor.set_text_raw(&text);
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
    core_client: Rc<RefCell<CoreClient>>,

    /// The currently active editor
    pub editor: TextEditor,

    /// The metadata
    pub metadata: String,
}

impl RequestModel {
    /// Returns a request model. Requires the core client which retrieves the
    /// proto message and calls the grpc client.
    pub fn new(core_client: Rc<RefCell<CoreClient>>) -> Self {
        Self {
            core_client,
            editor: TextEditor::new(),
            metadata: String::new(),
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

    pub fn set_text_raw(&mut self, text: &str) {
        self.editor.set_text_raw(text);
    }
}

#[derive(Clone)]
pub struct ResponseModel {
    // The response text field
    pub editor: TextEditor,
}

impl ResponseModel {
    /// Returns a response model.
    pub fn new() -> Self {
        Self {
            editor: TextEditor::new(),
        }
    }

    /// Clears the response
    pub fn clear(&mut self) {
        self.editor = TextEditor::new();
    }
}

/// Convenienve method to retty format a json string and just return
/// the input if formatting fails.
fn try_pretty_format_json(input: &str) -> String {
    pretty_format_json(input).unwrap_or_else(|_| input.to_string())
}
