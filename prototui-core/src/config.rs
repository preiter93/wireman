#![allow(clippy::module_name_repetitions)]
use crate::client::tls::TlsConfig;
use crate::error::PTError as Error;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

/// The shell command of grpcurl.
pub static CMD_GRPC: &str = "grpcurl";

/// The top level config.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ProtoTuiConfig {
    /// The top level workspace
    pub workspace: String,
    /// A list of proto files such as [internal.proto, api.proto]
    pub files: Vec<String>,
    /// Optional TLS settings
    #[serde(default)]
    pub tls: TlsConfig,
}

impl ProtoTuiConfig {
    /// Loads the config from a file.
    pub(crate) fn load(file: &str) -> Result<Self> {
        let data = read_to_string("./config.json").map_err(Error::ReadConfigError)?;
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
            ]
        }"#;
        let cfg = ProtoTuiConfig::parse_from_str(&data).unwrap();
        let expected = ProtoTuiConfig {
            workspace: "/Users/myworkspace".to_string(),
            files: vec!["lucky.proto".to_string(), "luke.proto".to_string()],
            tls: TlsConfig::default(),
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
                "skip_verification": true
            }
        }"#;
        let cfg = ProtoTuiConfig::parse_from_str(&data).unwrap();
        let expected = ProtoTuiConfig {
            workspace: "/Users/myworkspace".to_string(),
            files: vec!["lucky.proto".to_string(), "luke.proto".to_string()],
            tls: TlsConfig::new(true, None),
        };
        assert_eq!(cfg, expected);
    }
}
