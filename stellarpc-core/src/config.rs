#![allow(clippy::module_name_repetitions)]
use crate::client::tls::TlsConfig;
use crate::error::Error;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

/// The shell command of grpcurl.
pub static CMD_GRPC: &str = "grpcurl";

/// The top level config.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Config {
    /// The top level workspace
    pub workspace: String,
    /// A list of proto files such as [internal.proto, api.proto]
    pub files: Vec<String>,
    /// Optional TLS settings
    #[serde(default)]
    pub tls: TlsConfig,
    /// The default server address
    #[serde(default)]
    pub address: String,
}

impl Config {
    /// Loads the config from a file.
    pub(crate) fn load(file: &str) -> Result<Self> {
        let data = read_to_string(file).map_err(Error::ReadConfigError)?;
        Self::parse_from_str(&data)
    }

    /// Parses the config from a string.
    fn parse_from_str(data: &str) -> Result<Self> {
        serde_json::from_str(data).map_err(Error::ParseConfigError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::tls::TlsConfig;

    #[test]
    fn test_parse_from_str() {
        let data = r#"
        {
            "workspace": "/Users/myworkspace",
            "files": [
                "lucky.proto",
                "luke.proto"
            ],
            "address": "http://localhost:50051"
        }"#;
        let cfg = Config::parse_from_str(&data).unwrap();
        let expected = Config {
            workspace: "/Users/myworkspace".to_string(),
            files: vec!["lucky.proto".to_string(), "luke.proto".to_string()],
            tls: TlsConfig::default(),
            address: "http://localhost:50051".to_string(),
        };
        assert_eq!(cfg, expected);
    }

    #[test]
    fn test_parse_from_str_with_tls() {
        let data = r#"
        {
            "workspace": "/Users/myworkspace",
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
            workspace: "/Users/myworkspace".to_string(),
            files: vec!["lucky.proto".to_string(), "luke.proto".to_string()],
            tls: TlsConfig::new(Some("cert.pem".to_string())),
            address: String::new(),
        };
        assert_eq!(cfg, expected);
    }
}
