#![allow(clippy::module_name_repetitions)]
pub mod address;
pub mod messages;
pub mod metadata;
pub mod selection;
pub use address::AddressController;
pub use messages::MessagesController;
pub use metadata::MetadataController;
pub use selection::SelectionController;

use crate::{
    commons::HelpActions,
    model::{AddressModel, CoreClient, MessagesModel, MetadataModel, SelectionModel},
};
use crossterm::event::{KeyCode, KeyEvent};
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

    /// The active window
    pub window: Window,

    ///  Whether to display the help
    show_help: bool,

    /// Whether to display the address
    show_address: bool,

    /// Whether to display the metadata
    show_metadata: bool,
}

impl<'a> Controller<'a> {
    /// Instantiate the homepage
    pub fn new(core_client: CoreClient) -> Self {
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

        Self {
            selection,
            messages,
            address,
            metadata,
            window: Window::Selection,
            show_help: true,
            show_address: false,
            show_metadata: false,
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
            Window::Selection => self.selection_on_event(key),
            _ => todo!(),
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
                .map_or(false, |x| x.insert_mode())
    }

    // Handle key events in insert mode
    pub fn on_event_insert_mode(&mut self, key: KeyEvent) {
        match self.window {
            Window::Request => self.messages.request.editor.on_key_insert_mode(key),
            _ => unimplemented!(),
        }
    }

    /// Handle key events on the selection window
    fn selection_on_event(&mut self, key: KeyEvent) {
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
            }
            KeyCode::Enter if !model.is_parent_selected() => {
                model.collapse();
            }
            _ => todo!(),
        }
    }

    /// Returns the help actions of the current window
    pub fn help_hint(&self) -> Option<HelpActions> {
        if !self.show_help {
            return None;
        }
        // self.window.help_hint
        match &self.window {
            Window::Selection => {
                let action = if self.selection.is_parent_selected() {
                    ("Enter", "Select")
                } else {
                    ("Esc", "Collapse")
                };
                return Some(HelpActions::from_items(vec![
                    ("q", "Quit"),
                    ("Tab", "Go"),
                    ("j/↓", "down"),
                    ("k/↑", "up"),
                    action,
                ]));
            }
            _ => None,
        }
        // match &self.window {
        //     Window::Selection => Some(self.selection_controller.help()),
        //     Window::Request => Some(self.messages_controller.help()),
        //     Window::Address => Some(self.address_controller.help()),
        //     Window::Metadata => Some(self.metadata_controller.help()),
        // }
    }

    /// Toggle the help window on or off
    pub fn toggle_help(&mut self) -> bool {
        self.show_help = !self.show_help;
        false
    }

    /// Toggle the address window on or off
    pub fn toggle_address(&mut self) -> bool {
        self.show_address = !self.show_address;
        if self.show_address {
            self.window = Window::Address;
        } else {
            self.window = Window::Request;
        }
        false
    }

    /// Toggle the meetadata window on or off
    pub fn toggle_metadata(&mut self) -> bool {
        self.show_metadata = !self.show_metadata;
        if self.show_metadata {
            self.window = Window::Metadata;
        } else {
            self.window = Window::Request;
        }
        false
    }
}
#[derive(PartialEq, Eq)]
pub enum Window {
    Selection,
    Request,
    Address,
    Metadata,
}

impl Window {
    fn next(&self) -> Self {
        match &self {
            Self::Selection => Self::Request,
            Self::Request => Self::Selection,
            Self::Address => Self::Address,
            Self::Metadata => Self::Metadata,
        }
    }
}
