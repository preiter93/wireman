//! Configuration module for `WireMan`.
//!
//! This module provides functionality for defining and reading the configuration
//! for `WireMan`. The config file is read from a JSON file to customize `WireMan`.
//!
//! The config contains:
//!
//! - `includes`: A list of include directories for `gRPC`.
//! - `files`: A list of .proto files to include.
//! - `default_address`: The default address of the `gRPC` server.
//! - `history_dir`: The folder path where the history should be kept
pub mod config;
pub mod error;
pub use config::Config;
use std::{env, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// This env is used to read the path for the `WireMan` config.
/// If it is not set, the config is expected in the current
/// directory.
pub const ENV_CONFIG_DIR: &str = "WIREMAN_CONFIG_DIR";

/// The wireman config filename
pub const CONFIG_FNAME: &str = "config.toml";

/// Debug flag
pub const DEBUG: bool = true;

/// Autosaves the history when switching between histories
pub const AUTOSAVE_HISTORY: bool = false;

/// Initializes the config
pub fn init_env() -> Result<Config> {
    fn env_file() -> String {
        if let Ok(current_dir) = std::env::current_dir() {
            let config_path = current_dir.join(CONFIG_FNAME);
            if config_path.exists() && config_path.is_file() {
                return format!("{}/{}", current_dir.to_str().unwrap(), CONFIG_FNAME);
            }
        }
        env::var(ENV_CONFIG_DIR).unwrap_or(CONFIG_FNAME.to_string())
    }
    let cfg_file = env_file();
    let cfg = Config::load(&cfg_file)?;
    Ok(cfg)
}
