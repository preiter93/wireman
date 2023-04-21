use crate::{
    model::CoreClient,
    pages::{config::ConfigPage, home::HomePage},
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Frame, Terminal};

/// This struct holds the current state of the app.
pub struct App<'a> {
    /// The home page
    pub home: HomePage<'a>,

    /// The config page displaying metadata and server address
    pub config: ConfigPage<'a>,

    /// The active page
    page: Page,
}

impl<'a> App<'a> {
    pub fn new(core_client: CoreClient) -> App<'a> {
        App {
            home: HomePage::new(core_client),
            config: ConfigPage::new(),
            page: Page::Home,
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    let mut quit = false;
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.page {
                Page::Home => {
                    if key.code == KeyCode::Char('S') {
                        // Switch page
                        app.page = Page::Settings;
                    } else {
                        // Use keybindings of page
                        quit = app.home.on_key(key);
                    }
                }
                Page::Settings => {
                    if key.code == KeyCode::Char('S') {
                        // Switch page
                        app.page = Page::Home;
                        // Pass the config data to the homepage
                        app.home.process_global(app.config.metadata());
                    } else {
                        // Use keybindings of page
                        quit = app.config.on_key(key);
                    }
                }
            }
            if quit {
                return Ok(());
            }
        }
    }
}

/// render the app
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.page {
        Page::Home => app.home.ui(f),
        Page::Settings => app.config.ui(f),
    }
}
#[derive(Clone, PartialEq, Eq)]
enum Page {
    Home,
    Settings,
}
