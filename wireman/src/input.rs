use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use edtui::EditorMode;

use crate::{
    context::{AppContext, MessagesTab, SelectionTab},
    model::{
        headers::{HeadersModel, HeadersSelection},
        MessagesModel, SelectionModel,
    },
};

/// The input on the select services and methods page
pub struct SelectionInput<'a> {
    pub model: Rc<RefCell<SelectionModel>>,
    pub messages_model: Rc<RefCell<MessagesModel>>,
    pub ctx: &'a mut AppContext,
}

impl SelectionInput<'_> {
    #[allow(clippy::too_many_lines)]
    pub fn handle(&mut self, code: KeyCode, modifier: KeyModifiers) {
        let tab = self.ctx.selection_tab;
        match code {
            KeyCode::BackTab if !self.ctx.disable_root_events => {
                self.ctx.tab = self.ctx.tab.prev();
                self.on_navigate();
            }
            KeyCode::Tab if !self.ctx.disable_root_events => {
                self.ctx.tab = self.ctx.tab.next();
                self.on_navigate();
            }
            KeyCode::Tab if !self.ctx.disable_root_events => {
                self.ctx.tab = self.ctx.tab.next();
                self.on_navigate();
            }
            KeyCode::Enter if tab == SelectionTab::Services => {
                self.ctx.selection_tab = SelectionTab::Methods;
                // Select a method if there is none selected yet.
                if self.model.borrow().selected_method().is_none() {
                    self.model.borrow_mut().next_method();
                }
            }
            KeyCode::Enter if tab == SelectionTab::SearchServices => {
                self.ctx.selection_tab = SelectionTab::Services;
                self.ctx.disable_root_events = false;
            }
            KeyCode::Enter if tab == SelectionTab::SearchMethods => {
                self.ctx.selection_tab = SelectionTab::Methods;
                self.ctx.disable_root_events = false;
            }
            KeyCode::Enter if tab == SelectionTab::Methods => {
                if self.model.borrow().selected_method().is_none() {
                    self.model.borrow_mut().next_method();
                } else {
                    self.ctx.tab = self.ctx.tab.next();
                    self.on_navigate();
                }
            }
            KeyCode::Char('c')
                if modifier == KeyModifiers::CONTROL && tab == SelectionTab::Services =>
            {
                self.model.borrow_mut().clear_services_filter();
            }
            KeyCode::Char('c')
                if modifier == KeyModifiers::CONTROL && tab == SelectionTab::Methods =>
            {
                if self.model.borrow_mut().methods_filter.is_some() {
                    self.model.borrow_mut().clear_methods_filter();
                } else {
                    self.ctx.selection_tab = SelectionTab::Services;
                    self.model.borrow_mut().clear_methods_selection();
                }
            }
            KeyCode::Esc if tab == SelectionTab::Services => {
                self.model.borrow_mut().clear_services_filter();
            }
            KeyCode::Esc if tab == SelectionTab::Methods => {
                if self.model.borrow_mut().methods_filter.is_some() {
                    self.model.borrow_mut().clear_methods_filter();
                } else {
                    self.ctx.selection_tab = SelectionTab::Services;
                    self.model.borrow_mut().clear_methods_selection();
                }
            }
            KeyCode::Esc if tab == SelectionTab::SearchServices => {
                self.ctx.selection_tab = SelectionTab::Services;
                self.ctx.disable_root_events = false;
            }
            KeyCode::Esc if tab == SelectionTab::SearchMethods => {
                self.ctx.selection_tab = SelectionTab::Methods;
                self.ctx.disable_root_events = false;
            }
            KeyCode::Char('J') if tab == SelectionTab::Services => {
                self.ctx.selection_tab = SelectionTab::Methods;
            }
            KeyCode::Char('K') if tab == SelectionTab::Methods => {
                self.ctx.selection_tab = SelectionTab::Services;
            }
            KeyCode::Char('j') | KeyCode::Down if tab == SelectionTab::Services => {
                self.model.borrow_mut().next_service();
                self.model.borrow_mut().clear_methods_selection();
            }
            KeyCode::Char('j') | KeyCode::Down if tab == SelectionTab::Methods => {
                self.model.borrow_mut().next_method();
            }
            KeyCode::Char('k') | KeyCode::Up if tab == SelectionTab::Services => {
                self.model.borrow_mut().previous_service();
                self.model.borrow_mut().clear_methods_selection();
            }
            KeyCode::Char('k') | KeyCode::Up if tab == SelectionTab::Methods => {
                self.model.borrow_mut().previous_method();
            }
            KeyCode::Char('/') if tab == SelectionTab::Services => {
                self.ctx.selection_tab = SelectionTab::SearchServices;
                self.ctx.disable_root_events = true;
            }
            KeyCode::Char('/') if tab == SelectionTab::Methods => {
                self.ctx.selection_tab = SelectionTab::SearchMethods;
                self.ctx.disable_root_events = true;
            }
            KeyCode::Backspace if tab == SelectionTab::SearchServices => {
                self.model.borrow_mut().remove_char_services_filter();
            }
            KeyCode::Backspace if tab == SelectionTab::SearchMethods => {
                self.model.borrow_mut().remove_char_methods_filter();
            }
            KeyCode::Char(ch) if tab == SelectionTab::SearchServices => {
                self.model.borrow_mut().push_char_services_filter(ch);
            }
            KeyCode::Char(ch) if tab == SelectionTab::SearchMethods => {
                self.model.borrow_mut().push_char_methods_filter(ch);
            }
            _ => {}
        }
    }

    fn on_navigate(&mut self) {
        if self.ctx.selection_tab == SelectionTab::SearchServices {
            self.ctx.selection_tab = SelectionTab::Services;
        }
        if self.ctx.selection_tab == SelectionTab::SearchMethods {
            self.ctx.selection_tab = SelectionTab::Methods;
        }
        if let Some(method) = self.model.borrow().selected_method() {
            self.messages_model.borrow_mut().load_method(&method);
        } else {
            self.messages_model.borrow_mut().set_no_method_error();
        }
    }
}

