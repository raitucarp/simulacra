//! The eight playable thinker classes of the Agent Forge.
//!
//! Each class is the "Thinkers" palette with its key (`assets/palettes/thinkers/{key}.json`):
//! a complete `AgentConfig` preset, curated name suggestions and the Enneagram types that fit it.

mod catalog;
mod legacy;
mod transfer;

pub use catalog::{ClassPreset, NameSuggestion};
pub use transfer::{config_changes, config_changes_in, config_map, ConfigChange, CLASS_PRESERVED_FIELDS};

use crate::domain::trait_option::{TRAIT_ILLUSTRATION_EXT, TRAIT_ILLUSTRATION_ROOT};
use serde::{Deserialize, Serialize};

/// i18n namespace of class texts: `forge.class.{key}.{name|pitch|desc}`.
pub const CLASS_I18N_ROOT: &str = "forge.class";
/// Illustration directory (under the illustration root) holding one portrait per class.
pub const CLASS_ILLUSTRATION_DIR: &str = "classes";
const NAME_SUFFIX: &str = "name";
const PITCH_SUFFIX: &str = "pitch";
const DESCRIPTION_SUFFIX: &str = "desc";

/// A thinker archetype the user can start from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArchetypeClass {
    SocraticGadfly,
    RationalistArchitect,
    ExistentialPoet,
    StoicSage,
    EmpiricalScientist,
    ContrarianTrickster,
    CompassionateMentor,
    MysticVisionary,
}

use ArchetypeClass as C;

impl ArchetypeClass {
    pub const ALL: [Self; 8] = [
        C::SocraticGadfly,
        C::RationalistArchitect,
        C::ExistentialPoet,
        C::StoicSage,
        C::EmpiricalScientist,
        C::ContrarianTrickster,
        C::CompassionateMentor,
        C::MysticVisionary,
    ];

    /// Stable snake_case id; also the illustration and preset asset file stem.
    pub const fn key(self) -> &'static str {
        match self {
            C::SocraticGadfly => "socratic_gadfly",
            C::RationalistArchitect => "rationalist_architect",
            C::ExistentialPoet => "existential_poet",
            C::StoicSage => "stoic_sage",
            C::EmpiricalScientist => "empirical_scientist",
            C::ContrarianTrickster => "contrarian_trickster",
            C::CompassionateMentor => "compassionate_mentor",
            C::MysticVisionary => "mystic_visionary",
        }
    }

    /// The class with key `key`, if any.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|class| class.key() == key)
    }

    fn i18n(self, suffix: &str) -> String {
        format!("{CLASS_I18N_ROOT}.{}.{suffix}", self.key())
    }

    /// i18n key of the class name.
    pub fn label_key(self) -> String {
        self.i18n(NAME_SUFFIX)
    }

    /// i18n key of the one-line pitch.
    pub fn pitch_key(self) -> String {
        self.i18n(PITCH_SUFFIX)
    }

    /// i18n key of the longer description.
    pub fn description_key(self) -> String {
        self.i18n(DESCRIPTION_SUFFIX)
    }

    /// Asset path of the class illustration: `illustrations/classes/{key}.svg`.
    pub fn illustration(self) -> String {
        format!("{TRAIT_ILLUSTRATION_ROOT}/{CLASS_ILLUSTRATION_DIR}/{}.{TRAIT_ILLUSTRATION_EXT}", self.key())
    }
}

#[cfg(test)]
mod tests;
