#![allow(clippy::module_name_repetitions)]
use crate::{
    commons::{window_border, HelpActions},
    controller::{AddressController, MessagesController, MetadataController, SelectionController},
    model::{AddressModel, MessagesModel, MetadataModel, SelectionModel},
    view::{draw_address, draw_metadata, draw_request, draw_selection_and_help},
    CoreClient,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::{cell::RefCell, rc::Rc};

pub struct HomePage<'a> {
    //. The controller for the services and methods list
    pub selection_controller: SelectionController,

    /// The controller for the request and response messages
    pub messages_controller: MessagesController<'a>,

    /// The controller for the address field
    pub address_controller: AddressController<'a>,

    /// The controller for the address field
    pub metadata_controller: MetadataController<'a>,

    /// The active window
    window: Window,

    // Whether to display the help tile
    show_help: bool,

    // Whether to display the address tile
    show_address: bool,

    // Whether to display the metadata tile
    show_metadata: bool,
}

impl<'a> HomePage<'a> {
    /// Instantiate the homepage
    pub fn new(core_client: CoreClient) -> HomePage<'a> {
        // The core client is shared among controllers
        let core_client_rc = Rc::new(RefCell::new(core_client));

        // Construct the selection controller
        let list_model = SelectionModel::new(Rc::clone(&core_client_rc));
        let list_controller = SelectionController::new(list_model);

        // Construct the address controller
        let default_address = core_client_rc.borrow().get_default_address();
        let address_model = Rc::new(RefCell::new(AddressModel::new(&default_address)));
        let address_controller = AddressController::new(Rc::clone(&address_model));

        // Construct the metadata controller
        let metadata_model = Rc::new(RefCell::new(MetadataModel::new()));
        let metadata_controller = MetadataController::new(Rc::clone(&metadata_model));

        // Construct the request controller
        let messages_model = MessagesModel::new(
            core_client_rc,
            Rc::clone(&address_model),
            Rc::clone(&metadata_model),
        );
        let messages_controller = MessagesController::new(messages_model);

        Self {
            window: Window::Selection,
            messages_controller,
            address_controller,
            metadata_controller,
            selection_controller: list_controller,
            show_help: true,
            show_address: false,
            show_metadata: false,
        }
    }
    /// the key bindings on this page
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        // The currently selected proto method. This state is lifted up
        // from the list selection and passed to the request window which
        // loads the proto message in the correct format.
        let mut load_method = None;
        let mut clear_method = false;
        if !self.messages_controller.in_insert_mode()
            && !self.address_controller.in_insert_mode()
            && !self.metadata_controller.in_insert_mode()
        {
            match key.code {
                KeyCode::Char('q') => return true,
                KeyCode::Tab => self.window = self.window.next(),
                KeyCode::Char('H') => self.toggle_help(),
                KeyCode::Char('A') => self.toggle_address(),
                KeyCode::Char('M') => self.toggle_metadata(),
                _ => {}
            }
        }
        match self.window {
            Window::Selection => {
                (load_method, clear_method) = self.selection_controller.on_key(key);
            }
            Window::Request => self.messages_controller.on_key(key),
            Window::Address => {
                if !self.address_controller.in_insert_mode() && key.code == KeyCode::Esc {
                    self.toggle_address();
                } else {
                    self.address_controller.on_key(key);
                }
            }
            Window::Metadata => {
                if !self.metadata_controller.in_insert_mode() && key.code == KeyCode::Esc {
                    self.toggle_metadata();
                } else {
                    self.metadata_controller.on_key(key);
                }
            }
        }
        // Load the currently selected method. This should only
        // be called if the method actually has changed
        if let Some(method) = &load_method {
            self.messages_controller.load_method(method);
        }
        if clear_method {
            self.messages_controller.clear_method();
        }
        false
    }

    /// render the widgets of this page
    pub fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        let help_actions = self.help();

        draw_selection_and_help(
            f,
            chunks[0],
            &mut self.selection_controller,
            window_border("Selection", self.window == Window::Selection),
            &help_actions,
        );

        let address_length = if self.show_address { 3 } else { 0 };
        let metadata_length = if self.show_metadata { 5 } else { 0 };
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(address_length),
                    Constraint::Length(metadata_length),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        draw_address(
            f,
            chunks[0],
            &mut self.address_controller,
            window_border("Address", self.window == Window::Address),
        );

        draw_metadata(
            f,
            chunks[1],
            &mut self.metadata_controller,
            window_border("Metadata", self.window == Window::Metadata),
        );

        draw_request(
            f,
            chunks[2],
            &mut self.messages_controller,
            window_border("Request", self.window == Window::Request),
        );
    }

    /// Return the help actions of the current window
    pub fn help(&self) -> Option<HelpActions> {
        if !self.show_help {
            return None;
        }
        match &self.window {
            Window::Selection => Some(self.selection_controller.help()),
            Window::Request => Some(self.messages_controller.help()),
            Window::Address => Some(self.address_controller.help()),
            Window::Metadata => Some(self.metadata_controller.help()),
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
        }
    }

    /// Toggle the meetadata window on or off
    pub fn toggle_metadata(&mut self) {
        self.show_metadata = !self.show_metadata;
        if self.show_metadata {
            self.window = Window::Metadata;
        } else {
            self.window = Window::Request;
        }
    }
}

#[derive(PartialEq, Eq)]
enum Window {
    Selection,
    Request,
    Address,
    Metadata,
}

impl Window {
    pub fn next(&self) -> Self {
        match &self {
            Self::Selection => Self::Request,
            Self::Request => Self::Selection,
            Self::Address => Self::Address,
            Self::Metadata => Self::Metadata,
        }
    }
}
