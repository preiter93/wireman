use crate::{controller::Controller, model::CoreClient, pages::HomePage, ui::render};
use crossterm::event::{self, Event};
use ratatui::{backend::Backend, Terminal};

/// This struct holds the current state of the app.
pub struct App<'a> {
    /// The home page
    pub home: HomePage<'a>,

    /// The apps controller. It translates key events and calls
    /// the respective models.
    pub controller: Controller<'a>,

    /// The active page
    page: Page,
}

impl<'a> App<'a> {
    pub fn new(core_client: CoreClient) -> App<'a> {
        App {
            home: HomePage::new(core_client.clone()),
            controller: Controller::new(core_client),
            page: Page::Home,
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    let mut quit: bool;
    loop {
        terminal.draw(|f| render(f, &mut app.controller))?;

        if let Event::Key(event) = event::read()? {
            quit = app.controller.on_event(event);
            // match app.page {
            //     Page::Home => {
            //         quit = app.home.on_key(event);
            //     }
            // }
            if quit {
                return Ok(());
            }
        }
    }
}

// /// render the app
// pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
//     // match app.page {
//     //     Page::Home => app.home.ui(f),
//     // }
//     ui()
// }

#[derive(Clone, PartialEq, Eq)]
enum Page {
    Home,
}
