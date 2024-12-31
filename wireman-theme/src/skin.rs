use std::{collections::HashMap, error::Error, str::FromStr};

use serde::Deserialize;

use crate::{color::Color, set_fg_bg, Theme};

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
            set_fg_bg!(theme.border.unfocused, target, self.colors);
        }
        if let Some(target) = &self.border.focused {
            set_fg_bg!(theme.border.focused, target, self.colors);
        }

        // Title
        if let Some(target) = &self.title.focused {
            set_fg_bg!(theme.title.focused, target, self.colors);
        }
        if let Some(target) = &self.title.unfocused {
            set_fg_bg!(theme.title.unfocused, target, self.colors);
        }

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
    pub focused: Option<FgBg>,
    pub unfocused: Option<FgBg>,
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
pub(crate) struct FgBg {
    pub foreground: Option<String>,
    pub background: Option<String>,
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
