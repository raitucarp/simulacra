//! Appearance additions beside the theme (density and text size stay in `ThemeSettings`).

use super::super::choices::ReadingWidth;
use serde::{Deserialize, Serialize};

/// Reading comfort preferences.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearancePrefs {
    /// Maximum width of post prose. Consumed by a later wave.
    pub reading_width: ReadingWidth,
    /// Replace animations with instant changes. Consumed by a later wave.
    pub reduced_motion: bool,
}

impl AppearancePrefs {
    /// Nothing to clamp.
    pub fn validated(self) -> Self {
        self
    }
}
