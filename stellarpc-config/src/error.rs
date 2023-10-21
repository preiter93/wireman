#![allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
use thiserror::Error as ThisError;

/// The result type for this library
pub type Result<T> = std::result::Result<T, Error>;

/// The error type
#[derive(ThisError, Debug)]
pub enum Error {
    /// Error while reading the config file
    #[error("error reading config")]
    ReadConfigError(#[source] std::io::Error),

    /// Error while parsing the config
    #[error("error parsing config")]
    ParseConfigError(#[source] serde_json::Error),
}
