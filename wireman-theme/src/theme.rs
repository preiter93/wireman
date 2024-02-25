use crate::{skin, Config};
use logger::Logger;
use once_cell::sync::OnceCell;
use ratatui::{
    style::{Style, Stylize},
    widgets::BorderType,
};

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
    pub help: Help,
}

impl Theme {
    pub(crate) fn update_from_skin(&mut self, skin: &skin::Skin) {
        self.base.update_from_skin(&skin.base);
        self.list.update_from_skin(&skin.list);
        self.border.update_from_skin(&skin.border);
        self.navbar.update_from_skin(&skin.navbar);
        self.editor.update_from_skin(&skin.editor);
        self.history.update_from_skin(&skin.history);
        self.headers.update_from_skin(&skin.headers);
        self.help.update_from_skin(&skin.help);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Base {
    pub style: Style,
}

impl Base {
    fn update_from_skin(&mut self, skin: &skin::Base) {
        self.style = self.style.bg(skin.background.into());
    }
}

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

impl Border {
    fn update_from_skin(&mut self, skin: &skin::Border) {
        self.border = self.border.fg(skin.border.into());
        self.border_focused = self.border_focused.fg(skin.border_focused.into());
        self.text = self.border_focused.fg(skin.text.into());
        self.text_focused = self.border_focused.fg(skin.text_focused.into());
    }
}

#[derive(Debug, Clone, Default)]
pub struct Navbar {
    pub title: Style,
    pub tab: Style,
    pub tab_focused: Style,
}

impl Navbar {
    fn update_from_skin(&mut self, skin: &skin::Navbar) {
        self.title = self.title.fg(skin.title.into()).bold();
        self.tab = self.tab.fg(skin.tab_foreground.into());
        self.tab_focused = self
            .tab_focused
            .fg(skin.tab_focused_foreground.into())
            .bg(skin.tab_focused_background.into());
    }
}

#[derive(Debug, Clone, Default)]
pub struct List {
    pub text: Style,
    pub focused: Style,
}

impl List {
    fn update_from_skin(&mut self, skin: &skin::List) {
        self.text = self.text.fg(skin.foreground.into());
        self.focused = self
            .focused
            .fg(skin.focused_foreground.into())
            .bg(skin.focused_background.into());
    }
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

impl Editor {
    fn update_from_skin(&mut self, skin: &skin::Editor) {
        self.text = self.text.fg(skin.foreground.into());
        self.cursor = self
            .cursor
            .fg(skin.cursor_foreground.into())
            .bg(skin.cursor_background.into());
        self.selection = self
            .selection
            .fg(skin.selection_foreground.into())
            .bg(skin.selection_background.into());
        self.status_text = self
            .status_text
            .fg(skin.status_text_foreground.into())
            .bg(skin.status_text_background.into());
        self.status_line = self.status_line.bg(skin.status_line_background.into());
        self.hide_status_line = skin.hide_status_line;
    }
}

#[derive(Debug, Clone, Default)]
pub struct History {
    pub enabled: Style,
    pub disabled: Style,
    pub focused: Style,
}

impl History {
    fn update_from_skin(&mut self, skin: &skin::History) {
        self.enabled = self.enabled.fg(skin.enabled.into());
        self.disabled = self.disabled.fg(skin.disabled.into());
        self.focused = self
            .focused
            .fg(skin.focused_foreground.into())
            .bg(skin.focused_background.into());
    }
}

#[derive(Debug, Clone, Default)]
pub struct Headers {
    pub section: Style,
    pub tabs: Style,
    pub tabs_focused: Style,
}

impl Headers {
    fn update_from_skin(&mut self, skin: &skin::Headers) {
        self.section = self
            .section
            .fg(skin.section_foreground.into())
            .bg(skin.section_background.into());
        if skin.section_bold {
            self.section = self.section.bold();
        }
        self.tabs = self.tabs.fg(skin.tabs_foreground.into());
        self.tabs_focused = self
            .tabs_focused
            .fg(skin.tabs_focused_foreground.into())
            .bg(skin.tabs_focused_background.into());
    }
}

#[derive(Debug, Clone, Default)]
pub struct Help {
    pub key: Style,
    pub description: Style,
    pub hide: bool,
}

impl Help {
    fn update_from_skin(&mut self, skin: &skin::Help) {
        self.key = self
            .key
            .fg(skin.key_foreground.into())
            .bg(skin.key_background.into());
        self.description = self.description.fg(skin.description_foreground.into());
        self.hide = skin.hide;
    }
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
