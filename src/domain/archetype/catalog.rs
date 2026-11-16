//! Embedded class presets (`assets/archetypes/*.json`), parsed once on first use.

use super::ArchetypeClass;
use crate::domain::agent_config::AgentConfig;
use crate::domain::{EnneagramType, HumanGender};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Embeds `assets/archetypes/<key>.json` for every class, in `ArchetypeClass::ALL` order.
macro_rules! class_assets {
    ($($key:literal),* $(,)?) => {
        [$(include_str!(concat!("../../../assets/archetypes/", $key, ".json"))),*]
    };
}

const CLASS_ASSETS: [&str; 8] = class_assets![
    "socratic_gadfly", "rationalist_architect", "existential_poet", "stoic_sage",
    "empirical_scientist", "contrarian_trickster", "compassionate_mentor", "mystic_visionary",
];

/// A curated name that fits a class, with the pronouns that go with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NameSuggestion {
    pub name: String,
    pub gender: HumanGender,
    pub pronouns: String,
}

/// Everything a class contributes to a new thinker.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ClassPreset {
    /// Curated names, in suggestion order.
    pub names: Vec<NameSuggestion>,
    /// Enneagram types coherent with the class (the preset's own type first).
    pub enneagram_options: Vec<EnneagramType>,
    /// The full preset; fields the asset omits take `AgentConfig::default()` values.
    pub config: AgentConfig,
}

fn presets() -> &'static [ClassPreset] {
    static PRESETS: OnceLock<Vec<ClassPreset>> = OnceLock::new();
    PRESETS.get_or_init(|| {
        CLASS_ASSETS
            .iter()
            .zip(ArchetypeClass::ALL)
            .map(|(json, class)| {
                serde_json::from_str(json).unwrap_or_else(|e| {
                    // Compile-time constants covered by the archetype tests: a build defect.
                    panic!("embedded archetype `{}` is invalid: {e}", class.key())
                })
            })
            .collect()
    })
}

impl ArchetypeClass {
    fn position(self) -> usize {
        Self::ALL.iter().position(|class| *class == self).unwrap_or_default()
    }

    /// The class's full preset.
    pub fn preset(self) -> &'static ClassPreset {
        &presets()[self.position()]
    }

    /// The class's `AgentConfig` preset.
    pub fn config(self) -> AgentConfig {
        self.preset().config.clone()
    }

    /// Curated names that fit the class.
    pub fn name_suggestions(self) -> &'static [NameSuggestion] {
        &self.preset().names
    }
}
