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
}

impl Theme {
    pub(crate) fn update_from_skin(&mut self, skin: &skin::Skin) {
        skin.apply_to(self);
        // self.base.update_from_skin(&skin.base);
        // self.list.update_from_skin(&skin.list);
        // self.border.update_from_skin(&skin.border);
        // self.navbar.update_from_skin(&skin.navbar);
        // self.editor.update_from_skin(&skin.editor);
        // self.history.update_from_skin(&skin.history);
        // self.headers.update_from_skin(&skin.headers);
        // self.help.update_from_skin(&skin.help);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Base {
    pub style: Style,
}

// impl Base {
//     fn update_from_skin(&mut self, skin: &skin::Base) {
//         self.style = self
//             .style
//             .fg(skin.foreground.into())
//             .bg(skin.background.into());
//     }
// }

#[derive(Debug, Clone)]
pub struct Border {
    pub border: Style,
    pub border_focused: Style,
    pub text: Style,
    pub text_focused: Style,
    pub border_type: BorderType,
    pub border_type_focused: BorderType,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            border: Style::default(),
            border_focused: Style::default(),
            text: Style::default(),
            text_focused: Style::default(),
            border_type: BorderType::Plain,
            border_type_focused: BorderType::Double,
        }
    }
}

// impl Border {
//     fn update_from_skin(&mut self, skin: &skin::Border) {
//         self.border = self.border.fg(skin.border.into());
//         self.border_focused = self.border_focused.fg(skin.border_focused.into());
//         self.text = self.border_focused.fg(skin.text.into());
//         self.text_focused = self.border_focused.fg(skin.text_focused.into());
//     }
// }

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

impl Theme {
    /// Initializes the `Theme` from a config.
    pub fn init(config: &Config) {
        let mut theme = Theme::default();

        if let Some(skin_file_path) = &config.skin {
            match skin::Skin::from_file(skin_file_path) {
                Ok(skin) => {
                    theme.update_from_skin(&skin);
                }
                Err(err) => {
                    Logger::debug(format!(
                        "Failed read skin from file {skin_file_path}, err: {err}"
                    ));
                    let default_skin = skin::Skin::default();
                    theme.update_from_skin(&default_skin);
                }
            }
        } else {
            let default_skin = skin::Skin::default();
            theme.update_from_skin(&default_skin);
        }

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
