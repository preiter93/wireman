use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::AppContext,
    commons::debug::log_to_file,
    model::{MessagesModel, SelectionModel},
};

/// The input on the select services and methods page
pub struct SelectionInput<'a, 'b> {
    pub model: Rc<RefCell<SelectionModel>>,
    pub messages_model: Rc<RefCell<MessagesModel<'a>>>,
    pub context: &'b mut AppContext,
}

impl SelectionInput<'_, '_> {
    pub fn handle(&mut self, code: KeyCode) {
        match code {
            KeyCode::Enter if self.context.sub == 0 => {
                self.context.sub = 1;
                self.model.borrow_mut().next_method();
            }
            KeyCode::Enter if self.context.sub == 1 => {
                self.context.tab = self.context.tab.next();
                self.context.sub = 0;
            }
            KeyCode::Esc if self.context.sub == 1 => {
                self.context.sub = 0;
                self.model.borrow_mut().clear_method();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.context.sub == 0 {
                    self.model.borrow_mut().next_service();
                } else {
                    self.model.borrow_mut().next_method();
                }
                if let Some(method) = self.model.borrow().selected_method() {
                    self.messages_model.borrow_mut().load_method(&method);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.context.sub == 0 {
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

/// The input on the messages page.
pub struct MessagesInput<'a, 'b> {
    pub model: Rc<RefCell<MessagesModel<'a>>>,
    pub sub_index: usize,
    pub context: &'b mut AppContext,
}

impl MessagesInput<'_, '_> {
    pub fn handle(&self, event: KeyEvent) {
        match event {
            _ => {
                log_to_file("a");
                if self.context.sub == 0 {
                    log_to_file("v");
                    self.model.borrow_mut().request.editor.on_key(event);
                }
            }
        }
    }
}
