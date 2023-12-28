#![allow(clippy::cast_possible_truncation, clippy::module_name_repetitions)]
#![allow(dead_code)]
mod app;
mod commons;
mod controller;
mod input;
mod model;
mod term;
mod theme;
mod view;
mod widgets;
use app::App;
use config::Config;
use std::{env, error::Error};
use term::Term;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// This env is used to read the path for the `WireMan` config.
/// If it is not set, the config is expected in the current
/// directory.
const ENV_CONFIG: &str = "WIREMAN_CONFIG";

/// Debug flag
const DEBUG: bool = true;

/// Autosaves the history when switching between histories
const AUTOSAVE_HISTORY: bool = false;

fn main() -> Result<()> {
    App::run(init_env()?)?;

    Ok(())
}

fn init_env() -> Result<Config> {
    fn env_file() -> String {
        if let Ok(current_dir) = std::env::current_dir() {
            let config_path = current_dir.join("config.json");
            if config_path.exists() && config_path.is_file() {
                return format!("{}/config.json", current_dir.to_str().unwrap());
            }
        }
        env::var(ENV_CONFIG).unwrap_or("config.json".to_string())
    }
    let cfg_file = env_file();
    let cfg = Config::load(&cfg_file).map_err(|err| {
        Term::stop().unwrap();
        err
    })?;
    Ok(cfg)
}
