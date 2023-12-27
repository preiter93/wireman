#![allow(clippy::module_name_repetitions)]
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

    /// A flag indicating whether a request is being processed.
    pub is_processing: bool,
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
            is_processing: false,
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

    // Build the grpc request
    pub fn build_request(&self, method: &MethodDescriptor) -> Result<RequestMessage, ErrorKind> {
        let mut req = self.request.core_client.borrow().get_request(method);
        // Message
        if let Err(err) = req
            .message_mut()
            .from_json(&self.request.editor.get_text_raw())
        {
            return Err(ErrorKind::default_error(err.to_string()));
        }

        // Auth token
        let auth = self.headers_model.borrow().auth.value_expanded();
        if !auth.is_empty() {
            let _ = req.insert_metadata("authorization", &auth);
        }

        // Metadata headers
        for (key, val) in self.headers_model.borrow().meta.headers_raw() {
            if !key.is_empty() {
                let _ = req.insert_metadata(&key, &val);
            }
        }

        // Address
        req.set_address(&self.headers_model.borrow().address());
        Ok(req)
    }

    /// This method is called before `call_grpc` to give the ui an
    /// indication that a request is in process. The actual grpc
    /// request is done on the next frame.
    pub fn start_request(&mut self) {
        self.is_processing = true;
        self.response.editor.set_text_raw("Processing...");
    }

    /// Make a grpc call and set response or error.
    pub fn do_request(&mut self) {
        self.response.editor.set_error(None);
        self.response.editor.set_text_raw("");
        let Some(method) = self.selected_method.clone() else {
            let err = ErrorKind::default_error("Select a method!");
            self.response.editor.set_error(Some(err.clone()));
            self.response.editor.set_text_raw(&err.string());
            return;
        };

        let req = match self.build_request(&method) {
            Ok(req) => req,
            Err(err) => {
                self.response.editor.set_error(Some(err.clone()));
                self.response.editor.set_text_raw(&err.string());
                return;
            }
        };

        let resp = CoreClient::call_unary(&req);
        self.is_processing = false;

        match resp {
            Ok(resp) => {
                if let Ok(json) = resp.message.to_json() {
                    let formatted_json = try_pretty_format_json(&json);
                    self.response.editor.set_error(None);
                    self.response.editor.set_text_raw(&formatted_json);
                } else {
                    let err = ErrorKind::format_error("failed to parse json".to_string());
                    self.response.editor.set_error(Some(err.clone()));
                    self.response.editor.set_text_raw(&err.string());
                }
            }
            Err(err) => {
                self.response.editor.set_error(Some(err.clone()));
                self.response.editor.set_text_raw(&err.string());
            }
        }
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
