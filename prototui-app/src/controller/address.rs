#![allow(clippy::module_name_repetitions)]
use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyEvent, KeyEventKind};

use crate::{
    commons::{editor::EditorMode, HelpActions},
    model::address::AddressModel,
};

pub struct AddressController<'a> {
    pub model: Rc<RefCell<AddressModel<'a>>>,
}

impl<'a> AddressController<'a> {
    pub fn new(model: Rc<RefCell<AddressModel<'a>>>) -> Self {
        Self { model }
    }

    /// Handle user input.
    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            if self.model.borrow().editor.mode() == EditorMode::Normal {
                self.on_key_normal_mode(key);
            } else {
                self.on_key_insert_mode(key);
            }
        }
    }

    /// Key bindings in normal mode
    fn on_key_normal_mode(&mut self, key: KeyEvent) {
        self.model.borrow_mut().editor.on_key_normal_mode(key);
    }

    /// Key bindings in insert mode
    fn on_key_insert_mode(&mut self, key: KeyEvent) {
        self.model.borrow_mut().editor.on_key_insert_mode(key);
    }

    /// Returns wether the editor is in insert mode
    pub fn in_insert_mode(&self) -> bool {
        self.model.borrow().editor.mode() == EditorMode::Insert
    }

    /// Return a map of help actions. This is displayed in the
    /// helper wndow.
    pub fn help(&self) -> HelpActions {
        match self.model.borrow().editor.mode() {
            EditorMode::Normal => {
                let mut actions = HelpActions::new();
                actions.insert("q", "Quit");
                actions.insert("A/Esc", "Untoggle address");
                actions.insert("i", "Insert mode");
                actions
            }
            EditorMode::Insert => {
                let mut actions = HelpActions::new();
                actions.insert("Esc", "Normal mode");
                actions
            }
        }
    }
}
