use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use ratatui::style::Color as RatatuiColor;
use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) struct Color(pub(crate) RatatuiColor);

impl Color {
    #[allow(dead_code)]
    pub(crate) const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(RatatuiColor::Rgb(r, g, b))
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rcolor = RatatuiColor::from_str(s).map_err(|_| ParseColorError)?;
        Ok(Color(rcolor))
    }
}

/// Error type indicating a failure to parse a color string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ParseColorError;

impl std::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse Colors")
    }
}

impl Deref for Color {
    type Target = RatatuiColor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<RatatuiColor> for Color {
    fn from(value: RatatuiColor) -> Self {
        Self(value)
    }
}

impl From<Color> for RatatuiColor {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;

        impl de::Visitor<'_> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid color string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Parse the color string using ratatuis color parser
                let color = RatatuiColor::from_str(value).map_err(de::Error::custom)?;

                Ok(Color(color))
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}
