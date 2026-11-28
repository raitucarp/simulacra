//! Appearance additions beside the theme (density and text size stay in `ThemeSettings`).

use super::super::choices::ReadingWidth;
use crate::domain::packs::{is_safe_id, CLASSIC_PACK_ID, DEFAULT_PACK_ID};
use serde::{Deserialize, Serialize};

/// Reading comfort preferences and the illustration pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearancePrefs {
    /// Maximum width of post prose. Consumed by a later wave.
    pub reading_width: ReadingWidth,
    /// Replace animations with instant changes. Consumed by a later wave.
    pub reduced_motion: bool,
    /// Id of the illustration pack (WS5): `flat` (built in, the default), `classic` or an
    /// installed pack. A pack that is not available draws Classic.
    pub illustration_pack: String,
}

impl Default for AppearancePrefs {
    fn default() -> Self {
        Self { reading_width: ReadingWidth::default(), reduced_motion: false, illustration_pack: DEFAULT_PACK_ID.to_string() }
    }
}

impl AppearancePrefs {
    /// An id that cannot name a pack becomes `classic`.
    pub fn validated(self) -> Self {
        let pack = self.illustration_pack.trim();
        let illustration_pack = if is_safe_id(pack) { pack.to_string() } else { CLASSIC_PACK_ID.to_string() };
        Self { illustration_pack, ..self }
    }
}
