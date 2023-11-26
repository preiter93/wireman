#![allow(clippy::cast_possible_truncation)]
#![allow(dead_code)]
mod app;
mod commons;
mod controller;
mod input;
mod model;
mod theme;
mod view;
mod widgets;
use crate::app::App;
use config::Config;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use model::CoreClient;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, error::Error, io, path::PathBuf, str::FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// This env is used to read the path to the stellarpc config.
/// If it is not set, the config is expected in the current
/// directory.
const ENV_CONFIG: &str = "STELLARPC_CONFIG";
/// Debug flag
const DEBUG: bool = true;

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    let cfg = init_config()?;
    let history = init_history(&cfg)?;
    let core_client = init_core_client(cfg)?;
    let config = AppConfig { history };

    App::run(core_client, config, &mut terminal).unwrap();

    reset_terminal()?;
    terminal.show_cursor()?;

    Ok(())
}

pub struct AppConfig {
    pub history: PathBuf,
}

/// Get config
fn init_config() -> Result<Config> {
    let cfg_file = get_env();
    let cfg = Config::load(&cfg_file).map_err(|err| {
        reset_terminal().unwrap();
        err
    })?;
    Ok(cfg)
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
    let history = cfg.history();
    let path_str = if history.is_empty() {
        "./history"
    } else {
        &history
    };
    Ok(PathBuf::from_str(path_str).map_err(|err| {
        reset_terminal().unwrap();
        err
    })?)
}

fn get_env() -> String {
    let args: Vec<String> = env::args().collect();
    if let Some(config) = args.get(1) {
        return config.to_string();
    }
    env::var(ENV_CONFIG).unwrap_or("config.json".to_string())
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
