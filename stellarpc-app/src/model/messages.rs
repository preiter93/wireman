#![allow(clippy::module_name_repetitions)]
use super::{core_client::CoreClient, history::HistoryData, AddressModel, MetadataModel};
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
    pub(crate) selected_method: Option<MethodDescriptor>,

    /// A reference to the address model
    pub(super) address_model: Rc<RefCell<AddressModel<'a>>>,

    /// A reference to the address model
    pub(super) metadata_model: Rc<RefCell<MetadataModel<'a>>>,
}

impl<'a> MessagesModel<'a> {
    /// Instantiates a request and response model and returns
    /// the common messages model.
    pub fn new(
        core_client: Rc<RefCell<CoreClient>>,
        address_model: Rc<RefCell<AddressModel<'a>>>,
        metadata_model: Rc<RefCell<MetadataModel<'a>>>,
    ) -> Self {
        let request = RequestModel::new(core_client);
        let response = ResponseModel::new();
        Self {
            request,
            response,
            cache: HashMap::new(),
            loaded_cache_id: String::new(),
            selected_method: None,
            address_model,
            metadata_model,
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
        // Load the request template (only if the editor is empty)
        if self.request.editor.is_empty() {
            self.request.load_request_template(method);
        }
        // Mark method as selected
        self.selected_method = Some(method.clone());
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

    /// Make a grpc call and set response or error.
    pub fn call_grpc(&mut self) {
        if let Some(method) = &self.selected_method {
            // Message
            let mut req = self.request.core_client.borrow().get_request(method);
            if let Err(err) = req.message.from_json(&self.request.editor.get_text_raw()) {
                // Acquiring the request message failed
                let err = ErrorKind::default_error(err.to_string());
                self.request.editor.set_error(Some(err));
                self.response.clear();
                return;
            }

            // Metadata
            let metadata_map = self.metadata_model.borrow().as_raw();
            for (key, val) in metadata_map {
                let result = req.insert_metadata(&key, &val);
                if result.is_err() {
                    let err = ErrorKind::format_error("failed to insert metadata".to_string());
                    self.request.editor.set_error(Some(err));
                    self.response.clear();
                    return;
                }
            }

            // Address
            let address = self.address_model.borrow().editor.get_text_raw();

            // Request
            let resp = self
                .request
                .core_client
                .borrow_mut()
                .call_unary(&req, &address);

            match resp {
                Ok(resp) => {
                    if let Ok(resp) = resp.message.to_json() {
                        let resp = try_pretty_format_json(&resp);
                        self.request.editor.set_error(None);
                        self.response.editor.set_text_raw(&resp);
                    } else {
                        let err = ErrorKind::format_error("failed to parse json".to_string());
                        self.request.editor.set_error(Some(err));
                        self.response.clear();
                    }
                }
                Err(err) => {
                    self.request.editor.set_error(Some(err));
                    self.response.clear();
                }
            }
        } else {
            self.request
                .editor
                .set_error(Some(ErrorKind::default_error("Select a method!")));
        }
    }

    pub fn apply_template(&mut self) {
        if let Some(method) = &self.selected_method {
            self.request.load_request_template(method);
        }
    }

    pub fn apply_history(&mut self, history: &HistoryData) {
        *self.metadata_model.borrow_mut() = MetadataModel::from_raw(&history.metadata);
        *self.address_model.borrow_mut() = AddressModel::new(&history.address);
        self.request.editor.set_text_raw(&history.message);
    }

    /// Yanks the request message in grpcurl format
    pub fn yank_grpcurl(&mut self) {
        if let Some(method) = &self.selected_method {
            let address = self.address_model.borrow().editor.get_text_raw();

            let message = self.request.editor.get_text_raw();

            let metadata_map = self.metadata_model.borrow().as_raw();

            if let Ok(grpcurl) =
                self.request
                    .core_client
                    .borrow()
                    .grpcurl(&message, method, metadata_map, &address)
            {
                TextEditor::yank_to_clipboard(&grpcurl);
            }
        }
    }
}

#[derive(Clone)]
pub struct RequestModel<'a> {
    /// The core client retrieves default proto message and making grpc calls.
    core_client: Rc<RefCell<CoreClient>>,

    /// The currently active editor
    pub editor: TextEditor<'a>,

    /// The metadata
    pub metadata: String,
}

impl<'a> RequestModel<'a> {
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
    pub fn load_request_template(&mut self, method: &MethodDescriptor) {
        let req = self
            .core_client
            .borrow_mut()
            .get_request(method)
            .message
            .to_json()
            .map_or("{}".to_string(), |r| try_pretty_format_json(&r));
        self.editor.set_text_raw(&req);
    }
}

#[derive(Clone)]
pub struct ResponseModel<'a> {
    // The response text field
    pub editor: TextEditor<'a>,
}

impl<'a> ResponseModel<'a> {
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
