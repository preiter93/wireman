use crate::{skin, Config};
use logger::Logger;
use once_cell::sync::OnceCell;
use ratatui::{style::Style, widgets::BorderType};
use serde::{Deserialize, Serialize};

pub static THEME: OnceCell<Theme> = OnceCell::new();

/// Configuration for line numbers in the editor.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineNumbers {
    /// Line numbers are disabled (default).
    #[default]
    None,
    /// Display absolute line numbers.
    Absolute,
    /// Display relative line numbers.
    Relative,
}

#[derive(Debug, Default, Clone)]
pub struct Theme {
    pub base: Base,
    pub highlight: Highlight,
    pub border: Border,
    pub title: Title,
    pub editor: Editor,
    pub hide_footer: bool,
    pub hide_status: bool,
}

impl Theme {
    pub(crate) fn update_from_skin(&mut self, skin: &skin::Skin) {
        skin.apply_to(self);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Base {
    pub focused: Style,
    pub unfocused: Style,
}

#[derive(Debug, Clone, Default)]
pub struct Highlight {
    pub focused: Style,
    pub unfocused: Style,
}

#[derive(Debug, Clone, Default)]
pub struct Title {
    pub focused: Style,
    pub unfocused: Style,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone)]
pub struct Border {
    pub focused: Style,
    pub unfocused: Style,
    pub border_type: BorderType,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            focused: Style::default(),
            unfocused: Style::default(),
            border_type: BorderType::Rounded,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Editor {
    pub line_numbers: LineNumbers,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            line_numbers: LineNumbers::Absolute,
        }
    }
}

impl Theme {
    /// Initializes the `Theme` from a config.
    pub fn init(config: &Config) {
        let skin = config
            .skin
            .as_deref()
            .and_then(|skin_file| match skin::Skin::from_file(skin_file) {
                Ok(skin) => Some(skin),
                Err(err) => {
                    Logger::debug(format!(
                        "Failed to read skin from file {skin_file}, err: {err}"
                    ));
                    None
                }
            })
            .unwrap_or_default();

        let mut theme = Theme::default();
        theme.update_from_skin(&skin);

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
