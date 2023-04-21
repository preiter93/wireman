use super::core_client::CoreClient;
use crate::commons::editor::{pretty_format_json, ErrorKind, TextEditor};
use core::MethodDescriptor;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Map from Method to request/response message
type MessagesCache = HashMap<String, (String, String)>;

pub struct MessagesModel<'a> {
    /// The request editor model
    pub request: RequestModel<'a>,

    /// The response text model
    pub response: ResponseModel<'a>,

    /// Cache previous request/response
    cache: MessagesCache,

    // The id/name of the currently active cache key
    loaded_cache_id: String,

    /// The currently selected method
    selected_method: Option<MethodDescriptor>,
}

impl MessagesModel<'_> {
    /// Instantiates a request and response model and returns
    /// the common messages model.
    pub fn new(core_client: Rc<RefCell<CoreClient>>) -> Self {
        let request = RequestModel::new(core_client);
        let response = ResponseModel::new();
        Self {
            request,
            response,
            cache: HashMap::new(),
            loaded_cache_id: String::new(),
            selected_method: None,
        }
    }

    /// Calls [load_request_template]. Does not load the proto message
    /// if the editor has already text in it
    pub fn load_method(&mut self, method: &MethodDescriptor) {
        // Change editor
        let id = method.name();
        if id != self.loaded_cache_id {
            self.change_method(id);
        }
        // Load the request template (only if the editor is empty)
        if self.request.editor.is_empty() {
            self.request.load_request_template(method);
        }
        // Mark method as selected
        self.selected_method = Some(method.clone());
        // Clear the error state
        self.request.editor.set_error(None);
    }

    /// Change method. Check if the response and request are in cache.
    /// Generate a new cache entry if the method has not been visited.
    fn change_method(&mut self, id: &str) {
        // Save the current editor
        let current_id = self.loaded_cache_id.clone();
        if let Some(value) = self.cache.get_mut(&current_id) {
            *value = (
                self.request.editor.get_text_raw(),
                self.response.text.get_text_raw(),
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
        self.request.editor.set_text_raw(req);
        self.response.text.set_text_raw(resp);
    }

    /// Make a grpc call and set response or error.
    pub fn call_grpc(&mut self) {
        if let Some(method) = &self.selected_method {
            let mut req = self.request.core_client.borrow().get_request(method);
            if let Err(err) = req.from_json(&self.request.editor.get_text_raw()) {
                // Acquiring the request message failed
                let err = ErrorKind::default_error(err.to_string());
                self.request.editor.set_error(Some(err));
                self.response.clear();
                return;
            }
            // Metadata
            // Try do deserialize. If this fails, we will send no metadata
            let map: Result<HashMap<String, String>, serde_json::Error> =
                serde_json::from_str(&self.request.metadata.clone());
            if let Ok(map) = map {
                for (key, val) in map.clone().into_iter() {
                    req.insert_metadata(&key, &val);
                }
            }
            // req.insert_metadata(&map.keys(), &self.request.metadata);
            match self.request.core_client.borrow_mut().call_unary(&req) {
                // Call was successful
                Ok(resp) => {
                    let resp = try_pretty_format_json(&resp.to_json());
                    self.request.editor.set_error(None);
                    self.response.text.set_text_raw(resp);
                }
                // gRPC call returned with an error
                Err(err) => {
                    self.request.editor.set_error(Some(err));
                    self.response.clear();
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct RequestModel<'a> {
    /// The core client retrieves default proto message and making gRPC calls.
    core_client: Rc<RefCell<CoreClient>>,

    /// The currently active editor
    pub editor: TextEditor<'a>,

    /// The metadata
    pub metadata: String,
}

impl<'a> RequestModel<'a> {
    /// Returns a request model. Requires the core client
    /// which retrieves the proto message and calls the
    /// gRPC client.
    pub fn new(core_client: Rc<RefCell<CoreClient>>) -> Self {
        Self {
            core_client,
            editor: TextEditor::new(),
            metadata: String::new(),
        }
    }

    /// Loads a new request message template
    fn load_request_template(&mut self, method: &MethodDescriptor) {
        let req = self.core_client.borrow_mut().get_request(method);
        // Load message in editor
        self.editor
            .set_text_raw(try_pretty_format_json(&req.to_json()));
    }

    /// Set the metadata
    pub fn set_metadata(&mut self, metadata: String) {
        self.metadata = metadata;
    }
}

#[derive(Clone)]
pub struct ResponseModel<'a> {
    // The response text field
    pub text: TextEditor<'a>,
}

impl<'a> ResponseModel<'a> {
    /// Returns a response model.
    pub fn new() -> Self {
        Self {
            text: TextEditor::new(),
        }
    }

    /// Clears the response
    pub fn clear(&mut self) {
        self.text = TextEditor::new();
    }
}

/// Convenienve method to retty format a json string and just return
/// the input if formatting fails.
fn try_pretty_format_json(input: &str) -> String {
    pretty_format_json(input).unwrap_or_else(|_| input.to_string())
}
