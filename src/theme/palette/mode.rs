//! Light/dark appearance of a palette.

use serde::{Deserialize, Serialize};

/// Whether a palette is a dark or a light theme (JSON `"mode": "dark" | "light"`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaletteMode {
    #[default]
    Dark,
    Light,
}

impl PaletteMode {
    /// Both modes, dark first.
    pub const ALL: [PaletteMode; 2] = [PaletteMode::Dark, PaletteMode::Light];

    /// True for [`PaletteMode::Dark`].
    pub const fn is_dark(self) -> bool {
        matches!(self, PaletteMode::Dark)
    }

    /// Stable identifier (also the i18n suffix of the mode's label).
    pub const fn id(self) -> &'static str {
        match self {
            PaletteMode::Dark => "dark",
            PaletteMode::Light => "light",
        }
    }
}
