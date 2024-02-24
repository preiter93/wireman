use std::error::Error;

use serde::Deserialize;

use crate::color::Color;

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
    pub help: Help,
}

impl Skin {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let f = shellexpand::env(file_path).map_or(file_path.to_string(), |x| x.to_string());
        let toml_content = std::fs::read_to_string(f)?;
        let skin: Self = toml::from_str(&toml_content)?;
        Ok(skin)
    }
}

#[derive(Debug, Deserialize)]
pub struct Base {
    #[serde(default = "default_background_color")]
    pub background: Color,
}

impl Default for Base {
    fn default() -> Self {
        Self {
            background: default_background_color(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Border {
    #[serde(default = "default_border_color")]
    pub border: Color,
    #[serde(default = "default_border_color")]
    pub border_focused: Color,
    #[serde(default = "default_border_color")]
    pub text: Color,
    #[serde(default = "default_border_color")]
    pub text_focused: Color,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            border: default_border_color(),
            border_focused: default_border_color(),
            text: default_border_color(),
            text_focused: default_border_color(),
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct Navbar {
    #[serde(default = "default_foreground_color")]
    pub title: Color,
    #[serde(default = "default_foreground_color")]
    pub tab_foreground: Color,
    #[serde(default = "default_background_color")]
    pub tab_focused_foreground: Color,
    #[serde(default = "default_foreground_color")]
    pub tab_focused_background: Color,
}

impl Default for Navbar {
    fn default() -> Self {
        Self {
            title: default_foreground_color(),
            tab_foreground: default_foreground_color(),
            tab_focused_foreground: default_background_color(),
            tab_focused_background: default_foreground_color(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct List {
    #[serde(default = "default_foreground_color")]
    pub foreground: Color,
    #[serde(default = "default_foreground_color")]
    pub focused_foreground: Color,
    #[serde(default = "default_background_color")]
    pub focused_background: Color,
}

impl Default for List {
    fn default() -> Self {
        Self {
            foreground: default_foreground_color(),
            focused_foreground: default_background_color(),
            focused_background: default_foreground_color(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Editor {
    #[serde(default = "default_foreground_color")]
    pub foreground: Color,
    #[serde(default = "default_background_color")]
    pub cursor_foreground: Color,
    #[serde(default = "default_foreground_color")]
    pub cursor_background: Color,
    #[serde(default = "default_background_color")]
    pub selection_foreground: Color,
    #[serde(default = "default_foreground_color")]
    pub selection_background: Color,
    #[serde(default = "default_foreground_color")]
    pub status_text_foreground: Color,
    #[serde(default = "default_status_text_background_color")]
    pub status_text_background: Color,
    #[serde(default = "default_status_line_background_color")]
    pub status_line_background: Color,
    #[serde(default = "default_false")]
    pub hide_status_line: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            foreground: default_foreground_color(),
            cursor_foreground: default_background_color(),
            cursor_background: default_foreground_color(),
            selection_foreground: default_background_color(),
            selection_background: default_foreground_color(),
            status_text_foreground: default_foreground_color(),
            status_text_background: default_status_text_background_color(),
            status_line_background: default_status_line_background_color(),
            hide_status_line: default_false(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct History {
    #[serde(default = "default_foreground_color")]
    pub enabled: Color,
    #[serde(default = "default_text_disabled_color")]
    pub disabled: Color,
    #[serde(default = "default_background_color")]
    pub focused_foreground: Color,
    #[serde(default = "default_foreground_color")]
    pub focused_background: Color,
}

impl Default for History {
    fn default() -> Self {
        Self {
            enabled: default_foreground_color(),
            disabled: default_text_disabled_color(),
            focused_foreground: default_background_color(),
            focused_background: default_foreground_color(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Headers {
    #[serde(default = "default_foreground_color")]
    pub section_foreground: Color,
    #[serde(default = "default_section_color")]
    pub section_background: Color,
    #[serde(default = "default_true")]
    pub section_bold: bool,
    #[serde(default = "default_foreground_color")]
    pub tabs_foreground: Color,
    #[serde(default = "default_background_color")]
    pub tabs_focused_foreground: Color,
    #[serde(default = "default_foreground_color")]
    pub tabs_focused_background: Color,
}

impl Default for Headers {
    fn default() -> Self {
        Self {
            section_foreground: default_foreground_color(),
            section_background: default_section_color(),
            section_bold: default_true(),
            tabs_foreground: default_foreground_color(),
            tabs_focused_foreground: default_background_color(),
            tabs_focused_background: default_foreground_color(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Help {
    #[serde(default = "default_background_color")]
    pub key_foreground: Color,
    #[serde(default = "default_text_disabled_color")]
    pub key_background: Color,
    #[serde(default = "default_text_disabled_color")]
    pub description: Color,
    #[serde(default = "default_false")]
    pub hide: bool,
}

impl Default for Help {
    fn default() -> Self {
        Self {
            key_foreground: default_background_color(),
            key_background: default_text_disabled_color(),
            description: default_text_disabled_color(),
            hide: default_false(),
        }
    }
}

pub fn default_background_color() -> Color {
    SLATE_BLUE
}

pub fn default_foreground_color() -> Color {
    WHITE
}

pub fn default_border_color() -> Color {
    SLATE_WHITE
}

pub fn default_status_text_background_color() -> Color {
    LIGHT_PURPLE
}

pub fn default_status_line_background_color() -> Color {
    PURPLE
}

pub fn default_text_disabled_color() -> Color {
    GRAY
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
