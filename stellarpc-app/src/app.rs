use crate::{controller::Controller, model::CoreClient, view::render};
use crossterm::event::{self, Event};
use ratatui::{backend::Backend, Terminal};

pub struct App<'a> {
    pub controller: Controller<'a>,
}

impl<'a> App<'a> {
    pub fn new(core_client: CoreClient) -> App<'a> {
        App {
            controller: Controller::new(core_client),
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    let mut quit: bool;
    loop {
        terminal.draw(|f| render(f, &mut app.controller))?;

        if let Event::Key(event) = event::read()? {
            quit = app.controller.on_event(event);
            if quit {
                return Ok(());
            }
        }
    }
}
