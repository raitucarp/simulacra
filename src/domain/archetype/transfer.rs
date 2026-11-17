//! Field-level comparison and transfer of a class preset onto an existing config.

use super::ArchetypeClass;
use crate::domain::palette::Palette;
use crate::domain::agent_config::AgentConfig;
use serde_json::{Map, Value};

/// Serde name of the nested engine config.
const ENGINE_FIELD: &str = "engine";

/// Personal fields a class never overwrites: who the person is, where they live, which
/// languages they speak, their avatar, the advanced prompt-assembly settings, their AI
/// engine (credentials and model choices are the user's, not the class's), their body
/// (a tin can stays a tin can when it adopts a thinker's mind) and the days they post.
pub const CLASS_PRESERVED_FIELDS: [&str; 28] = [
    "gender",
    "pronouns",
    "age",
    "location_culture",
    "primary_language",
    "secondary_languages",
    "avatar",
    "is_active",
    "assembly_strategy",
    "prompt_template",
    "inject_identity_and_bio",
    "inject_psychological_profile",
    "inject_intelligence_profile",
    "inject_linguistic_stylistics",
    "inject_social_and_schedule",
    "inject_network_grounding",
    "engine",
    // --- p7 ontology ---
    "body_kind",
    "form",
    "material",
    "scale",
    "temporality",
    "perception",
    "mobility",
    "anthropomorphism",
    "relation_to_humans",
    "withdrawal",
    "active_days",
];

/// One field whose value differs between two configs (values in their JSON form).
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigChange {
    /// Serde name of the field.
    pub field: String,
    pub before: Value,
    pub after: Value,
}

/// The JSON object of `cfg`, with its engine spelled out even while it is the classic one.
pub fn config_map(cfg: &AgentConfig) -> Map<String, Value> {
    let mut map = match serde_json::to_value(cfg) {
        Ok(Value::Object(map)) => map,
        _ => Map::new(),
    };
    map.insert(ENGINE_FIELD.to_string(), serde_json::to_value(&cfg.engine).unwrap_or_default());
    map
}

/// Fields of `target` that differ from `current`, skipping `skip`, in field-name order.
pub fn config_changes_in(current: &Map<String, Value>, target: &Map<String, Value>, skip: &[&str]) -> Vec<ConfigChange> {
    target
        .iter()
        .filter(|(field, _)| !skip.contains(&field.as_str()))
        .filter_map(|(field, after)| {
            let old = current.get(field).cloned().unwrap_or(Value::Null);
            (&old != after).then(|| ConfigChange { field: field.clone(), before: old, after: after.clone() })
        })
        .collect()
}

/// Fields of `target` that differ from `current`, skipping `skip`, in field-name order.
pub fn config_changes(current: &AgentConfig, target: &AgentConfig, skip: &[&str]) -> Vec<ConfigChange> {
    config_changes_in(&config_map(current), &config_map(target), skip)
}

impl ArchetypeClass {
    /// The class's palette (the "Thinkers" palette with the class key).
    pub fn palette(self) -> Option<&'static Palette> {
        Palette::find(self.key())
    }

    /// What [`ArchetypeClass::apply_to`] would change on `cfg`.
    pub fn changes_for(self, cfg: &AgentConfig) -> Vec<ConfigChange> {
        self.palette().map(|palette| palette.changes_for(cfg)).unwrap_or_default()
    }

    /// Overwrites every non-preserved field of `cfg` with the class preset.
    pub fn apply_to(self, cfg: &mut AgentConfig) {
        if let Some(palette) = self.palette() {
            palette.apply_to(cfg);
        }
    }
}
