//! Field-level comparison and transfer of a class preset onto an existing config.

use super::ArchetypeClass;
use crate::domain::agent_config::AgentConfig;
use serde_json::{Map, Value};

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

fn as_map(cfg: &AgentConfig) -> Map<String, Value> {
    match serde_json::to_value(cfg) {
        Ok(Value::Object(map)) => map,
        _ => Map::new(),
    }
}

/// Fields of `target` that differ from `current`, skipping `skip`, in field-name order.
pub fn config_changes(current: &AgentConfig, target: &AgentConfig, skip: &[&str]) -> Vec<ConfigChange> {
    let before = as_map(current);
    as_map(target)
        .into_iter()
        .filter(|(field, _)| !skip.contains(&field.as_str()))
        .filter_map(|(field, after)| {
            let old = before.get(&field).cloned().unwrap_or(Value::Null);
            (old != after).then_some(ConfigChange { field, before: old, after })
        })
        .collect()
}

impl ArchetypeClass {
    /// What [`ArchetypeClass::apply_to`] would change on `cfg`.
    pub fn changes_for(self, cfg: &AgentConfig) -> Vec<ConfigChange> {
        config_changes(cfg, &self.preset().config, &CLASS_PRESERVED_FIELDS)
    }

    /// Overwrites every non-preserved field of `cfg` with the class preset.
    pub fn apply_to(self, cfg: &mut AgentConfig) {
        let mut merged = as_map(cfg);
        for change in self.changes_for(cfg) {
            merged.insert(change.field, change.after);
        }
        // Both sides are valid configs, so the merge always deserializes.
        if let Ok(next) = serde_json::from_value(Value::Object(merged)) {
            *cfg = next;
        }
    }
}
