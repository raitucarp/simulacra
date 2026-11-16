//! Theme palettes: the small, human-authored color sets (`assets/themes/*.json`) that every
//! [`ThemeTokens`](crate::theme::ThemeTokens) value is derived from.

mod colors;
pub(crate) mod hex_serde;
mod mode;

pub use colors::{PaletteColorKey, PaletteColors, PaletteOverrides};
pub use mode::PaletteMode;

use serde::{Deserialize, Serialize};

/// One palette: identity, light/dark mode, attribution and base colors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePalette {
    /// Stable identifier (`one-dark`, `custom-…`).
    pub id: String,
    /// Display name.
    pub name: String,
    pub mode: PaletteMode,
    /// Upstream author and license of the colors.
    #[serde(default)]
    pub credit: String,
    /// Built-in palette a custom palette was duplicated from (custom palettes only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub based_on: Option<String>,
    pub colors: PaletteColors,
}

impl ThemePalette {
    /// Parses one palette JSON document.
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Pretty JSON in the `assets/themes` schema (for export).
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// This palette with `overrides` applied to its base colors.
    pub fn with_overrides(&self, overrides: &PaletteOverrides) -> Self {
        Self { colors: self.colors.with_overrides(overrides), ..self.clone() }
    }
}

#[cfg(test)]
mod tests;
