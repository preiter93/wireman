use super::try_expand;
use crate::widgets::editor::TextEditor;
use edtui::EditorMode;

pub struct AuthHeader {
    pub(crate) bearer: TextEditor,
    pub(crate) basic: TextEditor,
    pub(crate) selected: AuthSelection,
}

impl Default for AuthHeader {
    fn default() -> Self {
        Self {
            bearer: TextEditor::single(),
            basic: TextEditor::single(),
            selected: AuthSelection::Bearer,
        }
    }
}

/// The selection state of `HeadersModel`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum AuthSelection {
    #[default]
    Bearer,
    Basic,
}

impl AuthHeader {
    pub fn next(&mut self) {
        match self.selected {
            AuthSelection::Bearer => self.selected = AuthSelection::Basic,
            AuthSelection::Basic => self.selected = AuthSelection::Bearer,
        }
    }

    pub fn insert_mode(&self) -> bool {
        self.bearer.insert_mode() || self.basic.insert_mode()
    }

    pub fn mode(&self) -> EditorMode {
        match self.selected {
            AuthSelection::Bearer => self.bearer.state.mode,
            AuthSelection::Basic => self.basic.state.mode,
        }
    }

    pub fn key() -> String {
        "authorization".to_string()
    }

    pub fn value(&self) -> String {
        self.get_value(false)
    }

    pub fn value_expanded(&self) -> String {
        self.get_value(true)
    }

    fn get_value(&self, expanded: bool) -> String {
        match self.selected {
            AuthSelection::Bearer => {
                let mut value = self.bearer.get_text_raw();
                if expanded {
                    value = try_expand(&value);
                };
                if value.is_empty() {
                    String::new()
                } else {
                    "Bearer ".to_owned() + &value
                }
            }
            AuthSelection::Basic => {
                let mut value = self.basic.get_text_raw();
                if expanded {
                    value = try_expand(&value);
                };
                if value.is_empty() {
                    String::new()
                } else {
                    "Basic ".to_owned() + &value
                }
            }
        }
    }

    pub fn set_text(&mut self, value: &str) {
        if value.starts_with("Bearer ") {
            self.bearer.set_text_raw(&value.replacen("Bearer ", "", 1));
            self.selected = AuthSelection::Bearer;
        }
        if value.starts_with("Basic ") {
            self.basic.set_text_raw(&value.replacen("Basic ", "", 1));
            self.selected = AuthSelection::Basic;
        }
    }

    pub(super) fn clear(&mut self) {
        self.basic.clear();
        self.bearer.clear();
        self.selected = AuthSelection::Bearer;
    }

    pub fn is_empty(&self) -> bool {
        match self.selected {
            AuthSelection::Bearer => self.bearer.is_empty(),
            AuthSelection::Basic => self.basic.is_empty(),
        }
    }

    /// Get the selected editor
    pub fn selected_editor<'b, 'a: 'b>(&'a self) -> &'b TextEditor {
        match self.selected {
            AuthSelection::Bearer => &self.bearer,
            AuthSelection::Basic => &self.basic,
        }
    }
    /// Get the selected editor
    pub fn selected_editor_mut<'b, 'a: 'b>(&'a mut self) -> &'b mut TextEditor {
        match self.selected {
            AuthSelection::Bearer => &mut self.bearer,
            AuthSelection::Basic => &mut self.basic,
        }
    }
}
