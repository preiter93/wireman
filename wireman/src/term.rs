use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io::{stdout, Stdout};
use std::ops::{Deref, DerefMut};

use ratatui::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        crossterm::execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        // Shutdown gracefully
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            let _ = Self::stop();
            original_hook(panic);
        }));

        Ok(Self { terminal })
    }

    pub fn stop() -> Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}

impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        // self.show_cursor().unwrap();
        let _ = Term::stop();
    }
}
