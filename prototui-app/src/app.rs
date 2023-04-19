use crate::commons::window_border;
use crate::commons::HelpActions;
use crate::controller::EditorController;
use crate::controller::ListWithChildrenController;
use crate::model::analyzer::ProtoAnalyzer;
use crate::model::EditorModel;
use crate::model::ListWithChildrenModel;
use crate::view::editor::draw_text_editor;
use crate::view::list_with_children::draw_list_and_help;
use core::ProtoDescriptor;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
};
use ratatui::{Frame, Terminal};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

/// This struct holds the current state of the app.
pub struct App<'a> {
    /// The currently active window
    active_window: ActiveWindow,
    // The controller for the services and methods list
    pub list_controller: ListWithChildrenController,
    // The controller for the request editor
    pub editor_controller: EditorController<'a>,
    // Whether to display the help tile
    show_help: bool,
}

impl<'a> App<'a> {
    pub fn new(desc: ProtoDescriptor) -> App<'a> {
        let analyzer = ProtoAnalyzer::new(desc);
        let analyzer_rc = Rc::new(RefCell::new(analyzer));
        let list_model = ListWithChildrenModel::new(Rc::clone(&analyzer_rc));
        let list_controller = ListWithChildrenController::new(list_model);
        let editor_model = EditorModel::new(analyzer_rc);
        let editor_controller = EditorController::new(editor_model);
        App {
            active_window: ActiveWindow::List,
            editor_controller,
            list_controller,
            show_help: true,
        }
    }

    /// Whether the editor is currently in insert mode
    /// In insert mode we dont want to quit the app with 'q' or switch windows with tab
    fn is_insert_mode(&self) -> bool {
        self.editor_controller.model.is_insert_mode()
    }

    /// Return the help actions of the current window
    pub fn help(&self) -> Option<HelpActions> {
        if !self.show_help {
            return None;
        }
        match &self.active_window {
            ActiveWindow::List => Some(self.list_controller.help()),
            ActiveWindow::Editor => Some(self.editor_controller.help()),
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
                if !app.is_insert_mode() {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => app.active_window = app.active_window.next(),
                        KeyCode::Char('H') => app.toggle_help(),
                        _ => {}
                    }
                }
                match app.active_window {
                    ActiveWindow::List => {
                        load_method = app.list_controller.on_key(key);
                    }
                    ActiveWindow::Editor => app.editor_controller.on_key(key),
                }
                // Load the currently selected method. This should only
                // be called if the method actually has changed
                if let Some(method) = &load_method {
                    app.editor_controller.model.load_method(method);
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

    draw_list_and_help(
        f,
        chunks[0],
        &mut app.list_controller.model,
        window_border("Selection", app.active_window == ActiveWindow::List),
        help_actions,
    );

    draw_text_editor(
        f,
        chunks[1],
        &mut app.editor_controller.model,
        window_border("Request", app.active_window == ActiveWindow::Editor),
    );
}

#[derive(PartialEq, Eq)]
enum ActiveWindow {
    List,
    Editor,
}

impl ActiveWindow {
    pub fn next(&self) -> Self {
        match &self {
            Self::List => Self::Editor,
            Self::Editor => Self::List,
        }
    }
}
