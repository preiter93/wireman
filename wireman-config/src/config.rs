#![allow(clippy::module_name_repetitions)]
use crate::error::Error;
use crate::error::Result;
use crate::install::expand_file;
use crate::install::expand_path;
use logger::LogLevel;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use theme::Config as ThemeConfig;

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
    /// The ui config
    #[serde(default)]
    pub ui: ThemeConfig,
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
        let f = expand_path(file);
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
        self.includes.iter().map(|e| expand_path(e)).collect()
    }

    /// Gets the files. Tries to shell expand the path if it contains
    ///  environment variables such as $HOME or ~.
    #[must_use]
    pub fn files(&self) -> Vec<String> {
        self.files.iter().map(|e| expand_file(e)).collect()
    }
}

/// The config for the server values of the grpc client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct ServerConfig {
    /// The default address
    pub default_address: Option<String>,
    /// The default auth header
    pub default_auth_header: Option<String>,
}

impl ServerConfig {
    #[must_use]
    pub fn new(default_address: &str, default_auth_header: &str) -> Self {
        let default_address = if default_address.is_empty() {
            None
        } else {
            Some(default_address.to_string())
        };

        let default_auth_header = if default_auth_header.is_empty() {
            None
        } else {
            Some(default_auth_header.to_string())
        };
        Self {
            default_address,
            default_auth_header,
        }
    }
}

/// The history config of the grpc client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct HistoryConfig {
    /// The directory where the history is saved
    #[serde(default)]
    pub directory: String,
    /// Wheter autosave should be enables
    #[serde(default = "default_autosave")]
    pub autosave: bool,
    /// Whether the history is disabled
    #[serde(default)]
    pub disabled: bool,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            directory: String::default(),
            autosave: true,
            disabled: false,
        }
    }
}

fn default_autosave() -> bool {
    true
}

impl HistoryConfig {
    /// Instantiate a new history config
    #[must_use]
    pub fn new(directory: &str, autosave: bool, disabled: bool) -> Self {
        Self {
            directory: directory.to_string(),
            autosave,
            disabled,
        }
    }

    /// Returns the path to the history. Tries to shell expand the path if it contains
    /// environment variables such as $HOME or ~.
    #[must_use]
    pub fn directory_expanded(&self) -> String {
        if self.directory.is_empty() {
            return String::new();
        }
        expand_path(&self.directory)
    }
}

/// The logger config for wireman
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct LoggingConfig {
    /// The log level
    #[serde(default)]
    pub level: LogLevel,
    /// The directory to where the log file should be stored
    pub directory: String,
}

impl LoggingConfig {
    /// Instantiate a new logging config
    #[must_use]
    pub fn new(level: LogLevel, file_path: &str) -> Self {
        Self {
            level,
            directory: file_path.to_string(),
        }
    }

    /// Returns the path to the directory of logger file. Tries to shell expand the path if it contains
    /// environment variables such as $HOME or ~.
    #[must_use]
    pub fn directory_expanded(&self) -> String {
        if self.directory.is_empty() {
            return String::new();
        }
        expand_path(&self.directory)
    }

    /// Returns the path to the logger file. Tries to shell expand the path if it contains
    /// environment variables such as $HOME or ~.
    #[must_use]
    pub fn file_path_expanded(&self) -> String {
        let directory_expanded = self.directory_expanded();
        let file_path = Path::new(&directory_expanded).join(Self::file_name());
        file_path.to_string_lossy().to_string()
    }

    #[must_use]
    pub(crate) fn file_name() -> String {
        String::from("wireman.log")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct TlsConfig {
    pub use_native: Option<bool>,
    pub custom_cert: Option<String>,
}

impl TlsConfig {
    #[must_use]
    pub fn new(use_native: bool) -> Self {
        Self {
            use_native: Some(use_native),
            custom_cert: None,
        }
    }

    #[must_use]
    pub fn custom(custom: &str) -> Self {
        Self {
            use_native: None,
            custom_cert: Some(custom.to_string()),
        }
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
        autosave = false
        [logging]
        directory = "/Users"
        level = "Debug"
        [tls]
        custom_cert = "cert.pem"
        [ui]
        skin = "skin.toml"
        "#;
        let cfg = Config::deserialize_toml(&data).unwrap();
        let expected = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["api.proto".to_string(), "internal.proto".to_string()],
            tls: TlsConfig::custom("cert.pem"),
            server: ServerConfig::new("http://localhost:50051", ""),
            logging: LoggingConfig::new(LogLevel::Debug, "/Users"),
            history: HistoryConfig::new("/Users/test", false, false),
            ui: theme::Config::new(Some(String::from("skin.toml"))),
        };
        assert_eq!(cfg, expected);
    }

    #[test]
    fn test_serialize_toml() {
        let cfg = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["api.proto".to_string(), "internal.proto".to_string()],
            tls: TlsConfig::default(),
            server: ServerConfig::new("http://localhost:50051", ""),
            logging: LoggingConfig::new(LogLevel::Debug, "/Users"),
            history: HistoryConfig::new("/Users/test", false, false),
            ui: theme::Config::new(Some(String::from("skin.toml"))),
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
directory = "/Users"

[ui]
skin = "skin.toml"

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
            ui: ThemeConfig::default(),
        };
        let got = cfg.includes();
        let home = std::env::var("HOME").unwrap();
        assert_eq!(got.first(), Some(&format!("{home}/workspace")));
    }
}
