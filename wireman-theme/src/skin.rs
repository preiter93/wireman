use std::error::Error;

use ratatui::style::Stylize;
use serde::Deserialize;

use crate::{color::Color, Theme};

macro_rules! set_fg {
    ($theme:expr, $skin:expr, $default:expr) => {
        $theme = $theme.fg($skin.unwrap_or($default).0);
    };
}

macro_rules! set_bg {
    ($theme:expr, $skin:expr, $default:expr) => {
        $theme = $theme.bg($skin.unwrap_or($default).0);
    };
}

#[derive(Debug, Deserialize, Default)]
pub struct Skin {
    #[serde(default)]
    pub base: Base,
    #[serde(default)]
    pub border: Border,
    #[serde(default)]
    pub navbar: Navbar,
    #[serde(default)]
    pub list: List,
    #[serde(default)]
    pub editor: Editor,
    #[serde(default)]
    pub history: History,
    #[serde(default)]
    pub headers: Headers,
    #[serde(default)]
    pub footer: Footer,
}

impl Skin {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let f = shellexpand::env(file_path).map_or(file_path.to_string(), |x| x.to_string());
        let toml_content = std::fs::read_to_string(f)?;
        let skin: Self = toml::from_str(&toml_content)?;
        Ok(skin)
    }

    pub fn apply_to(&self, theme: &mut Theme) {
        let fc = self.base.foreground;
        let bc = self.base.background;
        let hc = default_highlight_color();
        let dc = default_text_disabled_color();

        // Base
        theme.base.style = theme.base.style.fg(fc.0).bg(bc.0);

        // Border
        let unfocused = self.border.unfocused.as_ref();
        let focused = self.border.focused.as_ref();
        set_fg!(theme.border.text, unfocused.map(|x| x.text).flatten(), fc);
        set_fg!(
            theme.border.text_focused,
            focused.map(|x| x.text).flatten(),
            fc
        );
        set_fg!(
            theme.border.border,
            unfocused.map(|x| x.foreground).flatten(),
            fc
        );
        set_fg!(
            theme.border.border_focused,
            focused.map(|x| x.foreground).flatten(),
            fc
        );

        // Navbar
        let title = self.navbar.title.as_ref();
        set_fg!(
            theme.navbar.title,
            title.map(|x| x.foreground).flatten(),
            hc
        );
        set_bg!(
            theme.navbar.title,
            title.map(|x| x.background).flatten(),
            bc
        );
        if title.map(|x| x.bold).flatten().unwrap_or(true) {
            theme.navbar.title = theme.navbar.title.bold();
        }
        let tabs = self.navbar.tabs.as_ref();
        set_fg!(
            theme.navbar.tabs,
            tabs.and_then(|x| x.unfocused.as_ref().map(|x| x.foreground))
                .flatten(),
            fc
        );
        set_bg!(
            theme.navbar.tabs,
            tabs.and_then(|x| x.unfocused.as_ref().map(|x| x.background))
                .flatten(),
            bc
        );
        set_fg!(
            theme.navbar.tabs_focused,
            tabs.and_then(|x| x.focused.as_ref().map(|x| x.foreground))
                .flatten(),
            bc
        );
        set_bg!(
            theme.navbar.tabs_focused,
            tabs.and_then(|x| x.focused.as_ref().map(|x| x.background))
                .flatten(),
            hc
        );

        // List
        set_fg!(
            theme.list.text,
            self.list.unfocused.as_ref().map(|x| x.foreground).flatten(),
            fc
        );
        set_bg!(
            theme.list.text,
            self.list.unfocused.as_ref().map(|x| x.background).flatten(),
            bc
        );
        set_fg!(
            theme.list.focused,
            self.list.focused.as_ref().map(|x| x.foreground).flatten(),
            bc
        );
        set_bg!(
            theme.list.focused,
            self.list.focused.as_ref().map(|x| x.background).flatten(),
            fc
        );

        // Editor
        set_fg!(theme.editor.text, self.editor.text, fc);
        let cursor = self.editor.cursor.as_ref();
        set_fg!(
            theme.editor.cursor,
            cursor.map(|x| x.foreground).flatten(),
            bc
        );
        set_bg!(
            theme.editor.cursor,
            cursor.map(|x| x.background).flatten(),
            fc
        );
        let selection = self.editor.selection.as_ref();
        set_fg!(
            theme.editor.selection,
            selection.map(|x| x.foreground).flatten(),
            bc
        );
        set_bg!(
            theme.editor.selection,
            selection.map(|x| x.background).flatten(),
            fc
        );
        let status_line = self.editor.status_line.as_ref();
        let (sc1, sc2) = default_editor_status_line_colors();
        set_fg!(
            theme.editor.status_text,
            status_line.map(|x| x.foreground).flatten(),
            fc
        );
        set_bg!(
            theme.editor.status_text,
            status_line.map(|x| x.background).flatten(),
            sc1
        );
        set_bg!(
            theme.editor.status_line,
            status_line.map(|x| x.secondary).flatten(),
            sc2
        );
        if status_line.map(|x| x.bold).flatten().unwrap_or(false) {
            theme.editor.status_text = theme.editor.status_text.bold();
        }
        if let Some(hide_status_line) = status_line.map(|x| x.hide).flatten() {
            theme.editor.hide_status_line = hide_status_line;
        }

        // History
        let inactive = self.history.inactive.as_ref();
        let active = self.history.active.as_ref();
        let focused = self.history.focused.as_ref();
        set_fg!(
            theme.history.inactive,
            inactive.as_ref().map(|x| x.foreground).flatten(),
            dc
        );
        set_bg!(
            theme.history.inactive,
            inactive.as_ref().map(|x| x.background).flatten(),
            bc
        );
        set_fg!(
            theme.history.active,
            active.as_ref().map(|x| x.foreground).flatten(),
            fc
        );
        set_bg!(
            theme.history.active,
            active.as_ref().map(|x| x.background).flatten(),
            bc
        );
        set_fg!(
            theme.history.focused,
            focused.as_ref().map(|x| x.foreground).flatten(),
            bc
        );
        set_bg!(
            theme.history.focused,
            focused.as_ref().map(|x| x.background).flatten(),
            fc
        );

        // Headers
        let title = self.headers.titles.as_ref();
        set_fg!(
            theme.headers.titles,
            title.map(|x| x.foreground).flatten(),
            fc
        );
        set_bg!(
            theme.headers.titles,
            title.map(|x| x.background).flatten(),
            hc
        );
        if title.map(|x| x.bold).flatten().unwrap_or(false) {
            theme.headers.titles = theme.headers.titles.bold();
        }
        let tabs = self.headers.tabs.as_ref();
        set_fg!(
            theme.headers.tabs,
            tabs.and_then(|x| x.unfocused.as_ref().map(|x| x.foreground))
                .flatten(),
            fc
        );
        set_bg!(
            theme.headers.tabs,
            tabs.and_then(|x| x.unfocused.as_ref().map(|x| x.background))
                .flatten(),
            bc
        );
        set_fg!(
            theme.headers.tabs_focused,
            tabs.and_then(|x| x.focused.as_ref().map(|x| x.foreground))
                .flatten(),
            bc
        );
        set_bg!(
            theme.headers.tabs_focused,
            tabs.and_then(|x| x.focused.as_ref().map(|x| x.background))
                .flatten(),
            fc
        );

        // Footer
        let tabs = self.footer.tabs.as_ref();
        set_fg!(theme.footer.tabs, tabs.map(|x| x.foreground).flatten(), bc);
        set_bg!(theme.footer.tabs, tabs.map(|x| x.background).flatten(), dc);
        let text = self.footer.text.as_ref();
        set_fg!(theme.footer.text, text.map(|x| x.foreground).flatten(), dc);
        set_bg!(theme.footer.text, text.map(|x| x.background).flatten(), bc);
        if let Some(hide_footer) = self.footer.hide {
            theme.footer.hide = hide_footer;
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Base {
    #[serde(default = "default_background_color")]
    pub background: Color,
    #[serde(default = "default_foreground_color")]
    pub foreground: Color,
}

impl Default for Base {
    fn default() -> Self {
        Self {
            background: default_background_color(),
            foreground: default_foreground_color(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Border {
    pub focused: Option<BorderSkin>,
    pub unfocused: Option<BorderSkin>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Navbar {
    title: Option<Title>,
    tabs: Option<Tabs>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct List {
    unfocused: Option<FgBg>,
    focused: Option<FgBg>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Editor {
    text: Option<Color>,
    cursor: Option<FgBg>,
    selection: Option<FgBg>,
    status_line: Option<StatusLine>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct History {
    active: Option<FgBg>,
    inactive: Option<FgBg>,
    focused: Option<FgBg>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Headers {
    titles: Option<Title>,
    tabs: Option<Tabs>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Footer {
    tabs: Option<FgBg>,
    text: Option<FgBg>,
    hide: Option<bool>,
}

pub fn default_background_color() -> Color {
    SLATE_BLUE
}

pub fn default_foreground_color() -> Color {
    WHITE
}

pub fn default_highlight_color() -> Color {
    PURPLE
}

pub fn default_editor_status_line_colors() -> (Color, Color) {
    (LIGHT_PURPLE, PURPLE)
}

pub fn default_text_disabled_color() -> Color {
    GRAY
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Title {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub bold: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Tabs {
    pub unfocused: Option<FgBg>,
    pub focused: Option<FgBg>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FgBg {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct StatusLine {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub secondary: Option<Color>,
    pub bold: Option<bool>,
    pub hide: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct BorderSkin {
    pub foreground: Option<Color>,
    pub text: Option<Color>,
}

const SLATE_BLUE: Color = Color::rgb(15, 23, 42);
// const SLATE_WHITE: Color = Color::rgb(241, 245, 249);
const WHITE: Color = Color::rgb(255, 255, 255);
const LIGHT_PURPLE: Color = Color::rgb(126, 34, 206);
const PURPLE: Color = Color::rgb(88, 28, 135);
const GRAY: Color = Color::rgb(71, 85, 105);

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn test_resource(file_name: &str) -> String {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(manifest_dir)
            .join("resources/test")
            .join(file_name)
            .into_os_string()
            .into_string()
            .unwrap()
    }

    #[test]
    fn test_skin_from_empty_toml() {
        // Load Skin from file
        let skin: Skin = Skin::from_file(&test_resource("empty.toml")).unwrap();
        // Assert
        assert_eq!(skin.base.background, Color::rgb(15, 23, 42));
    }
    #[test]
    fn test_skin_from_file() {
        // Load Skin from file
        let skin: Skin = Skin::from_file(&test_resource("default.toml")).unwrap();
        // Assert
        assert_eq!(skin.base.background, Color::rgb(15, 23, 42));
    }
}
