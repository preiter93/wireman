#![allow(clippy::module_name_repetitions)]
use crate::commons::HelpActions;
use crate::{commons::editor::EditorMode, model::MetadataModel};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::{cell::RefCell, rc::Rc};

type RcMetadataModel<'a> = Rc<RefCell<MetadataModel<'a>>>;

pub struct MetadataController<'a> {
    pub model: RcMetadataModel<'a>,
}

impl<'a> MetadataController<'a> {
    pub fn new(model: RcMetadataModel<'a>) -> Self {
        Self { model }
    }

    /// Handle user input.
    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            if self.in_insert_mode() {
                self.on_key_insert_mode(key);
            } else {
                match key.code {
                    KeyCode::Tab => {
                        if self.is_key_selected() {
                            self.model.borrow_mut().select_val();
                        } else {
                            self.model.borrow_mut().select_key();
                        }
                    }
                    _ => self.on_key_normal_mode(key),
                }
            }
        }
    }

    /// Key bindings in normal mode
    fn on_key_normal_mode(&mut self, key: KeyEvent) {
        if let Some(editor) = self.model.borrow_mut().get_selected_mut() {
            editor.on_key_normal_mode(key);
        }
    }

    /// Key bindings in insert mode
    fn on_key_insert_mode(&mut self, key: KeyEvent) {
        if let Some(editor) = self.model.borrow_mut().get_selected_mut() {
            editor.on_key_insert_mode(key);
        }
    }

    /// Returns wether the editor is in insert mode
    pub fn in_insert_mode(&self) -> bool {
        if let Some(editor) = self.model.borrow().get_selected() {
            return editor.mode() == EditorMode::Insert;
        }
        false
    }

    /// Wheter a key is selected
    fn is_key_selected(&self) -> bool {
        self.model.borrow().is_key_selected()
    }

    /// Return a map of help actions. This is displayed in the
    /// helper wndow.
    pub fn help(&self) -> HelpActions {
        if self.in_insert_mode() {
            let mut actions = HelpActions::new();
            actions.insert("Esc", "Normal mode");
            actions
        } else {
            let mut actions = HelpActions::new();
            actions.insert("q", "Quit");
            actions.insert("M/Esc", "Untoggle metadata");
            actions.insert("i", "Insert mode");
            actions.insert("y", "Yank");
            actions.insert("p", "Paste");
            if self.is_key_selected() {
                actions.insert("Tab", "Select Value");
            } else {
                actions.insert("Tab", "Select Key");
            }
            actions
        }
    }
}
