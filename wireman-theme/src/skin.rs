use std::{collections::HashMap, error::Error, str::FromStr};

use ratatui::style::Stylize;
use serde::Deserialize;

use crate::{color::Color, set_fg_bg, Theme};

#[derive(Debug, Deserialize)]
pub struct Skin {
    #[serde(default)]
    pub colors: HashMap<String, Color>,
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

impl Default for Skin {
    fn default() -> Self {
        toml::from_str(include_str!("../assets/default.toml")).unwrap()
    }
}

impl Skin {
    pub(crate) fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let f = shellexpand::env(file_path).map_or(file_path.to_string(), |x| x.to_string());

        let toml_content = std::fs::read_to_string(f)?;

        Ok(toml::from_str(&toml_content)?)
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn apply_to(&self, theme: &mut Theme) {
        // Base
        let fc = resolve_color(&self.colors, self.base.foreground.as_deref());
        let bc = resolve_color(&self.colors, self.base.background.as_deref());
        if let Some(fc) = fc {
            theme.base.style = theme.base.style.fg(fc.0);
        }
        if let Some(bc) = bc {
            theme.base.style = theme.base.style.bg(bc.0);
        }

        // Border
        if let Some(target) = &self.border.unfocused {
            set_fg_bg!(theme.border.border.0, target, self.colors);
        }
        if let Some(target) = &self.border.focused {
            set_fg_bg!(theme.border.border.1, target, self.colors);
        }
        if let Some(target) = &self.border.text.as_ref().and_then(|x| x.unfocused.as_ref()) {
            set_fg_bg!(theme.border.text.0, target, self.colors);
        }
        if let Some(target) = &self.border.text.as_ref().and_then(|x| x.focused.as_ref()) {
            set_fg_bg!(theme.border.text.1, target, self.colors);
        }

        // Navbar
        let title = self.navbar.title.as_ref();
        if title.and_then(|x| x.bold).unwrap_or(true) {
            theme.navbar.title = theme.navbar.title.bold();
        }
        if let Some(target) = &self.navbar.title {
            set_fg_bg!(theme.navbar.title, target, self.colors);
        }

        let unfocused = self.navbar.tabs.as_ref().and_then(|x| x.unfocused.as_ref());
        if let Some(target) = &unfocused {
            set_fg_bg!(theme.navbar.tabs.0, target, self.colors);
        }
        if unfocused.and_then(|x| x.bold).unwrap_or(false) {
            theme.navbar.tabs_bold.0 = true;
        }

        let focused = self.navbar.tabs.as_ref().and_then(|x| x.focused.as_ref());
        if let Some(target) = &focused {
            set_fg_bg!(theme.navbar.tabs.1, target, self.colors);
        }
        if focused.and_then(|x| x.bold).unwrap_or(false) {
            theme.navbar.tabs_bold.1 = true;
        }

        // List
        let active = &self.list.active;
        if let Some(target) = active.as_ref().and_then(|x| x.unselected.as_ref()) {
            set_fg_bg!(theme.list.active.unselected, target, self.colors);
        }
        if let Some(target) = active.as_ref().and_then(|x| x.selected.as_ref()) {
            set_fg_bg!(theme.list.active.selected, target, self.colors);
        }
        let inactive = &self.list.inactive;
        if let Some(target) = inactive.as_ref().and_then(|x| x.unselected.as_ref()) {
            set_fg_bg!(theme.list.inactive.unselected, target, self.colors);
        }
        if let Some(target) = inactive.as_ref().and_then(|x| x.selected.as_ref()) {
            set_fg_bg!(theme.list.inactive.selected, target, self.colors);
        }

        // Editor
        if let Some(target) = &self.editor.text.as_ref() {
            set_fg_bg!(theme.editor.text, target, self.colors);
        }
        if let Some(target) = &self.editor.cursor.as_ref() {
            set_fg_bg!(theme.editor.cursor, target, self.colors);
        }
        if let Some(target) = &self.editor.selection.as_ref() {
            set_fg_bg!(theme.editor.selection, target, self.colors);
        }

        // Status line
        let status_line = self.editor.status_line.as_ref();
        let primary = status_line.and_then(|x| x.primary.as_ref());
        if let Some(target) = primary {
            set_fg_bg!(theme.editor.status_text, target, self.colors);
        }
        let secondary = status_line.and_then(|x| x.secondary.as_ref());
        if let Some(target) = secondary {
            set_fg_bg!(theme.editor.status_line, target, self.colors);
        }

        if let Some(hide_status_line) = status_line.and_then(|x| x.hide) {
            theme.editor.hide_status_line = hide_status_line;
        }

        if status_line.and_then(|x| x.bold).unwrap_or(true) {
            theme.editor.status_text = theme.editor.status_text.bold();
        }

        // History
        let history = &self.history;
        if let Some(target) = history.inactive.as_ref().and_then(|x| x.unfocused.as_ref()) {
            set_fg_bg!(theme.history.inactive.0, target, self.colors);
        }
        if let Some(target) = history.inactive.as_ref().and_then(|x| x.focused.as_ref()) {
            set_fg_bg!(theme.history.inactive.1, target, self.colors);
        }
        if let Some(target) = history.active.as_ref().and_then(|x| x.unfocused.as_ref()) {
            set_fg_bg!(theme.history.active.0, target, self.colors);
        }
        if let Some(target) = history.active.as_ref().and_then(|x| x.focused.as_ref()) {
            set_fg_bg!(theme.history.active.1, target, self.colors);
        }

        // Headers
        let headers = &self.headers;
        let unfocused = headers.titles.as_ref().and_then(|x| x.unfocused.as_ref());
        if let Some(target) = unfocused {
            set_fg_bg!(theme.headers.titles.0, target, self.colors);
        }
        let focused = headers.titles.as_ref().and_then(|x| x.focused.as_ref());
        if let Some(target) = focused {
            set_fg_bg!(theme.headers.titles.1, target, self.colors);
        }

        let active = headers.tabs.as_ref().and_then(|x| x.active.as_ref());
        if let Some(target) = active.and_then(|x| x.unfocused.as_ref()) {
            set_fg_bg!(theme.headers.tabs.active.0, target, self.colors);
        }
        if let Some(target) = active.and_then(|x| x.focused.as_ref()) {
            set_fg_bg!(theme.headers.tabs.active.1, target, self.colors);
        }

        let inactive = headers.tabs.as_ref().and_then(|x| x.inactive.as_ref());
        if let Some(target) = inactive.and_then(|x| x.unfocused.as_ref()) {
            set_fg_bg!(theme.headers.tabs.inactive.0, target, self.colors);
        }
        if let Some(target) = inactive.and_then(|x| x.focused.as_ref()) {
            set_fg_bg!(theme.headers.tabs.inactive.1, target, self.colors);
        }

        // Footer
        let footer = &self.footer;
        if let Some(target) = footer.tabs.as_ref() {
            set_fg_bg!(theme.footer.tabs, target, self.colors);
        }
        if let Some(target) = footer.text.as_ref() {
            set_fg_bg!(theme.footer.text, target, self.colors);
        }

        if let Some(hide_footer) = self.footer.hide {
            theme.footer.hide = hide_footer;
        } else {
            theme.footer.hide = false;
        }

        // Help dialog
        let fc = resolve_color(&self.colors, self.help_dialog.foreground.as_deref());
        let bc = resolve_color(&self.colors, self.help_dialog.background.as_deref());
        if let Some(fc) = fc {
            theme.help_dialog.style = theme.base.style.fg(fc.0);
        }
        if let Some(bc) = bc {
            theme.help_dialog.style = theme.base.style.bg(bc.0);
        }
    }
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Base {
    pub background: Option<String>,
    pub foreground: Option<String>,
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
    active: Option<Selectable>,
    inactive: Option<Selectable>,
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
    titles: Option<Focusable>,
    tabs: Option<HeaderTabs>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct HeaderTabs {
    active: Option<Focusable>,
    inactive: Option<Focusable>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Footer {
    tabs: Option<FgBg>,
    text: Option<FgBg>,
    hide: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct HelpDialog {
    pub foreground: Option<String>,
    pub background: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Title {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub bold: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Focusable {
    pub unfocused: Option<Title>,
    pub focused: Option<Title>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Selectable {
    pub selected: Option<FgBg>,
    pub unselected: Option<FgBg>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FgBg {
    pub foreground: Option<String>,
    pub background: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct StatusLine {
    pub primary: Option<FgBg>,
    pub secondary: Option<FgBg>,
    pub bold: Option<bool>,
    pub hide: Option<bool>,
}

pub(crate) fn resolve_color(colors: &HashMap<String, Color>, color: Option<&str>) -> Option<Color> {
    let color = color?;

    if let Some(color) = colors.get(color) {
        return Some(*color);
    }

    Color::from_str(color).ok()
}

pub(crate) mod macros {
    #[macro_export]
    macro_rules! set_fg_bg {
        ($theme:expr, $fg_bg:expr, $colors:expr) => {
            let fc = resolve_color(&$colors, $fg_bg.foreground.as_deref());
            let bc = resolve_color(&$colors, $fg_bg.background.as_deref());
            if let Some(fc) = fc {
                $theme = $theme.fg(fc.0);
            }
            if let Some(bc) = bc {
                $theme = $theme.bg(bc.0);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skin_default() {
        let skin: Skin = Skin::default();

        let background = resolve_color(&skin.colors, skin.base.background.as_deref());
        assert_eq!(background, Some(Color::rgb(16, 17, 22)));
    }
}
