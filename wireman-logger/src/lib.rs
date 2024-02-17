use chrono::Local;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

// Global instance of the logger
pub static LOGGER: OnceCell<Logger> = OnceCell::new();

// Define logger
#[derive(Debug)]
pub struct Logger {
    file_path: Mutex<String>,
    level: LogLevel,
}

impl Logger {
    /// Instantiates a new `Logger`
    pub fn new<S: Into<String>>(log_file_path: S, log_level: LogLevel) -> Self {
        Self {
            file_path: Mutex::new(log_file_path.into()),
            level: log_level,
        }
    }

    /// Initialize logger with file and log level
    ///
    ///
    /// # Errors
    /// - Could not set global logger
    pub fn init<S: Into<String>>(file_path: S, log_level: LogLevel) -> Result<(), LoggerError> {
        let logger = Logger::new(file_path.into(), log_level);
        LOGGER
            .set(logger)
            .map_err(|_| LoggerError::new("Failed to initialize logger"))
    }

    /// Set a new file path for the `Logger`
    pub fn set_file_path(&self, file_path: String) {
        if let Ok(mut path) = self.file_path.lock() {
            *path = file_path;
        }
    }

    /// Log a message in debug level with the global logger.
    pub fn debug<S: AsRef<str>>(message: S) {
        if let Some(logger) = LOGGER.get() {
            let _ = logger.log(LogLevel::Debug, message);
        }
    }

    /// Log a message in critical level with the global logger.
    pub fn critical<S: AsRef<str>>(message: S) {
        if let Some(logger) = LOGGER.get() {
            let _ = logger.log(LogLevel::Critical, message);
        }
    }

    /// Open or create log file
    fn open_log_file(&self) -> Result<File, LoggerError> {
        let path = self
            .file_path
            .lock()
            .map_err(|e| LoggerError::new(format!("Could not obtain lock: {e}")))?;
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*path)
            .map_err(|e| LoggerError::new(format!("Failed to open or create log file: {e}")))
    }

    // Log a message
    fn log<S: AsRef<str>>(&self, level: LogLevel, message: S) -> Result<(), LoggerError> {
        if level >= self.level {
            let mut file = self.open_log_file()?;
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            writeln!(file, "[{level}] {} [{now}]", message.as_ref())
                .map_err(|e| LoggerError::new(format!("Failed to write to log file: {e}")))?;
        }
        Ok(())
    }
}

// Defines log levels
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub enum LogLevel {
    #[default]
    Debug,
    Critical,
    None,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::None => "None",
            Self::Debug => "Debug",
            Self::Critical => "Critical",
        };
        write!(f, "{name}")
    }
}

// Defines errors happened during logging
#[derive(Debug)]
pub struct LoggerError {
    /// The error message
    pub error_msg: String,
}

impl LoggerError {
    /// Instantiates `LoggerError`
    pub fn new<S: Into<String>>(error_msg: S) -> Self {
        Self {
            error_msg: error_msg.into(),
        }
    }
}

impl Display for LoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

impl Error for LoggerError {
    fn description(&self) -> &str {
        &self.error_msg
    }
}
