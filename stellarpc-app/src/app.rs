use std::io::stdout;

use crate::{
    controller::Controller, input::SelectionInput, model::CoreClient, view::root::Root, AppConfig,
};
use crossterm::{
    event::{self, Event, KeyCode},
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
    /// The main tab
    pub tab: Tab,

    /// The sub window
    pub sub: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Tab {
    #[default]
    Selection,
    Messages,
    Headers,
}
impl Tab {
    pub fn next(&self) -> Self {
        match &self {
            Self::Selection => Self::Messages,
            Self::Messages => Self::Headers,
            Self::Headers => Self::Selection,
        }
    }
    pub fn prev(&self) -> Self {
        match &self {
            Self::Selection => Self::Headers,
            Self::Headers => Self::Messages,
            Self::Messages => Self::Selection,
        }
    }
    pub fn index(&self) -> usize {
        match &self {
            Self::Selection => 0,
            Self::Messages => 1,
            Self::Headers => 2,
        }
    }
}

// #[derive(PartialEq, Eq)]
// pub enum Window {
//     Services,
//     Methods,
//     Request,
//     Response,
//     Address,
//     Metadata,
//     History,
// }

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
            match event.code {
                KeyCode::BackTab => {
                    self.context.tab = self.context.tab.prev();
                }
                KeyCode::Tab => {
                    self.context.tab = self.context.tab.next();
                }
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                _ => match self.context.tab {
                    Tab::Selection => match event.code {
                        KeyCode::Enter if self.context.sub == 0 => {
                            self.context.sub = 1;
                            self.controller.selection.borrow_mut().next_method();
                        }
                        KeyCode::Enter if self.context.sub == 1 => {
                            self.context.tab = self.context.tab.next();
                        }
                        KeyCode::Esc if self.context.sub == 1 => {
                            self.context.sub = 0;
                            self.controller.selection.borrow_mut().clear_method();
                        }
                        _ => {
                            SelectionInput {
                                model: self.controller.selection.clone(),
                                messages_model: self.controller.messages.clone(),
                                sub_index: self.context.sub,
                            }
                            .handle(event.code);
                        }
                    },
                    _ => {}
                },
            }
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
