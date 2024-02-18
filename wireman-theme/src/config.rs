use serde::{Deserialize, Serialize};

/// The ui config for wireman
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd)]
pub struct Config {
    /// Whether to hide the footer help bar
    #[serde(default)]
    pub hide_footer_help: bool,
}