/// The input on the messages page.
pub struct MessagesInput<'a> {
    pub model: Rc<RefCell<MessagesModel>>,
    pub ctx: &'a mut AppContext,
}

impl MessagesInput<'_> {
    pub fn handle(&mut self, event: KeyEvent) {
        let tab = self.ctx.messages_tab;
        let modifier = event.modifiers;
        match event.code {
            KeyCode::Esc if !self.ctx.disable_root_events => {
                self.model.borrow_mut().abort_request();
            }
            KeyCode::BackTab if !self.ctx.disable_root_events => {
                self.ctx.tab = self.ctx.tab.prev();
                self.ctx.messages_tab = MessagesTab::default();
            }
            KeyCode::Tab if !self.ctx.disable_root_events => {
                self.ctx.tab = self.ctx.tab.next();
                self.ctx.messages_tab = MessagesTab::default();
            }
            KeyCode::Char('J') if tab == MessagesTab::Request && !self.ctx.disable_root_events => {
                self.ctx.messages_tab = MessagesTab::Response;
            }
            KeyCode::Char('K') if tab == MessagesTab::Response && !self.ctx.disable_root_events => {
                self.ctx.messages_tab = MessagesTab::Request;
            }
            KeyCode::Enter if tab == MessagesTab::Request && !self.ctx.disable_root_events => {
                if self.model.borrow().history_model.autosave {
                    self.model.borrow().history_model.save(&self.model.borrow());
                }
                self.model.borrow_mut().start_request();
            }
            KeyCode::Char('y')
                if modifier == KeyModifiers::CONTROL && !self.ctx.disable_root_events =>
            {
                self.model.borrow_mut().yank_grpcurl();
            }
            KeyCode::Char('f')
                if modifier == KeyModifiers::CONTROL
                    && tab == MessagesTab::Request
                    && !self.ctx.disable_root_events =>
            {
                let request = &mut self.model.borrow_mut().request.editor;
                request.format_json();
            }
            KeyCode::Char('d')
                if modifier == KeyModifiers::CONTROL && !self.ctx.disable_root_events =>
            {
                let method = self.model.borrow().selected_method.clone();
                if let Some(method) = method {
                    self.model.borrow().history_model.delete(&method);
                    self.model.borrow_mut().request.load_template(&method);
                    self.model.borrow_mut().headers_model.borrow_mut().clear();
                }
            }
            KeyCode::Char('s')
                if modifier == KeyModifiers::CONTROL && !self.ctx.disable_root_events =>
            {
                self.model.borrow().history_model.save(&self.model.borrow());
            }
            KeyCode::Char('1') if !self.ctx.disable_root_events => {
                self.handle_history_reload(1);
            }
            KeyCode::Char('2') if !self.ctx.disable_root_events => {
                self.handle_history_reload(2);
            }
            KeyCode::Char('3') if !self.ctx.disable_root_events => {
                self.handle_history_reload(3);
            }
            KeyCode::Char('4') if !self.ctx.disable_root_events => {
                self.handle_history_reload(4);
            }
            KeyCode::Char('5') if !self.ctx.disable_root_events => {
                self.handle_history_reload(5);
            }
            _ => {
                let mut disable_root_events = false;
                if tab == MessagesTab::Request {
                    let request = &mut self.model.borrow_mut().request.editor;
                    request.on_key(event);
                    disable_root_events = !request.normal_mode();
                }
                if tab == MessagesTab::Response {
                    let response = &mut self.model.borrow_mut().response.editor;
                    response.on_key(event);
                    disable_root_events = !response.normal_mode();
                }
                // Disable all root key events if one of the editors went into insert mode
                // to not overwrite keys such as 'q' for quitting.
                self.ctx.disable_root_events = disable_root_events;
            }
        }
    }

    fn handle_history_reload(&mut self, index: usize) {
        let mut model = self.model.borrow_mut();
        model.history_model.select(index);

        let history_model = model.history_model.clone();
        let _ = history_model.load(&mut model);
    }
}

