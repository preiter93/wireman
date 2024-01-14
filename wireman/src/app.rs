use crate::{
    context::{AppContext, Tab},
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
    ctx: AppContext,

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

impl App {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(env: Config) -> Result<App> {
        Ok(App {
            term: Term::new()?,
            ctx: AppContext::new(&env)?,
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
            let root = Root::new(&self.ctx);
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
            KeyCode::Char('q') if !self.ctx.disable_root_events => {
                self.should_quit = true;
            }
            _ => match self.ctx.tab {
                Tab::Selection => {
                    SelectionInput {
                        model: self.ctx.selection.clone(),
                        messages_model: self.ctx.messages.clone(),
                        context: &mut self.ctx,
                    }
                    .handle(event.code, event.modifiers);
                }
                Tab::Messages => MessagesInput {
                    model: self.ctx.messages.clone(),
                    ctx: &mut self.ctx,
                }
                .handle(event),
                Tab::Headers => HeadersInput {
                    model: self.ctx.headers.clone(),
                    context: &mut self.ctx,
                }
                .handle(event),
            },
        }

        // Dispatch the grpc request in a seperate thread.
        if self.ctx.messages.borrow().dispatch {
            let mut messages_model = self.ctx.messages.borrow_mut();
            messages_model.dispatch = false;
            match messages_model.collect_request() {
                Ok(req) => {
                    let handler = tokio::spawn(async move {
                        let resp = do_request(req).await;
                        let _ = sx.send(resp).await;
                    });
                    messages_model.handler = Some(handler);
                }
                Err(err) => {
                    messages_model.response.set_text(&err.string());
                    messages_model.response.set_error(err);
                }
            }
        }
        Ok(())
    }

    fn handle_internal_event(&mut self, result: RequestResult) -> Result<()> {
        result.set(&mut self.ctx.messages.borrow_mut().response.editor);
        self.ctx.messages.borrow_mut().handler.take();
        Ok(())
    }
}
