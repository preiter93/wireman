#![allow(clippy::module_name_repetitions)]
use crate::{
    commons::{editor::TextEditor, HelpActions},
    model::{
        history::HistoryModel, AddressModel, CoreClient, MessagesModel, MetadataModel,
        SelectionModel,
    },
    AppConfig,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{cell::RefCell, rc::Rc};

/// Translates key input to actions for the models
pub struct Controller<'a> {
    /// The model for the services and methods list
    pub selection: SelectionModel,

    /// The model for the request and response messages
    pub messages: MessagesModel<'a>,

    /// The model for the address field
    pub address: Rc<RefCell<AddressModel<'a>>>,

    /// The model for the metadata field
    pub metadata: Rc<RefCell<MetadataModel<'a>>>,

    /// The model for the request history
    pub history: HistoryModel,

    /// The active window
    pub window: Window,

    ///  Whether to display the help
    pub show_help: bool,

    /// Whether to display the address
    pub show_address: bool,

    /// Whether to display the metadata
    pub show_metadata: bool,

    /// Whether to display the history
    pub show_history: bool,
}

impl<'a> Controller<'a> {
    /// Instantiate the homepage
    pub fn new(core_client: CoreClient, config: AppConfig) -> Self {
        // The core client is shared
        let core_client_rc = Rc::new(RefCell::new(core_client));

        // The selection model
        let selection = SelectionModel::new(Rc::clone(&core_client_rc));

        // The address model
        let default_address = core_client_rc.borrow().get_default_address();
        let address = Rc::new(RefCell::new(AddressModel::new(&default_address)));

        // The metadata model
        let metadata = Rc::new(RefCell::new(MetadataModel::new()));

        // The messages model
        let messages =
            MessagesModel::new(core_client_rc, Rc::clone(&address), Rc::clone(&metadata));

        // The history model
        let history = HistoryModel::new(config.history);

        Self {
            selection,
            messages,
            address,
            metadata,
            history,
            window: Window::Selection,
            show_help: true,
            show_address: false,
            show_metadata: false,
            show_history: false,
        }
    }
    /// Handles key events. Returns true if the app should quit
    pub fn on_event(&mut self, key: KeyEvent) -> bool {
        // Handle events when in insert mode
        if self.insert_mode() {
            self.on_event_insert_mode(key);
            return false;
        }

        // Always quite on q in normal mode
        if key.code == KeyCode::Char('q') {
            return true;
        }

        // Handle events in normal mode
        match self.window {
            Window::Selection => self.on_event_selection(key),
            Window::Request => self.on_event_request(key),
            Window::Metadata => self.on_event_metadata(key),
            Window::Address => self.on_event_address(key),
            Window::History => self.on_event_history(key),
        }
        false
    }

    /// Whether we are in insert mode on any window
    fn insert_mode(&self) -> bool {
        self.messages.request.editor.insert_mode()
            || self.address.borrow().editor.insert_mode()
            || self
                .metadata
                .borrow()
                .get_selected()
                .map_or(false, TextEditor::insert_mode)
    }

    // Handle key events in insert mode
    pub fn on_event_insert_mode(&mut self, key: KeyEvent) {
        match self.window {
            Window::Request => self.messages.request.editor.on_key(key),
            Window::Metadata => {
                let mut model = self.metadata.borrow_mut();
                if let Some(editor) = model.get_selected_mut() {
                    // change between key and value on tab
                    if key.code == KeyCode::Tab {
                        editor.set_normal_mode();
                        if model.is_key_selected() {
                            model.select_val();
                        } else {
                            model.select_key();
                        }
                    } else {
                        editor.on_key(key);
                    }
                }
            }
            Window::Address => self.address.borrow_mut().editor.on_key(key),
            _ => (),
        }
    }

