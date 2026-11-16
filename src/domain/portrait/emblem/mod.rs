//! Personality emblem: a flat, procedural badge derived deterministically from MBTI
//! (glyph), Enneagram (shape and wing marker) and temperament (hues).
//!
//! The emblem renders to an SVG *template* whose colors are theme placeholders
//! (`{{surface}}`, `{{accent}}`, …), so it follows every theme like the illustrations do.

mod glyph;
mod shape;
mod svg;

pub use glyph::EmblemGlyph;
pub use shape::EmblemShape;

use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::{ClassicTemperament, EnneagramType, MbtiType};
use serde::{Deserialize, Serialize};

/// Theme hue an emblem is painted with; each names a palette color placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmblemHue {
    Accent,
    Accent2,
    Agent,
    Success,
    Warning,
    Danger,
    Info,
}

impl EmblemHue {
    /// Palette color key filled into the template (`{{<key>}}`).
    pub const fn palette_key(self) -> &'static str {
        match self {
            Self::Accent => "accent",
            Self::Accent2 => "accent_2",
            Self::Agent => "agent",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
        }
    }

    /// (primary, secondary) hues of a temperament: warm for choleric/sanguine, cool for
    /// melancholic/phlegmatic, blended for the mixed temperaments.
    pub const fn pair_for(temperament: ClassicTemperament) -> (Self, Self) {
        match temperament {
            ClassicTemperament::Melancholic => (Self::Info, Self::Accent2),
            ClassicTemperament::Choleric => (Self::Danger, Self::Warning),
            ClassicTemperament::Sanguine => (Self::Warning, Self::Accent),
            ClassicTemperament::Phlegmatic => (Self::Success, Self::Info),
            ClassicTemperament::MelancholicCholeric => (Self::Accent2, Self::Danger),
            ClassicTemperament::PhlegmaticSanguine => (Self::Agent, Self::Success),
        }
    }
}

/// Everything needed to draw an emblem; derived, never hand-edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct EmblemSpec {
    pub shape: EmblemShape,
    pub primary: EmblemHue,
    pub secondary: EmblemHue,
    pub glyph: EmblemGlyph,
    /// Enneagram wing (1–9), drawn as an orbiting marker.
    pub wing: u8,
}

impl EmblemSpec {
    /// The emblem of a personality; identical inputs always give the identical emblem.
    pub fn from_traits(mbti: MbtiType, enneagram: EnneagramType, temperament: ClassicTemperament) -> Self {
        let (core, wing) = enneagram_digits(enneagram);
        let (primary, secondary) = EmblemHue::pair_for(temperament);
        Self { shape: EmblemShape::for_core(core), primary, secondary, glyph: EmblemGlyph::from_mbti(mbti), wing }
    }

    /// The emblem of an agent configuration.
    pub fn for_config(config: &AgentConfig) -> Self {
        Self::from_traits(config.mbti, config.enneagram, config.temperament)
    }
}

impl Default for EmblemSpec {
    fn default() -> Self {
        Self::from_traits(MbtiType::default(), EnneagramType::default(), ClassicTemperament::default())
    }
}

/// Core type and wing digits of an Enneagram code (`"5w4"` → `(5, 4)`).
fn enneagram_digits(enneagram: EnneagramType) -> (u8, u8) {
    let mut digits = enneagram.code().bytes().filter(u8::is_ascii_digit).map(|b| b - b'0');
    let core = digits.next().unwrap_or_default();
    (core, digits.next().unwrap_or(core))
}
