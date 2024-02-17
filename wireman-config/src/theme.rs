use theme::{RootTheme, Theme, THEME};

use crate::error::{Error, Result};
use crate::{config::UiConfig, setup::SetupError};

/// Initialize `Theme` from the ui config
///
/// # Errors
/// - Could not set global theme
pub fn init_theme(config: &UiConfig) -> Result<()> {
    let root = RootTheme {
        hide_footer_help: config.hide_footer_help,
    };
    let theme = Theme { root };
    THEME.set(theme).map_err(|_| {
        Error::SetupError(SetupError::new("Failed to initialize global theme data"))
    })?;
    Ok(())
}
