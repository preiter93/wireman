use super::core_client::CoreClient;
use core::MethodDescriptor;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tui_textarea::TextArea;

type EditorMethodMap<'a> = HashMap<String, TextArea<'a>>;
#[derive(Clone)]
pub struct RequestModel<'a> {
    /// The core client retrieves default proto messages
    /// and acts as the gRPC client.
    core_client: Rc<RefCell<CoreClient>>,

    // The currently active editor
    pub editor: TextArea<'a>,

    // Hold one text editor for each proto method
    editor_map: EditorMethodMap<'a>,

    // The id/name of the currently active editor
    active_editor: String,

    // The editor mode
    pub mode: EditorMode,

    // The currently selected method
    pub selected_method: Option<MethodDescriptor>,

    // An optional error message
    pub error: Option<ErrorKind>,

    // An optional response message
    pub response: Option<String>,
}

impl<'a> RequestModel<'a> {
    /// Returns a request model. Requires the core client
    /// which retrieves the proto message and calls the
    /// gRPC client.
    pub fn new(core_client: Rc<RefCell<CoreClient>>) -> Self {
        Self {
            core_client,
            selected_method: None,
            editor: TextArea::new(Vec::new()),
            editor_map: HashMap::new(),
            active_editor: String::new(),
            mode: EditorMode::Normal,
            error: None,
            response: None,
        }
    }

    /// Gets the editors content as raw text
    fn get_text_raw(&self) -> String {
        self.editor.clone().into_lines().join("\n")
    }

    /// Set the editors content from raw text
    fn set_text_raw(&mut self, text: String) {
        self.editor = TextArea::new(text.lines().map(ToOwned::to_owned).collect());
    }

    /// Calls [load_request_template]. Does not load the proto message
    /// if the editor has already text in it
    pub fn load_method(&mut self, method: &MethodDescriptor) {
        // Change editor
        let id = method.name();
        if id != self.active_editor {
            self.change_editor(id);
        }
        // Load the request template (only if the editor is empty)
        if self.editor.is_empty() {
            self.load_request_template(method);
        }
        // Mark method as selected
        self.selected_method = Some(method.clone());
    }

    /// Loads a new request message template
    fn load_request_template(&mut self, method: &MethodDescriptor) {
        let req = self.core_client.borrow_mut().get_request(method);
        // Load message in editor
        self.set_text_raw(try_pretty_format_json(&req.to_json()));
    }

    /// Change the editor
    fn change_editor(&mut self, id: &str) {
        // Save the current editor
        let current_id = self.active_editor.clone();
        if let Some(value) = self.editor_map.get_mut(&current_id) {
            *value = self.editor.clone();
        }

        // Create a new editor if there is none for the given key
        if !self.editor_map.contains_key(id) {
            self.editor_map.insert(id.to_string(), TextArea::default());
        }

        // set the active editor
        self.active_editor = id.to_string();
        self.editor = self.editor_map[id].clone();
    }

    /// Pretty format the current editor text. Fails if the text
    /// is not a proper json string.
    pub fn format_json(&mut self) {
        match pretty_format_json(&self.get_text_raw()) {
            Ok(pretty) => {
                self.set_text_raw(pretty);
                self.error = None;
            }
            Err(err) => self.error = Some(err),
        }
    }

    /// Make a grpc call and set response or error
    pub fn call_grpc(&mut self) {
        if let Some(method) = &self.selected_method {
            let mut req = self.core_client.borrow().get_request(method);
            if let Err(err) = req.from_json(&self.get_text_raw()) {
                self.error = Some(ErrorKind::default_error(err.to_string()));
                self.response = None;
                return;
            }
            match self.core_client.borrow_mut().call_unary(&req) {
                Ok(resp) => {
                    self.response = Some(try_pretty_format_json(&resp));
                    self.error = None;
                }
                Err(err) => {
                    self.error = Some(ErrorKind::default_error(err));
                    self.response = None;
                }
            }
        }
    }
}

/// The editor mode, i.e. Normal or Insert.
#[derive(Clone, PartialEq, Eq, Default)]
pub enum EditorMode {
    #[default]
    Normal,
    Insert,
}

/// The error of the request. Can hold a kind value
/// to distinguish between format and gRPC errors.
#[derive(Clone)]
pub struct ErrorKind {
    pub kind: String,
    pub msg: String,
}

impl ErrorKind {
    fn format_error(msg: String) -> Self {
        Self {
            kind: "Format Error".to_owned(),
            msg,
        }
    }

    fn default_error(msg: String) -> Self {
        Self {
            kind: "Error".to_owned(),
            msg,
        }
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(err: serde_json::Error) -> Self {
        Self::format_error(err.to_string())
    }
}

/// Pretty formats a string assuming it is in json format.
/// Returns an error if formatting fails.
fn pretty_format_json(input: &str) -> Result<String, ErrorKind> {
    let parsed = serde_json::from_str::<serde_json::Value>(input)?;
    let pretty = serde_json::to_string_pretty(&parsed)?;
    Ok(pretty)
}

/// Convenienve method to retty format a json string and just return
/// the input if formatting fails.
fn try_pretty_format_json(input: &str) -> String {
    pretty_format_json(input).unwrap_or_else(|_| input.to_string())
}
