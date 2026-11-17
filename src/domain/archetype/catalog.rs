//! Class presets, read from the "Thinkers" palettes (`assets/palettes/thinkers/<key>.json`).

use super::ArchetypeClass;
use crate::domain::agent_config::AgentConfig;
use crate::domain::palette::Palette;
use crate::domain::EnneagramType;
use std::sync::OnceLock;

pub use crate::domain::palette::NameSuggestion;

/// Everything a class contributes to a new thinker.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassPreset {
    /// Curated names, in suggestion order.
    pub names: Vec<NameSuggestion>,
    /// Enneagram types coherent with the class (the preset's own type first).
    pub enneagram_options: Vec<EnneagramType>,
    /// The full preset; fields the palette omits take `AgentConfig::default()` values.
    pub config: AgentConfig,
}

impl ClassPreset {
    /// The preset of the thinker palette `palette` (applied to a default config).
    fn of(palette: Option<&Palette>) -> Self {
        let Some(palette) = palette else { return Self::default() };
        let mut config = palette.applied(&AgentConfig::default());
        config.palette = None;
        config.archetype_class = None;
        Self { names: palette.names.clone(), enneagram_options: palette.enneagram_options.clone(), config }
    }
}

fn presets() -> &'static [ClassPreset] {
    static PRESETS: OnceLock<Vec<ClassPreset>> = OnceLock::new();
    PRESETS.get_or_init(|| ArchetypeClass::ALL.iter().map(|class| ClassPreset::of(class.palette())).collect())
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
