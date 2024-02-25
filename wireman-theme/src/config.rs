use serde::{Deserialize, Serialize};

/// The ui config for wireman
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct Config {
    /// The file path to the skin toml file.
    pub skin: Option<String>,
}

impl Config {
    pub fn new(skin: Option<String>) -> Self {
        Self { skin }
    }
}
