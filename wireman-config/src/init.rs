use std::env::var;
use std::error::Error as StdError;
use std::fmt::{self};
use std::path::Path;

use logger::{Logger, LoggerError};

use crate::{Config, CONFIG_FNAME, ENV_CONFIG_DIR};

use crate::error::{Error, Result};

pub fn init_from_env() -> Result<Config> {
    let config_dir = var(ENV_CONFIG_DIR).map_err(|err| {
        Error::InitializeError(InitError::new(format!(
            "Failed to read the environment variable {ENV_CONFIG_DIR}, err: {}",
            err.to_string()
        )))
    })?;

    if config_dir.is_empty() {
        return Ok(Config::default());
    }

    let config_path = Path::new(&config_dir);
    if !config_path.exists() {
        return Err(Error::InitializeError(InitError::new(format!(
            "The directory specified by the env variable '{ENV_CONFIG_DIR}' \
             does not exist: {config_dir}",
        ))));
    }

    let config_file_path = config_path.join(CONFIG_FNAME);
    let config_file = config_file_path.to_string_lossy();
    if !config_path.exists() {
        return Err(Error::InitializeError(InitError::new(format!(
            "No config file found: {config_file}"
        ))));
    }

    let cfg = Config::load(&config_file)?;

    let mut use_default_logger_path = false;
    let mut logger_file_path = cfg.logging.file_path.clone();
    if logger_file_path.is_empty() {
        use_default_logger_path = true;
    }
    if Path::new(&logger_file_path)
        .parent()
        .map_or(true, |p| !p.exists())
    {
        use_default_logger_path = true;
    }
    if use_default_logger_path {
        let default_logger_path = config_path.join("wireman.log");
        logger_file_path = default_logger_path.to_string_lossy().to_string();
    }
    if let Err(err) = Logger::init(logger_file_path, cfg.logging.level) {
        return Err(Error::InitializeError(InitError::new(format!(
            "Failed to initialize the logger: {err}"
        ))));
    }
    if use_default_logger_path {
        if !cfg.logging.file_path.is_empty() {
            Logger::debug(format!(
                "Non existant path to log file {}. Fallback to default.",
                cfg.logging.file_path
            ))
        }
    }

    Ok(cfg)
}

#[derive(Debug)]
pub struct InitError {
    error_msg: String,
}

impl InitError {
    fn new<S: Into<String>>(error_msg: S) -> Self {
        Self {
            error_msg: error_msg.into(),
        }
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let general_error_msg = "Check the README for tips on how to set up wireman: \
        https://github.com/preiter93/wireman";
        write!(f, "{}\n{}\n", self.error_msg, general_error_msg)
    }
}

impl From<LoggerError> for InitError {
    fn from(value: LoggerError) -> Self {
        Self {
            error_msg: value.error_msg,
        }
    }
}

impl StdError for InitError {
    fn description(&self) -> &str {
        &self.error_msg
    }
}
