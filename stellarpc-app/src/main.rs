#![allow(clippy::cast_possible_truncation)]
mod app;
mod commons;
mod controller;
mod model;
mod theme;
mod view;
mod widgets;
use crate::app::{run_app, App};
use commons::debug::log_to_file;
use core::{init_from_file, Config};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use model::CoreClient;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, error::Error, io, path::PathBuf, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    let cfg = init_config()?;
    let history = init_history(&cfg)?;
    let core_client = init_core_client(cfg)?;

    let config = ConfigData { history };

    let app = App::new(core_client, config);
    run_app(&mut terminal, app).unwrap();

    reset_terminal()?;
    terminal.show_cursor()?;

    Ok(())
}

pub struct ConfigData {
    pub history: PathBuf,
}

/// Get config
fn init_config() -> Result<Config> {
    let cfg_file = get_env();
    log_to_file(cfg_file.clone());
    Ok(init_from_file(&cfg_file).map_err(|err| {
        reset_terminal().unwrap();
        err
    })?)
}

/// Initiate the core client.
fn init_core_client(cfg: Config) -> Result<CoreClient> {
    CoreClient::new(cfg).map_err(|err| {
        reset_terminal().unwrap();
        err
    })
}

/// Instanitate the history path
fn init_history(cfg: &Config) -> Result<PathBuf> {
    let path_str = cfg.history.clone().unwrap_or("./history".to_string());
    Ok(PathBuf::from_str(&path_str).map_err(|err| {
        reset_terminal().unwrap();
        err
    })?)
}

fn get_env() -> String {
    let args: Vec<String> = env::args().collect();
    log_to_file(args.clone());
    if let Some(config) = args.get(1) {
        return config.to_string();
    }
    "config.json".to_string()
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
