use crate::{context::AppContext, events::InternalStream, term::Term, view::root::Root};
use config::Config;
use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use std::error::Error;
use tokio::select;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Represents the app responsible for managing the terminal, context
/// and control flow.
pub struct App {
    /// The terminal instance.
    pub(crate) term: Term,

    /// The context containing app-specific data.
    pub(crate) ctx: AppContext,

    /// Indicating whether the application should quit or not.
    pub(crate) should_quit: bool,

    /// The crossterm event stream
    pub(crate) crossterm_stream: EventStream,

    /// The internal event stream
    pub(crate) internal_stream: InternalStream,
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
            frame.render_widget(root, frame.area());
        })?;
        Ok(())
    }

    async fn handle_events(&mut self) -> Result<()> {
        select! {
            crossterm_event = self.crossterm_stream.next() => {
                if let Some(Ok(event)) = crossterm_event {
                    match event {
                        Event::Key(event) => self.handle_crossterm_key_event(event),
                        Event::Mouse(event) => self.handle_crossterm_mouse_event(event),
                        _ => (),
                    }
                 }
            },
            internal_event = self.internal_stream.rx.recv() =>{
                if let Some(event) = internal_event {
                     self.handle_internal_event(&event);
                 }
            }
        };
        Ok(())
    }
}
