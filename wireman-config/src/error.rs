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

    /// Error  serializing toml-formatted config
    #[error("error serializing config")]
    SerializeConfigError(#[source] toml::ser::Error),

    /// Error deserializing toml-formatted config
    #[error("error deserializing config")]
    DeserializeConfigError(#[source] toml::de::Error),
}
