use crate::{
    commons::{
        editor::ErrorKind,
        editor::{EditorMode, TextEditor},
        HelpActions,
    },
    model::messages::MessagesModel,
};
use core::MethodDescriptor;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

/// Manages the request and response messages.
pub struct MessagesController<'a> {
    /// Business logic of request and responses
    model: MessagesModel<'a>,
}

impl<'a> MessagesController<'a> {
    /// Instantiate a Controller.
    pub fn new(model: MessagesModel<'a>) -> Self {
        Self { model }
    }

    /// Handle user input.
    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            if self.model.request.editor.mode() == EditorMode::Normal {
                self.on_key_normal_mode(key);
            } else {
                self.on_key_insert_mode(key);
            }
        }
    }

    /// Key bindings in normal mode
    fn on_key_normal_mode(&mut self, key: KeyEvent) {
        self.model.request.editor.on_key_normal_mode(key);
        if key.code == KeyCode::Enter {
            self.model.call_grpc();
        }
    }

    /// Key bindings in insert mode
    fn on_key_insert_mode(&mut self, key: KeyEvent) {
        self.model.request.editor.on_key_insert_mode(key);
    }

    /// Return a map of help actions. This is displayed in the
    /// helper wndow.
    pub fn help(&self) -> HelpActions {
        match self.model.request.editor.mode() {
            EditorMode::Normal => {
                let mut actions = HelpActions::default();
                actions.insert("Tab", "Go to Selection");
                actions.insert("i", "Insert mode");
                actions.insert("Enter", "gRPC request");
                actions
            }
            EditorMode::Insert => {
                let mut actions = HelpActions::new();
                actions.insert("Esc", "Normal mode");
                actions
            }
        }
    }

    /// Load a method in the request model
    pub fn load_method(&mut self, method: &MethodDescriptor) {
        self.model.load_method(method)
    }

    /// Returns the error to be displayed.
    pub fn get_error(&'a self) -> Option<ErrorKind> {
        self.model.request.editor.get_error()
    }

    /// Returns the request editor widget
    pub fn get_editor_request(&self) -> &'a TextEditor {
        &self.model.request.editor
    }

    /// Returns the response editor widget
    pub fn response_string(&self) -> String {
        self.model.response.text.get_text_raw()
    }

    /// Returns wether the editor is in insert mode
    pub fn in_insert_mode(&self) -> bool {
        self.model.request.editor.mode() == EditorMode::Insert
    }

    /// Set the metadadata
    pub fn set_metadata(&mut self, metadata: String) {
        self.model.request.set_metadata(metadata);
    }
    // // Unfortunately the editor style is stored in the text area widget in the
    // // model, so the model has some presentation logic responsibilities.
    // pub fn set_request_widget_style(
    //     &mut self,
    //     cursor_line_style: Style,
    //     block: Block<'a>,
    //     cursor_style: Style,
    // ) {
    //     // Set the cursor line style
    //     self.model
    //         .request
    //         .editor
    //         .set_cursor_line_style(cursor_line_style);
    //     self.model.request.editor.set_block(block);
    //     // Set the cursor style depending on the mode
    //     self.model.request.editor.set_cursor_style(cursor_style);
    // }
}
