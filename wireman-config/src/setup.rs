use std::env::var;
use std::error::Error as StdError;
use std::fmt::{self};
use std::path::Path;

use logger::Logger;

use crate::cli::Args;
use crate::config::{HistoryConfig, LoggingConfig};
use crate::install::{expand_file, expand_path, make_absolute_path};
use crate::{Config, CONFIG_FNAME, DEFAULT_CONFIG_DIR, ENV_CONFIG_DIR};
use theme::Theme;

use crate::error::{Error, Result};
use std::result::Result as StdResult;

/// Initializes the `Config` from environment variables
///
/// # Errors
/// See [`setup`].
pub fn init_from_env(args: &Args) -> Result<(Config, Option<String>)> {
    setup(false, args)
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
#[allow(clippy::too_many_lines)]
pub fn setup(dry_run: bool, args: &Args) -> Result<(Config, Option<String>)> {
    let (config_dir_str, config_file) = if let Some(config_file) = &args.config {
        let config_file_abs = expand_file(&make_absolute_path(config_file));
        let config_dir_str = get_parent_dir(&config_file_abs);
        (config_dir_str, config_file.to_string())
    } else {
        let config_dir_str = get_config_dir(dry_run)?;
        let config_dir = Path::new(&config_dir_str);
        let config_file = get_config_file(config_dir, dry_run)?;
        (config_dir_str, config_file)
    };
    let config_dir = Path::new(&config_dir_str);

    let mut config = match load_config(&config_file, dry_run) {
        Ok(config) => config,
        Err(err) => {
            if args.local_protos {
                Config::default()
            } else {
                return Err(err);
            }
        }
    };
    if args.local_protos {
        update_config_with_local_protos(&mut config)?;
    };

    init_history(&mut config, config_dir, dry_run)?;

    init_logger(&mut config, config_dir, dry_run)?;

    if !dry_run {
        Theme::init(&config.ui);
    }

    Ok((config, Some(config_file)))
}

fn get_config_dir(dry_run: bool) -> Result<String> {
    match config_dir_checked() {
        Err(err) => {
            if dry_run {
                println!("{:<20} Error: {}", "Config:", err);
            }
            Err(Error::SetupError(err))
        }
        Ok(config_dir) => Ok(config_dir),
    }
}

fn get_config_file(config_dir: &Path, dry_run: bool) -> Result<String> {
    match config_file_checked(config_dir) {
        Err(err) => {
            if dry_run {
                println!("{:<20} Error: {}", "Config:", err);
            }
            Err(Error::SetupError(err))
        }
        Ok(config_file) => {
            if dry_run {
                println!("{:<20} {}", "Config:", config_file);
            }
            Ok(config_file)
        }
    }
}

fn get_parent_dir(config_file: &str) -> String {
    let path = Path::new(&config_file);
    path.parent().map_or(
        std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        |dir| dir.to_string_lossy().to_string(),
    )
}

fn load_config(config_file: &str, dry_run: bool) -> Result<Config> {
    match Config::load(config_file) {
        Ok(config) => Ok(config),
        Err(err) => {
            if dry_run {
                println!("{:<20} Error: {}", "Config:", err);
            }
            Err(err)
        }
    }
}

fn update_config_with_local_protos(config: &mut Config) -> Result<()> {
    let (current_dir, protos) =
        list_local_protos().map_err(|err| Error::SetupError(SetupError::ListLocalProtos(err)))?;

    config.files = protos;
    config.includes = vec![current_dir];

    Ok(())
}

fn init_history(config: &mut Config, config_dir: &Path, dry_run: bool) -> Result<()> {
    if config.history.disabled {
        if dry_run {
            println!("{:<20} disabled", "History:");
        }
        return Ok(());
    }

    let history_dir = match history_dir_checked(config_dir, &config.history) {
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
    if dry_run {
        return Ok(());
    }

    if !Path::new(&history_dir).exists() {
        if let Err(err) = std::fs::create_dir(&history_dir) {
            return Err(Error::SetupError(SetupError::CreateDirectory(Box::new(
                err,
            ))));
        }
    }
    Ok(())
}

fn init_logger(config: &mut Config, config_dir: &Path, dry_run: bool) -> Result<()> {
    let logger_file = match logger_dir_checked(config_dir, &config.logging) {
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

    if dry_run {
        return Ok(());
    }

    if let Err(err) = Logger::init(logger_file, config.logging.level) {
        return Err(Error::SetupError(SetupError::InitializeLogger(Box::new(
            err,
        ))));
    }

    Ok(())
}

fn config_dir_checked() -> StdResult<String, SetupError> {
    let config_dir = var(ENV_CONFIG_DIR).unwrap_or(DEFAULT_CONFIG_DIR.to_string());
    let config_dir_expanded = expand_path(&config_dir);

    let config_path = Path::new(&config_dir_expanded);
    if config_dir.is_empty() || !config_path.exists() {
        return Err(SetupError::ConfigDirInvalid(config_dir));
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
    config_dir: &Path,
    history: &HistoryConfig,
) -> StdResult<String, SetupError> {
    let mut history_dir_path = history.directory_expanded();
    if history_dir_path.is_empty() {
        let default_history_path = {
            let path = config_dir.join("history").clone();
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

fn list_local_protos() -> std::io::Result<(String, Vec<String>)> {
    let current_dir = std::env::current_dir()?;
    let current_dir_str = current_dir.to_string_lossy().to_string();

    let mut proto_files = Vec::new();
    for entry in std::fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "proto") {
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                proto_files.push(file_name.to_string());
            }
        }
    }
    Ok((current_dir_str, proto_files))
}

#[derive(Debug)]
pub enum SetupError {
    ConfigDirEnvNotFound,
    ConfigDirInvalid(String),
    ConfigFileNotFound(String),
    HistoryPathNotFound(String),
    LoggerPathNotFound(String),
    CreateDirectory(Box<dyn StdError>),
    InitializeLogger(Box<dyn StdError>),
    ListLocalProtos(std::io::Error),
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupError::ConfigDirEnvNotFound => {
                write!(f, "The config dir ${ENV_CONFIG_DIR} was not found.")
            }
            SetupError::ConfigDirInvalid(d) => {
                write!(f, "The config dir ${d} is not a valid path.")
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
            SetupError::ListLocalProtos(err) => write!(f, "Cannot list local protos: {err}."),
        }
        // let general_error_msg = "Check the README for tips on how to set up wireman: \
        // https://github.com/preiter93/wireman";
        // write!(f, "{}\n{}\n", self.error_msg, general_error_msg)
    }
}

impl StdError for SetupError {}
