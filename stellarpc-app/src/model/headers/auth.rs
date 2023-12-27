use crossterm::event::{KeyCode, KeyEvent};
use edtui::EditorMode;

use crate::commons::editor::TextEditor;

use super::try_expand;

#[derive(Default)]
pub struct AuthHeader {
    pub(crate) bearer: TextEditor,
    pub(crate) basic: TextEditor,
    pub(crate) selected: AuthSelection,
}

/// The selection state of `HeadersModel`.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum AuthSelection {
    #[default]
    Bearer,
    Basic,
}

impl AuthHeader {
    pub fn is_empty(&self) -> bool {
        match self.selected {
            AuthSelection::Bearer => self.bearer.is_empty(),
            AuthSelection::Basic => self.basic.is_empty(),
        }
    }

    pub fn next(&mut self) {
        match self.selected {
            AuthSelection::Bearer => self.selected = AuthSelection::Basic,
            AuthSelection::Basic => self.selected = AuthSelection::Bearer,
        }
    }

    pub fn on_key(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Left if self.mode() == EditorMode::Normal => {
                self.next();
            }
            KeyCode::Right if self.mode() == EditorMode::Normal => {
                self.next();
            }
            _ => match self.selected {
                AuthSelection::Bearer => self.bearer.on_key(event),
                AuthSelection::Basic => self.basic.on_key(event),
            },
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
        self._value(false)
    }

    pub fn value_expanded(&self) -> String {
        self._value(true)
    }

    fn _value(&self, expanded: bool) -> String {
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
}
