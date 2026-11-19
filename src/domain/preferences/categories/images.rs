//! Images: the default engine and size, a style prefix and automatic avatars.

use super::super::choices::{ImageAspect, ImageEngineChoice};
use serde::{Deserialize, Serialize};

/// Longest style prefix kept, in characters.
const STYLE_PREFIX_MAX_CHARS: usize = 200;

/// Image generation preferences.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ImagePrefs {
    /// Engine new agents start with. Consumed by a later wave.
    pub default_engine: ImageEngineChoice,
    /// Consumed by a later wave.
    pub size: ImageAspect,
    /// Words put before every image prompt. Consumed by a later wave.
    pub style_prefix: String,
    /// Draw an avatar for agents made without a portrait. Consumed by a later wave.
    pub auto_avatars: bool,
}

impl ImagePrefs {
    /// Trims the style prefix and caps its length.
    pub fn validated(self) -> Self {
        let style_prefix = self.style_prefix.trim().chars().take(STYLE_PREFIX_MAX_CHARS).collect();
        Self { style_prefix, ..self }
    }
}
