//! Palettes: curated, ready-made configurations an agent can start from or borrow a step of.
//!
//! The catalog is data (`assets/palettes/**/*.json`, embedded): a **full** palette lives in
//! `assets/palettes/<category>/<key>.json` and sets a whole being (body, mind, voice, rhetoric,
//! schedule, engine); a **step** palette lives in `assets/palettes/steps/<step>/<key>.json` and
//! only touches the fields of one Agent Forge step. Each file holds a partial `AgentConfig`
//! JSON object (`config`), plus curated names for full palettes. The eight thinker classes are
//! the "Thinkers" palettes. Everything a palette sets stays editable afterwards.

mod apply;
mod catalog;
mod category;

pub use apply::{merge_json, BOOKKEEPING_FIELDS};
pub use catalog::PaletteFiles;
pub use category::{category_illustration, category_label_key, PaletteCategory, ALL_CATEGORIES_KEY, CATEGORY_ART_DIR};

use crate::domain::archetype::{ArchetypeClass, CLASS_I18N_ROOT};
use crate::domain::EnneagramType;
use crate::domain::HumanGender;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// i18n namespace of palette texts: `forge.palette.{key}.{name|pitch|desc}`.
pub const PALETTE_I18N_ROOT: &str = "forge.palette";
/// Illustration folder of full palettes (200×240, like class art).
pub const PALETTE_ART_DIR: &str = "palettes";
/// Illustration folder of step palettes (160×120 option cards).
pub const STEP_PALETTE_ART_DIR: &str = "palette_steps";
/// Catalog folder holding the step palettes, one sub-folder per step key.
pub const STEPS_DIR: &str = "steps";
const NAME_SUFFIX: &str = "name";
const PITCH_SUFFIX: &str = "pitch";
const DESCRIPTION_SUFFIX: &str = "desc";
const ART_ROOT: &str = "illustrations";
const ART_EXT: &str = "svg";

/// A curated name that fits a palette, with the pronouns that go with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NameSuggestion {
    pub name: String,
    pub gender: HumanGender,
    pub pronouns: String,
}

/// What a palette covers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteScope {
    /// A whole being, shown in the start gallery under its category.
    Full(PaletteCategory),
    /// Only the fields of the Agent Forge step with this key (`voice`, `daily_life`, `engine`…).
    Step(String),
}

/// One catalog entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    /// Stable id: the file stem, also the i18n and illustration key.
    pub key: String,
    pub scope: PaletteScope,
    /// Position within its category or step (then by key).
    pub order: u16,
    /// Curated names (full palettes), in suggestion order.
    pub names: Vec<NameSuggestion>,
    /// Enneagram types coherent with the palette (its own type first).
    pub enneagram_options: Vec<EnneagramType>,
    /// The partial `AgentConfig` JSON object the palette sets.
    pub config: Map<String, Value>,
}

impl Palette {
    /// Whether this palette sets a whole being.
    pub fn is_full(&self) -> bool {
        matches!(self.scope, PaletteScope::Full(_))
    }

    /// The gallery category of a full palette.
    pub fn category(&self) -> Option<PaletteCategory> {
        match self.scope {
            PaletteScope::Full(category) => Some(category),
            PaletteScope::Step(_) => None,
        }
    }

    /// The step key of a step palette.
    pub fn step_key(&self) -> Option<&str> {
        match &self.scope {
            PaletteScope::Step(step) => Some(step),
            PaletteScope::Full(_) => None,
        }
    }

    /// The thinker class this palette is (the eight classes keep their texts and art).
    pub fn legacy_class(&self) -> Option<ArchetypeClass> {
        ArchetypeClass::from_key(&self.key)
    }

    fn i18n(&self, suffix: &str) -> String {
        let root = if self.legacy_class().is_some() { CLASS_I18N_ROOT } else { PALETTE_I18N_ROOT };
        format!("{root}.{}.{suffix}", self.key)
    }

    /// i18n key of the palette name.
    pub fn label_key(&self) -> String {
        self.i18n(NAME_SUFFIX)
    }

    /// i18n key of the one-line pitch.
    pub fn pitch_key(&self) -> String {
        self.i18n(PITCH_SUFFIX)
    }

    /// i18n key of the longer description (full palettes).
    pub fn description_key(&self) -> String {
        self.i18n(DESCRIPTION_SUFFIX)
    }

    /// Asset path of the palette's illustration.
    pub fn illustration(&self) -> String {
        let dir = match (&self.scope, self.legacy_class()) {
            (PaletteScope::Step(_), _) => STEP_PALETTE_ART_DIR,
            (PaletteScope::Full(_), Some(_)) => crate::domain::archetype::CLASS_ILLUSTRATION_DIR,
            (PaletteScope::Full(_), None) => PALETTE_ART_DIR,
        };
        format!("{ART_ROOT}/{dir}/{}.{ART_EXT}", self.key)
    }

    /// Whether the palette sets the field with serde name `field` (top-level key).
    pub fn sets(&self, field: &str) -> bool {
        self.config.contains_key(field)
    }
}

#[cfg(test)]
mod tests;
