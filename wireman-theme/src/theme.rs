use crate::{skin, Config};
use logger::Logger;
use once_cell::sync::OnceCell;
use ratatui::{style::Style, widgets::BorderType};

pub static THEME: OnceCell<Theme> = OnceCell::new();

#[derive(Debug, Default, Clone)]
pub struct Theme {
    pub base: Base,
    pub list: List,
    pub border: Border,
    pub navbar: Navbar,
    pub editor: Editor,
    pub history: History,
    pub headers: Headers,
    pub footer: Footer,
    pub help_dialog: HelpDialog,
}

impl Theme {
    pub(crate) fn update_from_skin(&mut self, skin: &skin::Skin) {
        skin.apply_to(self);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Base {
    pub style: Style,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone)]
pub struct Border {
    pub border: (Style, Style),
    pub text: (Style, Style),
    pub border_type: (BorderType, BorderType),
}

impl Default for Border {
    fn default() -> Self {
        Self {
            border: (Style::default(), Style::default()),
            text: (Style::default(), Style::default()),
            border_type: (BorderType::Plain, BorderType::Double),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Navbar {
    pub title: Style,
    pub tabs: (Style, Style),
}

#[derive(Debug, Clone, Default)]
pub struct List {
    pub text: Style,
    pub focused: Style,
}

#[derive(Debug, Clone, Default)]
pub struct Editor {
    pub text: Style,
    pub cursor: Style,
    pub selection: Style,
    pub status_text: Style,
    pub status_line: Style,
    pub hide_status_line: bool,
}

#[derive(Debug, Clone, Default)]
pub struct History {
    pub active: (Style, Style),
    pub inactive: (Style, Style),
}

#[derive(Debug, Clone, Default)]
pub struct Headers {
    pub titles: Style,
    pub tabs: (Style, Style),
}

#[derive(Debug, Clone, Default)]
pub struct Footer {
    pub tabs: Style,
    pub text: Style,
    pub hide: bool,
}

#[derive(Debug, Clone, Default)]
pub struct HelpDialog {
    pub style: Style,
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
