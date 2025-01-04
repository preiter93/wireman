use std::error::Error;

use crate::Config;
use logger::Logger;
use once_cell::sync::OnceCell;
use ratatheme::{DeserializeTheme, Subtheme};
use ratatui::style::Style;
use serde::Deserialize;

pub static THEME: OnceCell<Theme> = OnceCell::new();

#[derive(Debug, Clone, DeserializeTheme)]
pub struct Theme {
    #[theme(styles(focused, unfocused))]
    pub base: Base,

    #[theme(styles(focused, unfocused))]
    pub highlight: Highlight,

    #[theme(styles(focused, unfocused))]
    pub border: Border,

    #[theme(styles(focused, unfocused))]
    pub title: Title,

    pub footer: Footer,

    pub status: Status,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Footer {
    pub hide: bool,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Status {
    pub hide: bool,
}

impl Default for Theme {
    fn default() -> Self {
        let toml_str = include_str!("../assets/default.toml");
        let deserializer = toml::Deserializer::new(toml_str);
        Self::deserialize_theme(deserializer).unwrap()
    }
}

#[derive(Debug, Clone, Default, Subtheme)]
pub struct Base {
    #[theme(style)]
    pub focused: Style,

    #[theme(style)]
    pub unfocused: Style,
}

#[derive(Debug, Clone, Default, Subtheme)]
pub struct Highlight {
    #[theme(style)]
    pub focused: Style,

    #[theme(style)]
    pub unfocused: Style,
}

#[derive(Debug, Clone, Default, Subtheme)]
pub struct Title {
    #[theme(style)]
    pub focused: Style,

    #[theme(style)]
    pub unfocused: Style,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Subtheme)]
pub struct Border {
    #[theme(style)]
    pub focused: Style,

    #[theme(style)]
    pub unfocused: Style,
}

impl Theme {
    /// Initializes the `Theme` from a config.
    pub fn init(config: &Config) {
        let theme = config
            .skin
            .as_ref()
            .and_then(|f| read_toml(f).ok())
            .and_then(|t| {
                let deserializer = toml::Deserializer::new(&t);
                Theme::deserialize_theme(deserializer).ok()
            })
            .unwrap_or_default();

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

fn read_toml(file_path: &str) -> Result<String, Box<dyn Error>> {
    let f = shellexpand::env(file_path).map_or(file_path.to_string(), |x| x.to_string());

    let toml_str = std::fs::read_to_string(f)?;

    Ok(toml_str)
}
