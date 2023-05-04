use crate::{model::CoreClient, pages::HomePage};
use crossterm::event::{self, Event};
use ratatui::{backend::Backend, Frame, Terminal};

/// This struct holds the current state of the app.
pub struct App<'a> {
    /// The home page
    pub home: HomePage<'a>,

    /// The active page
    page: Page,
}

impl<'a> App<'a> {
    pub fn new(core_client: CoreClient) -> App<'a> {
        App {
            home: HomePage::new(core_client),
            page: Page::Home,
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    let mut quit: bool;
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.page {
                Page::Home => {
                    quit = app.home.on_key(key);
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
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Page {
    Home,
}
