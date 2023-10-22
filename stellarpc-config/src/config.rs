#![allow(clippy::module_name_repetitions)]
use crate::error::Error;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

/// The top level config.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    /// The top level workspace
    pub includes: Vec<String>,
    /// A list of proto files such as [internal.proto, api.proto]
    pub files: Vec<String>,
    /// Optional TLS settings
    #[serde(default)]
    pub tls: TlsConfig,
    /// The default server address
    #[serde(default)]
    pub address: String,
    /// The path in which to store the request history
    #[serde(default)]
    pub history: String,
}

impl Config {
    /// Loads the config from a file.
    pub fn load(file: &str) -> Result<Self> {
        let data = read_to_string(file).map_err(Error::ReadConfigError)?;
        Self::parse_from_str(&data)
    }

    /// Parses the config from a string.
    fn parse_from_str(data: &str) -> Result<Self> {
        serde_json::from_str(data).map_err(Error::ParseConfigError)
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
    fn test_parse_from_str() {
        let data = r#"
        {
            "includes": [
                "/Users/myworkspace"
            ],
            "files": [
                "lucky.proto",
                "luke.proto"
            ],
            "address": "http://localhost:50051",
            "history": "/Users/test"
        }"#;
        let cfg = Config::parse_from_str(&data).unwrap();
        let expected = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["lucky.proto".to_string(), "luke.proto".to_string()],
            tls: TlsConfig::default(),
            address: "http://localhost:50051".to_string(),
            history: "/Users/test".to_string(),
        };
        assert_eq!(cfg, expected);
    }

    #[test]
    fn test_parse_from_str_with_tls() {
        let data = r#"
        {
            "includes": [
                "/Users/myworkspace"
            ],
            "files": [
                "lucky.proto",
                "luke.proto"
            ],
            "tls": {
                "custom_cert": "cert.pem"
            }
        }"#;
        let cfg = Config::parse_from_str(&data).unwrap();
        let expected = Config {
            includes: vec!["/Users/myworkspace".to_string()],
            files: vec!["lucky.proto".to_string(), "luke.proto".to_string()],
            tls: TlsConfig::new(Some("cert.pem".to_string())),
            address: String::new(),
            history: String::new(),
        };
        assert_eq!(cfg, expected);
    }
}
