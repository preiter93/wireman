use std::io::stdout;

use crate::{
    controller::Controller,
    input::{HeadersInput, MessagesInput, SelectionInput},
    model::CoreClient,
    view::root::Root,
    AppConfig,
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

    /// Disable root key events. if an editor
    /// goes into insert mode, global key events
    /// such as quit and tab should be disabled
    pub disable_root_events: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Tab {
    #[default]
    Selection,
    Messages,
    Headers,
}
impl Tab {
    pub fn next(self) -> Self {
        match &self {
            Self::Selection => Self::Messages,
            Self::Messages => Self::Headers,
            Self::Headers => Self::Selection,
        }
    }
    pub fn prev(self) -> Self {
        match &self {
            Self::Selection => Self::Headers,
            Self::Headers => Self::Messages,
            Self::Messages => Self::Selection,
        }
    }
    pub fn index(self) -> usize {
        match &self {
            Self::Selection => 0,
            Self::Messages => 1,
            Self::Headers => 2,
        }
    }
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
            frame.render_widget(Root::new(&self.context, &self.controller), frame.size());
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::BackTab if !self.context.disable_root_events => {
                    self.context.tab = self.context.tab.prev();
                    self.context.sub = 0;
                }
                KeyCode::Tab if !self.context.disable_root_events => {
                    self.context.tab = self.context.tab.next();
                    self.context.sub = 0;
                }
                KeyCode::Char('q') if !self.context.disable_root_events => {
                    self.should_quit = true;
                }
                _ => match self.context.tab {
                    Tab::Selection => {
                        SelectionInput {
                            model: self.controller.selection.clone(),
                            messages_model: self.controller.messages.clone(),
                            context: &mut self.context,
                        }
                        .handle(event.code);
                    }
                    Tab::Messages => MessagesInput {
                        model: self.controller.messages.clone(),
                        history_model: self.controller.history.clone(),
                        context: &mut self.context,
                    }
                    .handle(event),
                    Tab::Headers => HeadersInput {
                        model: self.controller.headers.clone(),
                        context: &mut self.context,
                    }
                    .handle(event),
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
