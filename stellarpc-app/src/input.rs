use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyCode;

use crate::model::{MessagesModel, SelectionModel};

/// The page where to select services and methods.
pub struct SelectionInput<'a> {
    pub model: Rc<RefCell<SelectionModel>>,
    pub messages_model: Rc<RefCell<MessagesModel<'a>>>,
    pub sub_index: usize,
}

impl SelectionInput<'_> {
    pub fn handle(&self, code: KeyCode) {
        match code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.sub_index == 0 {
                    self.model.borrow_mut().next_service();
                } else {
                    self.model.borrow_mut().next_method();
                }
                if let Some(method) = self.model.borrow().selected_method() {
                    self.messages_model.borrow_mut().load_method(&method);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.sub_index == 0 {
                    self.model.borrow_mut().previous_service();
                } else {
                    self.model.borrow_mut().previous_method();
                }
                if let Some(method) = self.model.borrow().selected_method() {
                    self.messages_model.borrow_mut().load_method(&method);
                }
            }
            _ => {}
        }
    }
}
