use crate::{
    controller::Controller,
    input::{HeadersInput, MessagesInput, SelectionInput},
    model::messages::{do_request, RequestResult},
    term::Term,
    view::root::Root,
};
use config::Config;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent};
use futures::StreamExt;
use std::error::Error;
use tokio::{
    select,
    sync::mpsc::{self, Receiver, Sender},
};

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

    /// The crossterm event stream
    crossterm_stream: EventStream,

    /// The internal event stream
    internal_stream: InternalStream,
}

type InternalStreamData = RequestResult;
struct InternalStream {
    sx: Sender<InternalStreamData>,
    rx: Receiver<InternalStreamData>,
}

impl InternalStream {
    fn new() -> Self {
        let (sx, rx) = mpsc::channel::<RequestResult>(10);
        Self { sx, rx }
    }
}

#[derive(Debug, Default)]
pub struct AppContext {
    /// The main tab.
    pub tab: Tab,

    /// The index of the sub window.
    pub sub: usize,

    /// Disable root key events. Disables keys such as
    /// quit when an editor is in insert mode.
    pub disable_root_events: bool,
}

impl AppContext {
    pub fn new() -> Self {
        Self::default()
    }
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
            context: AppContext::new(),
            should_quit: false,
            crossterm_stream: EventStream::new(),
            internal_stream: InternalStream::new(),
        })
    }

    #[allow(clippy::needless_pass_by_value)]
    pub async fn run(env: Config) -> Result<()> {
        let mut app = Self::new(env)?;
        while !app.should_quit {
            app.draw()?;
            app.handle_events().await?;
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

    async fn handle_events(&mut self) -> Result<()> {
        select! {
            crossterm_event = self.crossterm_stream.next() => {
                match crossterm_event {
                    Some(Ok(Event::Key(event))) => {
                        self.handle_crossterm_event(event).await?;
                    }
                    _ => {},
                }
            },
            internal_event = self.internal_stream.rx.recv() =>{
                match internal_event {
                    Some(event) => {
                        self.handle_internal_event(event)?;
                    }
                    _ => {},
                }
            }
        };
        Ok(())
    }

    async fn handle_crossterm_event(&mut self, event: KeyEvent) -> Result<()> {
        let sx = self.internal_stream.sx.clone();
        match event.code {
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

        // Dispatch the grpc request in a seperate thread.
        if self.controller.messages.borrow().dispatch {
            self.controller.messages.borrow_mut().dispatch = false;
            let req = self.controller.messages.borrow_mut().collect_request();
            if let Ok(req) = req {
                tokio::spawn(async move {
                    let resp = do_request(req).await;
                    let _ = sx.send(resp).await;
                });
            }
        }
        Ok(())
    }

    fn handle_internal_event(&mut self, result: RequestResult) -> Result<()> {
        result.set(&mut self.controller.messages.borrow_mut().response.editor);
        Ok(())
    }
}
