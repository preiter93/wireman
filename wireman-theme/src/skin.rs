use std::error::Error;

use ratatui::style::Stylize;
use serde::Deserialize;

use crate::{color::Color, set_fg_bg, set_focusable, Theme};

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
    #[serde(default)]
    pub help_dialog: HelpDialog,
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
        let hc2 = secondary_highlight_color();
        let dc = default_text_disabled_color();
        let tc = default_title_color();

        // Base
        theme.base.style = theme.base.style.fg(fc.0).bg(bc.0);

        // Border
        let unfocused = self.border.unfocused.as_ref();
        let focused = self.border.focused.as_ref();
        set_focusable!(theme.border.text, self.border.text, fc, bc, tc, bc);
        set_fg_bg!(theme.border.border.0, unfocused, fc, bc);
        set_fg_bg!(theme.border.border.1, focused, fc, bc);

        // Navbar
        set_fg_bg!(theme.navbar.title, self.navbar.title, tc, bc);
        let title = self.navbar.title.as_ref();
        if title.and_then(|x| x.bold).unwrap_or(true) {
            theme.navbar.title = theme.navbar.title.bold();
        }
        set_focusable!(theme.navbar.tabs, self.navbar.tabs, fc, bc, bc, hc2);

        // List
        set_fg_bg!(theme.list.text, self.list.unfocused, fc, bc);
        set_fg_bg!(theme.list.focused, self.list.focused, bc, hc);

        // Editor
        set_fg_bg!(theme.editor.text, self.editor.text, fc, bc);
        set_fg_bg!(theme.editor.cursor, self.editor.cursor, bc, fc);
        set_fg_bg!(theme.editor.selection, self.editor.selection, bc, hc);
        let (sc1, sc2) = default_editor_status_line_colors();
        let status_line = self.editor.status_line.as_ref();
        set_fg_bg!(
            theme.editor.status_text,
            status_line.and_then(|x| x.primary.as_ref()),
            bc,
            sc1
        );

        set_fg_bg!(
            theme.editor.status_line,
            status_line.and_then(|x| x.secondary.as_ref()),
            fc,
            sc2
        );
        if status_line.and_then(|x| x.bold).unwrap_or(false) {
            theme.editor.status_text = theme.editor.status_text.bold();
        }
        if let Some(hide_status_line) = status_line.and_then(|x| x.hide) {
            theme.editor.hide_status_line = hide_status_line;
        }

        // History
        let inactive = self.history.inactive.as_ref();
        let active = self.history.active.as_ref();
        set_focusable!(theme.history.inactive, inactive, dc, bc, bc, dc);
        set_focusable!(theme.history.active, active, hc, bc, bc, hc);

        // Headers
        set_fg_bg!(theme.headers.titles, self.headers.titles, tc, bc);
        set_focusable!(theme.headers.tabs, self.headers.tabs, fc, bc, bc, hc);

        // Footer
        set_fg_bg!(theme.footer.tabs, self.footer.tabs, bc, dc);
        set_fg_bg!(theme.footer.text, self.footer.text, dc, bc);
        if let Some(hide_footer) = self.footer.hide {
            theme.footer.hide = hide_footer;
        } else {
            theme.footer.hide = false;
        }

        // Help dialog
        let foreground = self.help_dialog.foreground;
        let background = self.help_dialog.background;
        let bl = default_help_dialog_background_color();
        theme.help_dialog.style = theme.help_dialog.style.fg(foreground.unwrap_or(fc).0);
        theme.help_dialog.style = theme.help_dialog.style.bg(background.unwrap_or(bl).0);
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
    pub focused: Option<FgBg>,
    pub unfocused: Option<FgBg>,
    pub text: Option<Focusable>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Navbar {
    title: Option<Title>,
    tabs: Option<Focusable>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct List {
    unfocused: Option<FgBg>,
    focused: Option<FgBg>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Editor {
    text: Option<FgBg>,
    cursor: Option<FgBg>,
    selection: Option<FgBg>,
    status_line: Option<StatusLine>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct History {
    active: Option<Focusable>,
    inactive: Option<Focusable>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Headers {
    titles: Option<Title>,
    tabs: Option<Focusable>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Footer {
    tabs: Option<FgBg>,
    text: Option<FgBg>,
    hide: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct HelpDialog {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

pub fn default_background_color() -> Color {
    DARK_NIGHT
}

pub fn default_foreground_color() -> Color {
    WHITE
}

pub fn default_highlight_color() -> Color {
    ORANGE
}

pub fn secondary_highlight_color() -> Color {
    GREEN
}

pub fn default_title_color() -> Color {
    MAGENTA
}

pub fn default_editor_status_line_colors() -> (Color, Color) {
    (GREEN, DARK_NIGHT)
}

pub fn default_text_disabled_color() -> Color {
    GRAY
}

pub fn default_help_dialog_background_color() -> Color {
    GREEN
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Title {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub bold: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Focusable {
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
    pub primary: Option<FgBg>,
    pub secondary: Option<FgBg>,
    pub bold: Option<bool>,
    pub hide: Option<bool>,
}

const WHITE: Color = Color::rgb(241, 245, 249);
const GRAY: Color = Color::rgb(68, 71, 90);
const DARK_NIGHT: Color = Color::rgb(16, 17, 22);
const ORANGE: Color = Color::rgb(255, 153, 0);
const MAGENTA: Color = Color::rgb(255, 51, 204);
const GREEN: Color = Color::rgb(0, 204, 102);

pub(crate) mod macros {
    #[macro_export]
    macro_rules! set_fg_bg {
        ($theme:expr, $skin:expr, $fg:expr, $bg:expr) => {
            $theme = $theme.fg($skin.as_ref().and_then(|x| x.foreground).unwrap_or($fg).0);
            $theme = $theme.bg($skin.as_ref().and_then(|x| x.background).unwrap_or($bg).0);
        };
    }
    #[macro_export]
    macro_rules! set_focusable {
        ($theme:expr, $skin:expr, $fg_unfocused:expr, $bg_unfocused:expr, $fg_focused:expr, $bg_focused:expr) => {
            $theme.0 = $theme.0.fg($skin
                .as_ref()
                .and_then(|t| t.unfocused.as_ref())
                .and_then(|x| x.foreground)
                .unwrap_or($fg_unfocused)
                .0);
            $theme.0 = $theme.0.bg($skin
                .as_ref()
                .and_then(|t| t.unfocused.as_ref())
                .and_then(|x| x.background)
                .unwrap_or($bg_unfocused)
                .0);
            $theme.1 = $theme.1.fg($skin
                .as_ref()
                .and_then(|t| t.focused.as_ref())
                .and_then(|x| x.foreground)
                .unwrap_or($fg_focused)
                .0);
            $theme.1 = $theme.1.bg($skin
                .as_ref()
                .and_then(|t| t.focused.as_ref())
                .and_then(|x| x.background)
                .unwrap_or($bg_focused)
                .0);
        };
    }
}

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
        assert_eq!(skin.base.background, Color::rgb(16, 17, 22));
    }
    #[test]
    fn test_skin_from_file() {
        // Load Skin from file
        let skin: Skin = Skin::from_file(&test_resource("default.toml")).unwrap();
        // Assert
        assert_eq!(skin.base.background, Color::rgb(15, 23, 42));
    }
}
