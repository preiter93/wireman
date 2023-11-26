#![allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
use thiserror::Error as ThisError;

/// The result type for this library
pub type Result<T> = std::result::Result<T, Error>;

/// The error type
#[derive(ThisError, Debug)]
pub enum Error {
    /// Error while reading the config file
    #[error("error reading config")]
    ReadConfigError {
        filename: String,
        source: std::io::Error,
    },

    /// Error while serializing the config
    #[error("error serializing config")]
    SerializeConfigError(#[source] serde_json::Error),

    /// Error while deserializing the config
    #[error("error deserializing config")]
    DeserializeConfigError(#[source] serde_json::Error),
}