    /// Handle key events on the selection window
    fn on_event_selection(&mut self, key: KeyEvent) {
        let model = &mut self.selection;
        match key.code {
            KeyCode::Tab => {
                self.window = Window::Request;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                model.next();
                if let Some(method) = model.selected_method() {
                    self.messages.load_method(&method);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                model.previous();
                if let Some(method) = model.selected_method() {
                    self.messages.load_method(&method);
                }
            }
            KeyCode::Enter if model.is_parent_selected() => {
                model.expand();
                if let Some(method) = model.selected_method() {
                    self.messages.load_method(&method);
                }
            }
            KeyCode::Esc if !model.is_parent_selected() => {
                model.collapse();
                model.clear_method();
                self.messages.clear_method();
            }
            KeyCode::Char('u') if !model.is_parent_selected() => {
                model.collapse();
                model.clear_method();
                self.messages.clear_method();
            }
            KeyCode::Char('?') => self.toggle_help(),
            KeyCode::Char('A') => self.toggle_address(),
            KeyCode::Char('M') => self.toggle_metadata(),
            _ => {}
        }
    }

    /// Handle key events on the request window
    fn on_event_request(&mut self, key: KeyEvent) {
        let model = &mut self.messages;
        match key.code {
            KeyCode::Tab => {
                self.window = Window::Selection;
            }
            KeyCode::Char('y') if key.modifiers == KeyModifiers::CONTROL => {
                model.yank_grpcurl();
            }
            KeyCode::Enter => {
                model.call_grpc();
            }
            KeyCode::Char('S') => {
                self.history.save(&self.messages);
            }
            KeyCode::Char('?') => self.toggle_help(),
            KeyCode::Char('A') => self.toggle_address(),
            KeyCode::Char('M') => self.toggle_metadata(),
            KeyCode::Char('H') => {
                if let Some(method) = &model.selected_method {
                    self.history.load(method);
                    self.toggle_history();
                }
            }
            _ => model.request.editor.on_key(key),
        }
    }

    /// handle key events on the metadata window
    fn on_event_metadata(&mut self, key: KeyEvent) {
        let model = &mut self.metadata;
        match key.code {
            KeyCode::Tab => {
                if model.borrow().is_key_selected() {
                    model.borrow_mut().select_val();
                } else {
                    model.borrow_mut().select_key();
                }
            }
            KeyCode::Char('?') => self.toggle_help(),
            KeyCode::Char('M') | KeyCode::Esc => self.toggle_metadata(),
            _ => {
                if let Some(editor) = model.borrow_mut().get_selected_mut() {
                    editor.on_key(key);
                }
            }
        }
    }

    /// handle key events on the address window
    fn on_event_address(&mut self, key: KeyEvent) {
        let model = &mut self.address;
        match key.code {
            KeyCode::Char('?') => self.toggle_help(),
            KeyCode::Char('A') | KeyCode::Esc => self.toggle_address(),
            _ => {
                model.borrow_mut().editor.on_key(key);
            }
        }
    }

    /// handle key events on the history window
    fn on_event_history(&mut self, key: KeyEvent) {
        let model = &mut self.history;
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                model.next();
                self.history.apply(&mut self.messages);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                model.previous();
                self.history.apply(&mut self.messages);
            }
            KeyCode::Char('?') => self.toggle_help(),
            KeyCode::Char('H') | KeyCode::Esc | KeyCode::Enter => self.toggle_history(),
            KeyCode::Char('D') => model.delete_selected(),
            _ => {}
        }
    }

    /// Returns the help actions of the current active window
    pub fn help_hint(&self) -> Option<HelpActions> {
        if !self.show_help {
            return None;
        }

        // help in insert mode
        if self.insert_mode() {
            return Some(HelpActions::from_items(vec![("Esc", "Normal mode")]));
        }

        // help in normal mode
        match &self.window {
            Window::Selection => {
                let action = if self.selection.is_parent_selected() {
                    ("Enter", "Select")
                } else {
                    ("Esc", "Collapse")
                };
                Some(HelpActions::from_items(vec![
                    ("q", "Quit"),
                    ("Tab", "Go to Request"),
                    ("j/↓", "down"),
                    ("k/↑", "up"),
                    action,
                ]))
            }
            Window::Request => Some(HelpActions::from_items(vec![
                ("q", "Quit"),
                ("Tab", "Go to Selection"),
                ("i", "Insert mode"),
                ("p", "Paste"),
                ("u", "Undo"),
                ("y", "Yank"),
                ("S", "Save request"),
                ("H", "Show history"),
                ("<C-y>", "Yank as grpcurl"),
                ("Enter", "gRPC request"),
            ])),
            Window::Metadata => {
                let action = if self.metadata.borrow().is_key_selected() {
                    ("Tab", "Select Value")
                } else {
                    ("Tab", "Select Key")
                };
                Some(HelpActions::from_items(vec![
                    ("q", "Quit"),
                    ("M", "Untoggle metadata"),
                    action,
                    ("i", "Insert mode"),
                    ("p", "Paste"),
                    ("u", "Undo"),
                    ("y", "Yank"),
                ]))
            }
            Window::Address => Some(HelpActions::from_items(vec![
                ("q", "Quit"),
                ("A", "Untoggle address"),
                ("i", "Insert mode"),
            ])),
            Window::History => Some(HelpActions::from_items(vec![
                ("q", "Quit"),
                ("H", "Close dialog"),
                ("D", "Delete selected"),
                ("Enter", "gRPC request"),
                ("j/↓", "down"),
                ("k/↑", "up"),
            ])),
        }
    }

    /// Toggle the help window on or off
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Toggle the address window on or off
    pub fn toggle_address(&mut self) {
        self.show_address = !self.show_address;
        if self.show_address {
            self.window = Window::Address;
        } else {
            self.window = Window::Request;
        };
    }

    /// Toggle the metadata window on or off
    pub fn toggle_metadata(&mut self) {
        self.show_metadata = !self.show_metadata;
        if self.show_metadata {
            self.window = Window::Metadata;
        } else {
            self.window = Window::Request;
        };
    }

    /// Toggle the history window on or off
    pub fn toggle_history(&mut self) {
        self.show_history = !self.show_history;
        if self.show_history {
            self.window = Window::History;
        } else {
            self.window = Window::Request;
        };
    }
}
#[derive(PartialEq, Eq)]
pub enum Window {
    Selection,
    Request,
    Address,
    Metadata,
    History,
}
