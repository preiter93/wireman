#![allow(clippy::module_name_repetitions)]
use crate::error::Grp3Error as Error;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

/// The shell command of grpcurl.
pub static CMD_GRPC: &str = "grpcurl";

/// The top level config.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ProtoConfig {
    /// The top level workspace
    pub workspace: String,
    /// A list of proto files such as [internal.proto, api.proto]
    pub files: Vec<String>,
}

impl ProtoConfig {
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
        let cfg = ProtoConfig::parse_from_str(&data).unwrap();
        let expected = ProtoConfig {
            workspace: "/Users/myworkspace".to_string(),
            files: vec!["lucky.proto".to_string(), "luke.proto".to_string()],
        };
        assert_eq!(cfg, expected);
    }
}
