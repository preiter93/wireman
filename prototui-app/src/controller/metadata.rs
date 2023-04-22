use crate::commons::editor::ErrorKind;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    commons::editor::{EditorMode, TextEditor},
    model::MetadataModel,
};

pub struct MetadataController<'a> {
    model: MetadataModel<'a>,
}

impl<'a> MetadataController<'a> {
    /// Instantiate the settings page
    pub fn new() -> Self {
        let model = MetadataModel::new();
        Self { model }
    }

    /// The key bindings. Returns with true to indicate
    /// the app to exit.
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        if self.model.editor.mode() == EditorMode::Normal {
            // handle exit event
            if let KeyCode::Char('q') = key.code {
                return true;
            }
            self.model.editor.on_key_normal_mode(key);
        } else {
            // auto-format
            if let KeyCode::Esc = key.code {
                self.model.editor.format_json();
            }
            self.model.editor.on_key_insert_mode(key);
        }

        false
    }

    /// Returns a reference to the metadata editor
    pub fn get_editor(&'a self) -> &'a TextEditor {
        &self.model.editor
    }

    /// Returns the error to be displayed.
    pub fn get_error(&'a self) -> Option<ErrorKind> {
        self.model.editor.get_error()
    }
}
