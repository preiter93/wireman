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
    pub list: List,
    #[serde(default)]
    pub border: Border,
    #[serde(default)]
    pub navbar: Navbar,
    #[serde(default)]
    pub editor: Editor,
    #[serde(default)]
    pub history: History,
    #[serde(default)]
    pub headers: Headers,
    #[serde(default)]
    pub help: Footer,
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
        set_fg!(theme.border.text, self.border.text, fc);
        set_fg!(theme.border.text_focused, self.border.text_focused, fc);
        set_fg!(theme.border.border, self.border.border, fc);
        set_fg!(theme.border.border_focused, self.border.border_focused, fc);

        // Navbar
        set_fg!(theme.navbar.title, self.navbar.title, hc);
        if self.navbar.title_bold.unwrap_or(true) {
            theme.navbar.title = theme.navbar.title.bold();
        }
        set_fg!(theme.navbar.tabs, self.navbar.tabs_foreground, fc);
        set_fg!(
            theme.navbar.tabs_focused,
            self.navbar.tabs_focused_foreground,
            fc
        );
        set_bg!(
            theme.navbar.tabs_focused,
            self.navbar.tabs_focused_background,
            hc
        );

        // List
        set_fg!(theme.list.text, self.list.foreground, fc);
        set_fg!(theme.list.focused, self.list.focused_foreground, bc);
        set_bg!(theme.list.focused, self.list.focused_background, fc);

        // Editor
        set_fg!(theme.editor.text, self.editor.foreground, fc);
        set_fg!(theme.editor.cursor, self.editor.cursor_foreground, bc);
        set_bg!(theme.editor.cursor, self.editor.cursor_background, fc);
        set_fg!(theme.editor.selection, self.editor.selection_foreground, bc);
        set_bg!(theme.editor.selection, self.editor.selection_background, fc);
        let (sc1, sc2) = default_editor_status_line_colors();
        set_fg!(
            theme.editor.status_text,
            self.editor.status_line_foreground,
            fc
        );
        set_bg!(
            theme.editor.status_text,
            self.editor.status_line_background,
            sc1
        );
        if self.editor.status_line_bold.unwrap_or(false) {
            theme.editor.status_text = theme.editor.status_text.bold();
        }
        set_bg!(
            theme.editor.status_line,
            self.editor.status_line_secondary,
            sc2
        );

        // History
        set_fg!(theme.history.disabled, self.history.disabled, dc);
        set_fg!(theme.history.enabled, self.history.enabled, fc);
        set_fg!(theme.history.focused, self.history.focused_foreground, bc);
        set_bg!(theme.history.focused, self.history.focused_background, fc);

        // Headers
        set_fg!(theme.headers.titles, self.headers.subtitles_foreground, fc);
        set_bg!(theme.headers.titles, self.headers.subtitles_background, hc);
        set_fg!(theme.headers.tabs, self.headers.auth_tabs_foreground, fc);
        set_fg!(
            theme.headers.tabs_focused,
            self.headers.auth_tabs_focused_foreground,
            bc
        );
        set_bg!(
            theme.headers.tabs_focused,
            self.headers.auth_tabs_focused_background,
            fc
        );
        // Footer
        set_fg!(theme.help.key, self.help.key_foreground, bc);
        set_bg!(theme.help.key, self.help.key_background, dc);
        set_fg!(theme.help.description, self.help.description_foreground, dc);
    }
}

#[derive(Debug, Deserialize)]
pub struct Base {
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
pub struct Border {
    pub border: Option<Color>,
    pub border_focused: Option<Color>,
    pub text: Option<Color>,
    pub text_focused: Option<Color>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Navbar {
    pub title: Option<Color>,
    pub title_bold: Option<bool>,
    pub tabs_foreground: Option<Color>,
    pub tabs_focused_foreground: Option<Color>,
    pub tabs_focused_background: Option<Color>,
}

#[derive(Debug, Deserialize, Default)]
pub struct List {
    pub foreground: Option<Color>,
    pub focused_foreground: Option<Color>,
    pub focused_background: Option<Color>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Editor {
    pub foreground: Option<Color>,
    pub cursor_foreground: Option<Color>,
    pub cursor_background: Option<Color>,
    pub selection_foreground: Option<Color>,
    pub selection_background: Option<Color>,
    pub status_line_foreground: Option<Color>,
    pub status_line_background: Option<Color>,
    pub status_line_bold: Option<bool>,
    pub status_line_secondary: Option<Color>,
    pub hide_status_line: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct History {
    pub enabled: Option<Color>,
    pub disabled: Option<Color>,
    pub focused_foreground: Option<Color>,
    pub focused_background: Option<Color>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Headers {
    pub subtitles_foreground: Option<Color>,
    pub subtitles_background: Option<Color>,
    pub subtitles_bold: Option<bool>,
    pub auth_tabs_foreground: Option<Color>,
    pub auth_tabs_focused_foreground: Option<Color>,
    pub auth_tabs_focused_background: Option<Color>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Footer {
    pub key_foreground: Option<Color>,
    pub key_background: Option<Color>,
    pub description_foreground: Option<Color>,
    pub hide: Option<bool>,
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

pub fn default_status_text_background_color() -> Color {
    LIGHT_PURPLE
}

pub fn default_border_color() -> Color {
    SLATE_WHITE
}

pub fn default_status_line_background_color() -> Color {
    PURPLE
}

pub fn default_section_color() -> Color {
    PURPLE
}

pub fn default_false() -> bool {
    false
}

pub fn default_true() -> bool {
    true
}

const SLATE_BLUE: Color = Color::rgb(15, 23, 42);
const SLATE_WHITE: Color = Color::rgb(241, 245, 249);
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
