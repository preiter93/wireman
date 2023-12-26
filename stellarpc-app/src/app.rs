use crate::{
    controller::Controller,
    input::{HeadersInput, MessagesInput, SelectionInput},
    term::Term,
    view::root::Root,
};
use config::Config;
use crossterm::event::{self, Event, KeyCode};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Represents the app responsible for managing the terminal, context
/// and control flow.
pub struct App {
    /// The terminal instance.
    term: Term,

    /// The context containing app-specific data.
    context: AppContext,

    /// The controller managing the app's flow and inputs.
    controller: Controller,

    /// Indicating whether the application should quit or not.
    should_quit: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppContext {
    /// The main tab.
    pub tab: Tab,

    /// The index of the sub window.
    pub sub: usize,

    /// Disable root key events. Disables keys such as
    /// quit when an editor is in insert mode.
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

impl App {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(env: Config) -> Result<App> {
        Ok(App {
            term: Term::new()?,
            controller: Controller::new(&env)?,
            context: AppContext::default(),
            should_quit: false,
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn run(env: Config) -> Result<()> {
        let mut app = Self::new(env)?;
        while !app.should_quit {
            app.draw()?;
            app.handle_events()?;
        }
        Term::stop()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.term.draw(|frame| {
            let root = Root::new(&self.context, &self.controller);
            frame.render_widget(root, frame.size());
        })?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
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
            // Draw and handle events again in certain scenarios. This is
            // to avoid having to handle two event channels, the crossterm
            // key events and internal app events, which would require the
            // introduction of async code. Currently, app events are used
            // to indicate that a grpc request is being processed, with a
            // "Processing..." text being displayed in the response editor
            // in the first frame after which the grpc request is made.
            if self.controller.messages.borrow().is_processing {
                self.draw()?;
                self.controller.messages.borrow_mut().do_request();
                loop {
                    if self.controller.event_recv.recv().is_ok() {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
