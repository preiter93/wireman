use crate::{
    commons::{window_border, HelpActions},
    controller::{MessagesController, SelectionController},
    model::{messages::MessagesModel, SelectionModel},
    view::{draw_request, draw_selection_and_help},
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
    // The controller for the services and methods list
    pub selection_controller: SelectionController,

    // The controller for the request and response messages
    pub messages_controller: MessagesController<'a>,

    /// The active window
    window: Window,

    // Whether to display the help tile
    show_help: bool,
}

impl<'a> HomePage<'a> {
    /// Instantiate the homepage
    pub fn new(core_client: CoreClient) -> HomePage<'a> {
        // The core client communicates with the prototui core pkg
        let core_client_rc = Rc::new(RefCell::new(core_client));

        // Construct the selection controller
        let list_model = SelectionModel::new(Rc::clone(&core_client_rc));
        let list_controller = SelectionController::new(list_model);

        // Construct the request controller
        let messages_model = MessagesModel::new(core_client_rc);
        let messages_controller = MessagesController::new(messages_model);

        Self {
            window: Window::Selection,
            messages_controller,
            selection_controller: list_controller,
            show_help: true,
        }
    }
    /// the key bindings on this page
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        // The currently selected proto method. This state is lifted up
        // from the list selection and passed to the request window which
        // loads the proto message in the correct format.
        let mut load_method = None;
        if !self.messages_controller.in_insert_mode() {
            match key.code {
                KeyCode::Char('q') => return true,
                KeyCode::Tab => self.window = self.window.next(),
                KeyCode::Char('H') => self.toggle_help(),
                _ => {}
            }
        }
        match self.window {
            Window::Selection => {
                load_method = self.selection_controller.on_key(key);
            }
            Window::Request => self.messages_controller.on_key(key),
        }
        // Load the currently selected method. This should only
        // be called if the method actually has changed
        if let Some(method) = &load_method {
            self.messages_controller.load_method(method);
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
            help_actions,
        );

        draw_request(
            f,
            chunks[1],
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
        }
    }

    /// Toggle the help window on or off
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help
    }

    /// Processes data from other pages
    pub fn process_route_data(&mut self, data: String) {
        self.messages_controller.set_metadata(data);
    }
}

#[derive(PartialEq, Eq)]
enum Window {
    Selection,
    Request,
}

impl Window {
    pub fn next(&self) -> Self {
        match &self {
            Self::Selection => Self::Request,
            Self::Request => Self::Selection,
        }
    }
}
