use crate::commons::{window_border, HelpActions};
use crate::controller::{MessagesController, SelectionController};
use crate::model::messages::MessagesModel;
use crate::model::{CoreClient, SelectionModel};
use crate::view::{draw_request, draw_selection_and_help};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};
use std::{cell::RefCell, io, rc::Rc};

/// This struct holds the current state of the app.
pub struct App<'a> {
    // The controller for the services and methods list
    pub selection_controller: SelectionController,

    // The controller for the request and response messages
    pub messages_controller: MessagesController<'a>,

    /// The currently active window
    active_window: ActiveWindow,

    // Whether to display the help tile
    show_help: bool,
}

impl<'a> App<'a> {
    pub fn new(core_client: CoreClient) -> App<'a> {
        // The core client communicates with the prototui core pkg
        let core_client_rc = Rc::new(RefCell::new(core_client));

        // Construct the selection controller
        let list_model = SelectionModel::new(Rc::clone(&core_client_rc));
        let list_controller = SelectionController::new(list_model);

        // Construct the request controller
        let messages_model = MessagesModel::new(core_client_rc);
        let messages_controller = MessagesController::new(messages_model);

        App {
            active_window: ActiveWindow::Selection,
            messages_controller,
            selection_controller: list_controller,
            show_help: true,
        }
    }

    /// Return the help actions of the current window
    pub fn help(&self) -> Option<HelpActions> {
        if !self.show_help {
            return None;
        }
        match &self.active_window {
            ActiveWindow::Selection => Some(self.selection_controller.help()),
            ActiveWindow::Request => Some(self.messages_controller.help()),
        }
    }

    /// Toggle the help window on or off
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    // The currently selected proto method. This state is lifted up
    // from the list selection and passed to the request window which
    // loads the proto message in the correct format.
    let mut load_method = None;
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if !app.messages_controller.request_insert_mode() {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => app.active_window = app.active_window.next(),
                        KeyCode::Char('H') => app.toggle_help(),
                        _ => {}
                    }
                }
                match app.active_window {
                    ActiveWindow::Selection => {
                        load_method = app.selection_controller.on_key(key);
                    }
                    ActiveWindow::Request => app.messages_controller.on_key(key),
                }
                // Load the currently selected method. This should only
                // be called if the method actually has changed
                if let Some(method) = &load_method {
                    app.messages_controller.load_method(method);
                    // Once we loaded the method we set it to None to
                    // avoid to load it multiple times
                    load_method = None;
                }
            }
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let help_actions = app.help();

    draw_selection_and_help(
        f,
        chunks[0],
        &mut app.selection_controller,
        window_border("Selection", app.active_window == ActiveWindow::Selection),
        help_actions,
    );

    draw_request(
        f,
        chunks[1],
        &mut app.messages_controller,
        window_border("Request", app.active_window == ActiveWindow::Request),
    );
}

#[derive(PartialEq, Eq)]
enum ActiveWindow {
    Selection,
    Request,
}

impl ActiveWindow {
    pub fn next(&self) -> Self {
        match &self {
            Self::Selection => Self::Request,
            Self::Request => Self::Selection,
        }
    }
}
