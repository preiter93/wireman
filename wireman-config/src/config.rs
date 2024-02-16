#![allow(clippy::module_name_repetitions)]
use crate::error::Error;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

/// The top level config.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    /// The include directories in which to search for the protos
    pub includes: Vec<String>,
    /// A list of proto files such as [internal.proto, api.proto]
    pub files: Vec<String>,
    /// Optional TLS settings
    #[serde(default)]
    pub tls: TlsConfig,
    /// The default server address
    #[serde(default)]
    pub default_address: String,
    /// The path in which to store the request history
    #[serde(default)]
    pub history_dir: String,
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
        shellexpand::env(&self.history_dir).map_or(self.history_dir.clone(), |x| x.to_string())
    }
}

/// The TLS config of the grpc client.
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
        default_address = "http://localhost:50051"
        history_dir = "/Users/test"
        [tls]
        custom_cert = "cert.pem"
        "#;
        let cfg = Config::deserialize_toml(&data).unwrap();
        let expected = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["api.proto".to_string(), "internal.proto".to_string()],
            tls: TlsConfig::new(Some("cert.pem".to_string())),
            default_address: "http://localhost:50051".to_string(),
            history_dir: "/Users/test".to_string(),
        };
        assert_eq!(cfg, expected);
    }

    #[test]
    fn test_serialize_toml() {
        let cfg = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["api.proto".to_string(), "internal.proto".to_string()],
            tls: TlsConfig::default(),
            default_address: "http://localhost:50051".to_string(),
            history_dir: "/Users/test".to_string(),
        };
        let expected = r#"includes = ["/Users/myworkspace"]
files = ["api.proto", "internal.proto"]
default_address = "http://localhost:50051"
history_dir = "/Users/test"

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
            default_address: String::new(),
            history_dir: String::new(),
        };
        let got = cfg.includes();
        let home = std::env::var("HOME").unwrap();
        assert_eq!(got.first(), Some(&format!("{home}/workspace")));
    }
}
