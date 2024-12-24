#![allow(clippy::module_name_repetitions)]
use std::env::var;
use std::error::Error as StdError;
use std::fmt::{self};
use std::path::Path;

use logger::Logger;

use crate::config::{HistoryConfig, LoggingConfig};
use crate::install::expand_path;
use crate::{Config, CONFIG_FNAME, DEFAULT_CONFIG_DIR, ENV_CONFIG_DIR};
use theme::Theme;

use crate::error::{Error, Result};
use std::result::Result as StdResult;

/// Initializes the `Config` from environment variables
///
/// # Errors
/// See [`setup`].
pub fn init_from_env() -> Result<(Config, String)> {
    setup(false)
}

/// Runs the setup, allowing for a dry-run mode where no files are created.
///
/// In dry-run mode, additional information is logged to the console.
///
/// # Errors
///
/// This function can return the following errors:
///
/// - `Config Init Errors`: Error initializing the configuration.
/// - `Logger Init Errors`: Error initializing the logger.
/// - `History Init Errors`: Error initializing the history.
pub fn setup(dry_run: bool) -> Result<(Config, String)> {
    let config_dir = match config_dir_checked() {
        Err(err) => {
            if dry_run {
                println!("{:<20} Error: {}", "Config:", err);
            }
            return Err(Error::SetupError(err));
        }
        Ok(config_dir) => config_dir,
    };
    let config_dir_path = Path::new(&config_dir);

    let config_file = match config_file_checked(config_dir_path) {
        Err(err) => {
            if dry_run {
                println!("{:<20} Error: {}", "Config:", err);
            }
            return Err(Error::SetupError(err));
        }
        Ok(config_dir) => {
            if dry_run {
                println!("{:<20} {}", "Config:", config_dir);
            }
            config_dir
        }
    };

    let mut config = match Config::load(&config_file) {
        Err(err) => {
            if dry_run {
                println!("{:<20} Error: {}", "Config:", err);
            }
            return Err(err);
        }
        Ok(config_dir) => config_dir,
    };

    if config.history.disabled {
        if dry_run {
            println!("{:<20} disabled", "History:");
        }
    } else {
        let history_dir = match history_dir_checked(config_dir_path, &config.history) {
            Err(err) => {
                if dry_run {
                    println!("{:<20} Error: {}", "History:", err);
                }
                return Err(Error::SetupError(err));
            }
            Ok(history_dir) => {
                if dry_run {
                    println!("{:<20} {}", "History:", history_dir);
                }
                history_dir
            }
        };
        config.history.directory.clone_from(&history_dir);
        if !dry_run && !Path::new(&history_dir).exists() {
            if let Err(err) = std::fs::create_dir(&history_dir) {
                return Err(Error::SetupError(SetupError::CreateDirectory(Box::new(
                    err,
                ))));
            }
        }
    }

    let logger_file = match logger_dir_checked(config_dir_path, &config.logging) {
        Err(err) => {
            if dry_run {
                println!("{:<20} Error: {}", "Logging:", err);
            }
            return Err(Error::SetupError(err));
        }
        Ok(logger_dir) => {
            config.logging.directory.clone_from(&logger_dir);
            let logger_file = config.logging.file_path_expanded();
            if dry_run {
                println!("{:<20} {}", "Logging:", logger_file);
            }
            logger_file
        }
    };
    if !dry_run {
        if let Err(err) = Logger::init(logger_file, config.logging.level) {
            return Err(Error::SetupError(SetupError::InitializeLogger(Box::new(
                err,
            ))));
        }
    }

    if !dry_run {
        Theme::init(&config.ui);
    }

    Ok((config, config_file))
}

fn config_dir_checked() -> StdResult<String, SetupError> {
    let config_dir = var(ENV_CONFIG_DIR).unwrap_or(DEFAULT_CONFIG_DIR.to_string());
    let config_dir_expanded = expand_path(&config_dir);

    let config_path = Path::new(&config_dir_expanded);
    if config_dir.is_empty() || !config_path.exists() {
        return Err(SetupError::ConfigDirInvalid);
    }

    Ok(config_dir_expanded)
}

fn config_file_checked(config_path: &Path) -> StdResult<String, SetupError> {
    let config_file_path = config_path.join(CONFIG_FNAME);
    let config_file = config_file_path.to_string_lossy();
    if !config_path.exists() {
        return Err(SetupError::ConfigFileNotFound(config_file.to_string()));
    }

    Ok(config_file.to_string())
}

fn history_dir_checked(
    config_path: &Path,
    history: &HistoryConfig,
) -> StdResult<String, SetupError> {
    let mut history_dir_path = history.directory_expanded();
    if history_dir_path.is_empty() {
        let default_history_path = {
            let path = config_path.join("history").clone();
            path.to_string_lossy().to_string()
        };
        history_dir_path = default_history_path.to_string();
    }

    if Path::new(&history_dir_path)
        .parent()
        .is_none_or(|p| !p.exists())
    {
        return Err(SetupError::HistoryPathNotFound(
            history_dir_path.to_string(),
        ));
    }

    Ok(history_dir_path)
}

fn logger_dir_checked(
    config_path: &Path,
    logging: &LoggingConfig,
) -> StdResult<String, SetupError> {
    let mut logger_dir = logging.directory_expanded();
    if logger_dir.is_empty() {
        let default_logger_path = config_path.to_string_lossy().to_string();
        logger_dir = default_logger_path.to_string();
    }

    if !Path::new(&logger_dir).exists() {
        return Err(SetupError::LoggerPathNotFound(logger_dir.to_string()));
    }

    Ok(logger_dir)
}

#[derive(Debug)]
pub enum SetupError {
    ConfigDirEnvNotFound,
    ConfigDirInvalid,
    ConfigFileNotFound(String),
    HistoryPathNotFound(String),
    LoggerPathNotFound(String),
    CreateDirectory(Box<dyn StdError>),
    InitializeLogger(Box<dyn StdError>),
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupError::ConfigDirEnvNotFound => {
                write!(f, "The config dir ${ENV_CONFIG_DIR} was not found.")
            }
            SetupError::ConfigDirInvalid => {
                write!(f, "The config dir ${ENV_CONFIG_DIR} is not a valid path.")
            }
            SetupError::CreateDirectory(err) => {
                write!(f, "Could not create directory in ${ENV_CONFIG_DIR}: {err}.")
            }
            SetupError::ConfigFileNotFound(file) => {
                write!(f, "The config file {file} was not found.")
            }
            SetupError::HistoryPathNotFound(path) => {
                write!(f, "The historys parent path {path} does not exist.")
            }
            SetupError::LoggerPathNotFound(path) => {
                write!(f, "The loggers parent path {path} does not exist.")
            }
            SetupError::InitializeLogger(err) => write!(f, "Failed to initialize logger: {err}."),
        }
        // let general_error_msg = "Check the README for tips on how to set up wireman: \
        // https://github.com/preiter93/wireman";
        // write!(f, "{}\n{}\n", self.error_msg, general_error_msg)
    }
}

impl StdError for SetupError {}
