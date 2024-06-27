#![allow(clippy::module_name_repetitions)]
use std::env::var;
use std::error::Error as StdError;
use std::fmt::{self};
use std::path::Path;

use logger::{Logger, LoggerError};

use crate::config::{HistoryConfig, LoggingConfig};
use crate::{Config, CONFIG_FNAME, ENV_CONFIG_DIR};
use theme::Theme;

use crate::error::{Error, Result};
use std::result::Result as StdResult;

/// Initializes the `Config` from environment variables
///
/// # Errors
/// See `setup`
pub fn init_from_env() -> Result<Config> {
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
pub fn setup(dry_run: bool) -> Result<Config> {
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
                return Err(Error::SetupError(SetupError::new(format!(
                    "Failed to create the directory {history_dir}, err: {err}",
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
            return Err(Error::SetupError(SetupError::new(format!(
                "Failed to initialize the logger: {err}"
            ))));
        }
    }

    if !dry_run {
        Theme::init(&config.ui);
    }

    Ok(config)
}

fn config_dir_checked() -> StdResult<String, SetupError> {
    let config_dir = var(ENV_CONFIG_DIR).map_err(|err| {
        SetupError::new(format!(
            "Failed to read the environment variable {ENV_CONFIG_DIR}, err: {err}",
        ))
    })?;

    if config_dir.is_empty() {
        return Err(SetupError::new(
            "The {ENV_CONFIG_DIR} environment variable is empty. \
Please provide a configuration path.",
        ));
    }

    let config_path = Path::new(&config_dir);
    if !config_path.exists() {
        return Err(SetupError::new(format!(
            "The directory specified by the env variable '{ENV_CONFIG_DIR}' \
             does not exist: {config_dir}",
        )));
    }

    Ok(config_dir)
}

fn config_file_checked(config_path: &Path) -> StdResult<String, SetupError> {
    let config_file_path = config_path.join(CONFIG_FNAME);
    let config_file = config_file_path.to_string_lossy();
    if !config_path.exists() {
        return Err(SetupError::new(format!(
            "No config file found: {config_file}"
        )));
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
        .map_or(true, |p| !p.exists())
    {
        return Err(SetupError::new(format!(
            "Non existant parent of history directory {history_dir_path}.",
        )));
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
        return Err(SetupError::new(format!(
            "Non existant path to log file {logger_dir}.",
        )));
    }

    Ok(logger_dir)
}

#[derive(Debug)]
pub struct SetupError {
    error_msg: String,
}

impl SetupError {
    pub fn new<S: Into<String>>(error_msg: S) -> Self {
        Self {
            error_msg: error_msg.into(),
        }
    }
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let general_error_msg = "Check the README for tips on how to set up wireman: \
        https://github.com/preiter93/wireman";
        write!(f, "{}\n{}\n", self.error_msg, general_error_msg)
    }
}

impl From<LoggerError> for SetupError {
    fn from(value: LoggerError) -> Self {
        Self {
            error_msg: value.error_msg,
        }
    }
}

impl StdError for SetupError {
    fn description(&self) -> &str {
        &self.error_msg
    }
}
