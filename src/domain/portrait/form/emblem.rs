//! An emblem derived from a form (kind → silhouette and hues, form → glyph and wing), for
//! beings without an MBTI-driven personality badge.

use super::kind::FormKind;
use super::spec::FormSpec;
use crate::domain::portrait::{EmblemGlyph, EmblemHue, EmblemShape, EmblemSpec};

/// FNV-1a offset basis and prime (a stable, platform-independent string hash).
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;
/// Wing positions on the emblem dial (1–9).
const WING_SLOTS: u64 = 9;

/// Silhouette and (primary, secondary) hues of each kind.
const fn kind_look(kind: FormKind) -> (EmblemShape, EmblemHue, EmblemHue) {
    match kind {
        FormKind::Creature => (EmblemShape::Circle, EmblemHue::Warning, EmblemHue::Danger),
        FormKind::Plant => (EmblemShape::Burst, EmblemHue::Success, EmblemHue::Warning),
        FormKind::Object => (EmblemShape::Square, EmblemHue::Info, EmblemHue::Accent),
        FormKind::Element => (EmblemShape::Diamond, EmblemHue::Accent2, EmblemHue::Info),
        FormKind::Machine => (EmblemShape::Hexagon, EmblemHue::Accent, EmblemHue::Success),
        FormKind::Place => (EmblemShape::Shield, EmblemHue::Agent, EmblemHue::Warning),
        FormKind::Abstraction => (EmblemShape::Star, EmblemHue::Accent2, EmblemHue::Agent),
    }
}

fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(FNV_OFFSET, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME))
}

impl EmblemSpec {
    /// The emblem of a `form` of `kind`; identical inputs always give the identical emblem.
    pub fn from_form(kind: FormKind, form: &str) -> Self {
        let (shape, primary, secondary) = kind_look(kind);
        let bits = fnv1a(form);
        let bit = |i: u32| bits >> i & 1 == 1;
        let glyph = EmblemGlyph { outward: bit(0), intuitive: bit(1), thinking: bit(2), judging: bit(3) };
        Self { shape, primary, secondary, glyph, wing: (bits % WING_SLOTS + 1) as u8 }
    }

    /// The emblem of a form spec.
    pub fn for_form(spec: &FormSpec) -> Self {
        Self::from_form(spec.kind, spec.form())
    }
}