/// The input on the headers page.
pub struct HeadersInput<'a> {
    pub model: Rc<RefCell<HeadersModel>>,
    pub ctx: &'a mut AppContext,
}

impl HeadersInput<'_> {
    pub fn handle(&mut self, event: KeyEvent) {
        const SUBS: usize = 2;
        let mut model = self.model.borrow_mut();
        match event.code {
            KeyCode::Esc if !self.ctx.disable_root_events => {
                model.selected = HeadersSelection::None;
            }
            KeyCode::Tab if !self.ctx.disable_root_events => {
                self.ctx.tab = self.ctx.tab.next();
            }
            KeyCode::BackTab if !self.ctx.disable_root_events => {
                self.ctx.tab = self.ctx.tab.prev();
            }
            KeyCode::Char('k') | KeyCode::Up
                if !(self.ctx.disable_root_events
                    || model.selected == HeadersSelection::Meta && model.meta.block_prev()) =>
            {
                model.selected = model.prev();
            }
            KeyCode::Char('j') | KeyCode::Down
                if !(self.ctx.disable_root_events
                    || model.selected == HeadersSelection::Meta && model.meta.block_next()) =>
            {
                model.selected = model.next();
            }
            KeyCode::Enter if model.selected != HeadersSelection::None => {
                if let Some(editor) = model.selected_editor_mut() {
                    editor.state.mode = EditorMode::Normal;
                }
            }
            // Prevent editors going into search mode
            KeyCode::Char('/') => {}
            _ => {
                let selected = model.selected.clone();
                match selected {
                    HeadersSelection::Addr => model.addr.on_key(event),
                    HeadersSelection::Auth => model.auth.on_key(event),
                    HeadersSelection::Meta => model.meta.on_key(event),
                    HeadersSelection::None => match event.code {
                        KeyCode::Enter => {
                            model.selected = HeadersSelection::Addr;
                        }
                        KeyCode::Char('h' | 'a') if event.modifiers == KeyModifiers::CONTROL => {
                            model.meta.add();
                            model.selected = HeadersSelection::Meta;
                        }
                        _ => {}
                    },
                }
                // Disable all root key events unless all editors are in normal mode.
                self.ctx.disable_root_events = model.mode() != EditorMode::Normal;
                // Make sure that a valid field is selected
                if selected == HeadersSelection::Meta && model.meta.headers.is_empty() {
                    model.selected = HeadersSelection::None;
                }
            }
        }
    }
}
