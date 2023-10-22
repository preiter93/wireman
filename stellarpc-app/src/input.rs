use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::AppContext,
    model::{
        headers::{HeadersModel, HeadersSelection},
        MessagesModel, SelectionModel,
    },
};

/// The input on the select services and methods page
pub struct SelectionInput<'a, 'b> {
    pub model: Rc<RefCell<SelectionModel>>,
    pub messages_model: Rc<RefCell<MessagesModel<'a>>>,
    pub context: &'b mut AppContext,
}

impl SelectionInput<'_, '_> {
    pub fn handle(&mut self, code: KeyCode) {
        const SUBS: usize = 2;
        match code {
            KeyCode::Enter if self.context.sub == 0 => {
                self.context.sub = 1;
                // Select a method if there is none selected yet.
                if self.model.borrow().selected_method().is_none() {
                    self.model.borrow_mut().next_method();
                }
                if let Some(method) = self.model.borrow().selected_method() {
                    self.messages_model.borrow_mut().load_method(&method);
                }
            }
            KeyCode::Enter if self.context.sub == 1 => {
                if self.model.borrow().selected_method().is_none() {
                    // Select a method if there is none selected yet.
                    self.model.borrow_mut().next_method();
                } else {
                    // Otherwise go to next tab
                    self.context.tab = self.context.tab.next();
                    self.context.sub = 0;
                }
            }
            KeyCode::Esc if self.context.sub == 1 => {
                self.context.sub = 0;
                self.model.borrow_mut().clear_method();
            }
            KeyCode::Up => {
                self.context.sub = (self.context.sub + SUBS).saturating_sub(1) % SUBS;
            }
            KeyCode::Down => {
                self.context.sub = self.context.sub.saturating_add(1) % SUBS;
            }
            KeyCode::Char('j') => {
                if self.context.sub == 0 {
                    self.model.borrow_mut().next_service();
                    self.model.borrow_mut().clear_method();
                } else {
                    self.model.borrow_mut().next_method();
                }
                if let Some(method) = self.model.borrow().selected_method() {
                    self.messages_model.borrow_mut().load_method(&method);
                }
            }
            KeyCode::Char('k') => {
                if self.context.sub == 0 {
                    self.model.borrow_mut().previous_service();
                    self.model.borrow_mut().clear_method();
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
    pub context: &'b mut AppContext,
}

impl MessagesInput<'_, '_> {
    pub fn handle(&mut self, event: KeyEvent) {
        const SUBS: usize = 2;
        match event.code {
            KeyCode::Down if !self.context.disable_root_events => {
                self.context.sub = self.context.sub.saturating_add(1) % SUBS;
            }
            KeyCode::Up if !self.context.disable_root_events => {
                self.context.sub = (self.context.sub + SUBS).saturating_sub(1) % SUBS;
            }
            KeyCode::Enter if self.context.sub == 0 && !self.context.disable_root_events => {
                self.model.borrow_mut().call_grpc();
            }
            KeyCode::Char('F') if self.context.sub == 0 && !self.context.disable_root_events => {
                let request = &mut self.model.borrow_mut().request.editor;
                request.format_json();
            }
            _ => {
                let request = &mut self.model.borrow_mut().request.editor;
                if self.context.sub == 0 {
                    request.on_key(event);
                }
                // Disable all root key events if one of the editors went into insert mode
                // to not overwrite keys such as 'q' for quitting.
                self.context.disable_root_events = request.insert_mode();
            }
        }
    }
}

/// The input on the headers page.
pub struct HeadersInput<'a> {
    pub model: Rc<RefCell<HeadersModel>>,
    pub context: &'a mut AppContext,
}

impl HeadersInput<'_> {
    pub fn handle(&mut self, event: KeyEvent) {
        const SUBS: usize = 2;
        match event.code {
            KeyCode::Char('j') | KeyCode::Down if !self.context.disable_root_events => {
                let prev = self.model.borrow().selected.prev();
                self.model.borrow_mut().selected = prev;
            }
            KeyCode::Char('k') | KeyCode::Up if !self.context.disable_root_events => {
                let next = self.model.borrow().selected.next();
                self.model.borrow_mut().selected = next;
            }
            _ => {
                let selected = self.model.borrow().selected.clone();
                match selected {
                    HeadersSelection::Address => self.model.borrow_mut().address.on_key(event),
                    HeadersSelection::Bearer => self.model.borrow_mut().bearer.on_key(event),
                    _ => {}
                }
                // Disable all root key events if one of the editors went into insert mode
                // to not overwrite keys such as 'q' for quitting.
                self.context.disable_root_events = self.model.borrow().bearer.insert_mode();
            }
        }
    }
}
