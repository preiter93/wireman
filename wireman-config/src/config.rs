#![allow(clippy::module_name_repetitions)]
use crate::error::Error;
use crate::error::Result;
use logger::LogLevel;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

/// The top level config.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    /// The include directories in which to search for the protos
    pub includes: Vec<String>,
    /// A list of proto files such as [internal.proto, api.proto]
    pub files: Vec<String>,
    /// The history config
    #[serde(default)]
    pub history: HistoryConfig,
    /// The server config
    #[serde(default)]
    pub server: ServerConfig,
    /// The logger config
    #[serde(default)]
    pub logging: LoggingConfig,
    /// Optional TLS settings
    #[serde(default)]
    pub tls: TlsConfig,
}

impl Config {
    /// Loads the config from a file.
    ///
    /// # Errors
    ///
    /// Failed to read the config file.
    pub fn load(file: &str) -> Result<Self> {
        let f = shellexpand::env(file).map_or(file.to_string(), |x| x.to_string());
        let data = read_to_string(&f).map_err(|err| Error::ReadConfigError {
            filename: f,
            source: err,
        })?;
        Self::deserialize_toml(&data)
    }

    /// Parses the config from a toml-formatted string.
    ///
    /// # Errors
    ///
    /// Returns an error if serde deserialization fails.
    fn deserialize_toml(data: &str) -> Result<Self> {
        toml::from_str(data).map_err(Error::DeserializeConfigError)
    }

    /// Serializes the config to a toml-formatted string.
    ///
    /// # Errors
    ///
    /// Returns an error if serde serialization fails.
    pub fn serialize_toml(&self) -> Result<String> {
        toml::to_string(self).map_err(Error::SerializeConfigError)
    }

    /// Gets the includes directories. Tries to shell expand the path
    /// if it contains environment variables such as $HOME or ~.
    #[must_use]
    pub fn includes(&self) -> Vec<String> {
        self.includes
            .iter()
            .map(|e| shellexpand::env(e).map_or(e.clone(), |x| x.to_string()))
            .collect()
    }

    /// Gets the files. Tries to shell expand the path if it contains
    ///  environment variables such as $HOME or ~.
    #[must_use]
    pub fn files(&self) -> Vec<String> {
        self.files
            .iter()
            .map(|e| shellexpand::env(e).map_or(e.clone(), |x| x.to_string()))
            .collect()
    }

    /// Gets the history. Tries to shell expand the path if it contains
    /// environment variables such as $HOME or ~.
    #[must_use]
    pub fn history(&self) -> String {
        shellexpand::env(&self.history.directory)
            .map_or(self.history.directory.clone(), |x| x.to_string())
    }
}

/// The TLS config of the grpc client.
/// The config for the server values of the grpc client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct ServerConfig {
    /// The default address
    pub default_address: String,
}

impl ServerConfig {
    pub fn new(default_address: &str) -> Self {
        Self {
            default_address: default_address.to_string(),
        }
    }
}

/// The history config of the grpc client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct HistoryConfig {
    /// The directory where the history is saved
    #[serde(default)]
    pub directory: String,
    /// Wheter autosave should be enables
    #[serde(default)]
    pub autosave: bool,
    /// Whether the history is disabled
    #[serde(default)]
    pub disabled: bool,
}

impl HistoryConfig {
    pub fn new(directory: &str, autosave: bool, disabled: bool) -> Self {
        Self {
            directory: directory.to_string(),
            autosave,
            disabled,
        }
    }
}

/// The logger config for wireman
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct LoggingConfig {
    /// The log level
    #[serde(default)]
    pub level: LogLevel,
    /// The filepath where the log is stored
    pub file_path: String,
}

impl LoggingConfig {
    pub fn new(level: LogLevel, file_path: &str) -> Result<Self> {
        Ok(Self {
            level,
            file_path: file_path.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct TlsConfig {
    /// Custom certificates
    custom_cert: Option<String>,
}

impl TlsConfig {
    /// Instantiate a `TlsConfig`
    #[must_use]
    pub fn new(custom_cert: Option<String>) -> Self {
        Self { custom_cert }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_toml() {
        let data = r#"
        includes = [
            "/Users/myworkspace"
        ]
        files = [
            "api.proto",
            "internal.proto"
        ]
        [server]
        default_address = "http://localhost:50051"
        [history]
        directory = "/Users/test"
        [logging]
        file_path = "/Users/wireman.log"
        level = "Debug"
        [tls]
        custom_cert = "cert.pem"
        "#;
        let cfg = Config::deserialize_toml(&data).unwrap();
        let expected = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["api.proto".to_string(), "internal.proto".to_string()],
            tls: TlsConfig::new(Some("cert.pem".to_string())),
            server: ServerConfig::new("http://localhost:50051"),
            logging: LoggingConfig::new(LogLevel::Debug, "/Users/wireman.log").unwrap(),
            history: HistoryConfig::new("/Users/test", false, false),
        };
        assert_eq!(cfg, expected);
    }

    #[test]
    fn test_serialize_toml() {
        let cfg = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["api.proto".to_string(), "internal.proto".to_string()],
            tls: TlsConfig::default(),
            server: ServerConfig::new("http://localhost:50051"),
            logging: LoggingConfig::new(LogLevel::Debug, "/Users/wireman.log").unwrap(),
            history: HistoryConfig::new("/Users/test", false, false),
        };
        let expected = r#"includes = ["/Users/myworkspace"]
files = ["api.proto", "internal.proto"]

[history]
directory = "/Users/test"
autosave = false
disabled = false

[server]
default_address = "http://localhost:50051"

[logging]
level = "Debug"
file_path = "/Users/wireman.log"

[tls]
"#;
        assert_eq!(cfg.serialize_toml().unwrap(), expected);
    }

    #[test]
    fn test_shell_expand() {
        let cfg = Config {
            includes: vec!["$HOME/workspace".to_string()],
            files: vec![],
            tls: TlsConfig::default(),
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            history: HistoryConfig::default(),
        };
        let got = cfg.includes();
        let home = std::env::var("HOME").unwrap();
        assert_eq!(got.first(), Some(&format!("{home}/workspace")));
    }
}
