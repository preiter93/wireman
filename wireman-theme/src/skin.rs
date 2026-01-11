use std::{collections::HashMap, error::Error, str::FromStr};

use ratatui::widgets::BorderType;
use serde::Deserialize;

use crate::{color::Color, set_fg_bg, theme::LineNumbers, Theme};

#[derive(Debug, Deserialize)]
pub struct Skin {
    #[serde(default)]
    pub colors: HashMap<String, Color>,
    #[serde(default)]
    pub base: Base,
    #[serde(default)]
    pub title: Title,
    #[serde(default)]
    pub highlight: Highlight,
    #[serde(default)]
    pub border: Border,
    #[serde(default)]
    pub footer: Footer,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub editor: Editor,
    #[serde(default)]
    pub layout: Layout,
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
        // Layout
        if let Some(main_split) = &self.layout.main_split {
            theme.layout.main_split = match main_split {
                SplitDirection::Vertical => ratatui::layout::Direction::Vertical,
                SplitDirection::Horizontal => ratatui::layout::Direction::Horizontal,
            }
        }

        // Base
        if let Some(target) = &self.base.focused {
            set_fg_bg!(theme.base.focused, target, self.colors);
        }
        if let Some(target) = &self.base.unfocused {
            set_fg_bg!(theme.base.unfocused, target, self.colors);
        }

        // Highlight
        if let Some(target) = &self.highlight.focused {
            set_fg_bg!(theme.highlight.focused, target, self.colors);
        }
        if let Some(target) = &self.highlight.unfocused {
            set_fg_bg!(theme.highlight.unfocused, target, self.colors);
        }

        // Border
        if let Some(target) = &self.border.unfocused {
            if let Some(style) = &target.style {
                set_fg_bg!(theme.border.unfocused, style, self.colors);
            }
            if let Some(border_type) = target.border_type {
                theme.border.border_type_unfocused = border_type.into();
            }
        }
        if let Some(target) = &self.border.focused {
            if let Some(style) = &target.style {
                set_fg_bg!(theme.border.focused, style, self.colors);
            }
            if let Some(border_type) = target.border_type {
                theme.border.border_type_focused = border_type.into();
            }
        }

        // Title
        if let Some(target) = &self.title.focused {
            set_fg_bg!(theme.title.focused, target, self.colors);
        }
        if let Some(target) = &self.title.unfocused {
            set_fg_bg!(theme.title.unfocused, target, self.colors);
        }
        theme.title.focused = theme.title.focused.bold();
        theme.title.unfocused = theme.title.unfocused.bold();

        // Footer
        if let Some(hide_footer) = self.footer.hide {
            theme.hide_footer = hide_footer;
        } else {
            theme.hide_footer = false;
        }

        // Status
        if let Some(hide_status) = self.status.hide {
            theme.hide_status = hide_status;
        } else {
            theme.hide_status = false;
        }

        // Editor
        if let Some(line_numbers) = self.editor.line_numbers {
            theme.editor.line_numbers = line_numbers;
        }
    }
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Base {
    pub focused: Option<FgBg>,
    pub unfocused: Option<FgBg>,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Highlight {
    pub focused: Option<FgBg>,
    pub unfocused: Option<FgBg>,
}

#[derive(Default, Debug, Deserialize)]
pub(crate) struct Title {
    pub focused: Option<FgBg>,
    pub unfocused: Option<FgBg>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Border {
    pub focused: Option<BorderConfig>,
    pub unfocused: Option<BorderConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct BorderConfig {
    #[serde(flatten)]
    pub style: Option<FgBg>,
    pub border_type: Option<SkinBorderType>,
}

/// Border type configuration that can be deserialized from skin files.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SkinBorderType {
    Plain,
    Rounded,
    Double,
    Thick,
}

impl From<SkinBorderType> for BorderType {
    fn from(value: SkinBorderType) -> Self {
        match value {
            SkinBorderType::Plain => BorderType::Plain,
            SkinBorderType::Rounded => BorderType::Rounded,
            SkinBorderType::Double => BorderType::Double,
            SkinBorderType::Thick => BorderType::Thick,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Footer {
    hide: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Status {
    hide: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Editor {
    pub line_numbers: Option<LineNumbers>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct FgBg {
    pub foreground: Option<String>,
    pub background: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct Layout {
    pub main_split: Option<SplitDirection>,
}

#[derive(Default, Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SplitDirection {
    #[default]
    Vertical,
    Horizontal,
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
