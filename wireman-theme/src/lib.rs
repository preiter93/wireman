mod config;
pub use config::Config;

use logger::Logger;
use once_cell::sync::OnceCell;

pub static THEME: OnceCell<Theme> = OnceCell::new();

#[derive(Debug, Default, Clone)]
pub struct Theme {
    pub root: RootTheme,
}

#[derive(Debug, Default, Clone)]
pub struct RootTheme {
    pub hide_footer_help: bool,
}

impl Theme {
    /// Initializes the `Theme` from a config.
    pub fn init(config: &Config) {
        let mut theme = Theme::default();
        theme.root.hide_footer_help = config.hide_footer_help;

        let _ = THEME.set(theme.clone());
    }

    /// Gets the globally shared theme data
    #[must_use]
    pub fn global() -> &'static Theme {
        THEME.get_or_init(|| {
            Logger::debug("Theme was not initialized. Fallback to default.");
            Theme::default()
        })
    }
}
