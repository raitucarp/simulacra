//! The appearance mode (Dark, Light or follow the system) and the preferred palette of each
//! mode, as plain values.

use crate::theme::palette::PaletteMode;
use crate::theme::registry::{DEFAULT_LIGHT_PALETTE_ID, DEFAULT_PALETTE_ID};
/// Which palette mode the app shows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AppearanceMode {
    #[default]
    Dark,
    Light,
    /// Follows the operating system's light or dark setting, live.
    System,
}

impl AppearanceMode {
    /// Every mode, in picker and toggle order.
    pub const ALL: &'static [Self] = &[Self::Dark, Self::Light, Self::System];

    /// Stable identifier (persisted, and the i18n suffix of the mode's label).
    pub const fn id(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            Self::System => "system",
        }
    }

    /// The mode stored as `id`.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|mode| mode.id() == id)
    }

    /// The mode a fixed palette mode stands for.
    pub const fn fixed(mode: PaletteMode) -> Self {
        match mode {
            PaletteMode::Dark => Self::Dark,
            PaletteMode::Light => Self::Light,
        }
    }

    /// The next mode of the title bar toggle (Dark, Light, System, then Dark again).
    pub fn next(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::System,
            Self::System => Self::Dark,
        }
    }

    /// The palette mode shown while the operating system is in `system` mode.
    pub const fn resolve(self, system: PaletteMode) -> PaletteMode {
        match self {
            Self::Dark => PaletteMode::Dark,
            Self::Light => PaletteMode::Light,
            Self::System => system,
        }
    }
}

/// The palette id used by default in `mode`.
pub const fn default_palette_for(mode: PaletteMode) -> &'static str {
    match mode {
        PaletteMode::Dark => DEFAULT_PALETTE_ID,
        PaletteMode::Light => DEFAULT_LIGHT_PALETTE_ID,
    }
}

/// The preferred palette id of each mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferredThemes {
    pub dark: String,
    pub light: String,
}

impl Default for PreferredThemes {
    fn default() -> Self {
        Self { dark: DEFAULT_PALETTE_ID.to_string(), light: DEFAULT_LIGHT_PALETTE_ID.to_string() }
    }
}

impl PreferredThemes {
    /// The preferred palette id of `mode`.
    pub fn of(&self, mode: PaletteMode) -> &str {
        match mode {
            PaletteMode::Dark => &self.dark,
            PaletteMode::Light => &self.light,
        }
    }

    /// Makes `id` the preferred palette of `mode`.
    pub fn set(&mut self, mode: PaletteMode, id: &str) {
        let slot = match mode {
            PaletteMode::Dark => &mut self.dark,
            PaletteMode::Light => &mut self.light,
        };
        *slot = id.to_string();
    }
}

