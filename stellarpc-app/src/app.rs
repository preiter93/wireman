use std::io::stdout;

use crate::{controller::Controller, model::CoreClient, view::root::Root, AppConfig};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::Backend, Terminal};

pub struct App<'a> {
    pub controller: Controller<'a>,
    context: AppContext,
    should_quit: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppContext {
    pub tab_index: usize,
}

impl<'a> App<'a> {
    pub fn new(core_client: CoreClient, config: AppConfig) -> App<'a> {
        App {
            controller: Controller::new(core_client, config),
            context: AppContext::default(),
            should_quit: false,
        }
    }

    pub fn run<B: Backend>(
        core_client: CoreClient,
        config: AppConfig,
        terminal: &mut Terminal<B>,
    ) -> std::io::Result<()> {
        install_panic_hook();
        let mut app = Self::new(core_client, config);
        while !app.should_quit {
            app.draw(terminal)?;
            app.handle_events()?;
        }
        Term::stop()?;
        Ok(())
    }

    fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(Root::new(&self.context, &self.controller), frame.size())
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(event) = event::read()? {
            let quit = self.controller.on_event(event);
            self.should_quit = quit;
        }
        Ok(())
    }
}

pub fn install_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Term::stop();
        hook(info);
        std::process::exit(1);
    }));
}
pub struct Term {}

impl Term {
    pub fn stop() -> std::io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
