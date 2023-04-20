mod app;
mod commons;
mod controller;
mod model;
mod theme;
mod view;
mod widgets;
use crate::app::{run_app, App};
use core::init_from_file;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use model::CoreClient;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    let core_client = init_core_client().map_err(|err| {
        // gracefully shutdown if the config cant be parsed
        reset_terminal().unwrap();
        err
    })?;

    let app = App::new(core_client);
    let _ = run_app(&mut terminal, app);

    reset_terminal()?;
    terminal.show_cursor()?;

    Ok(())
}

/// Initiate the core client.
fn init_core_client() -> Result<CoreClient> {
    let cfg = init_from_file("./config.json")?;

    Ok(CoreClient::new(cfg)?)
}

/// Initializes the terminal.
fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    panic_hook();

    Ok(terminal)
}

/// Resets the terminal.
fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

fn panic_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));
}
