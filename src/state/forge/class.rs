//! Applying an archetype class to a draft, and previewing what that would change.

use super::draft::AgentDraft;
use super::fields::{field_label_key, field_step, TraitField, TraitValue};
use super::step::ForgeStep;
use crate::domain::archetype::{ArchetypeClass, ConfigChange};
use crate::domain::agent_config::AgentConfig;
use serde_json::Value;

/// A field value as the UI should show it in a "what will change" list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeValue {
    Text(String),
    Number(i64),
    Flag(bool),
    /// A trait option; show it through `TraitValue::label_key`.
    Trait(TraitValue),
}

impl ChangeValue {
    fn from_json(value: &Value) -> Self {
        match value {
            Value::String(text) => Self::Text(text.clone()),
            Value::Bool(flag) => Self::Flag(*flag),
            Value::Number(number) => Self::Number(number.as_i64().unwrap_or_default()),
            other => Self::Text(other.to_string()),
        }
    }
}

/// One field a class would overwrite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldChange {
    /// Serde path of the field.
    pub field: String,
    /// Step that owns the field (for grouping the preview).
    pub step: ForgeStep,
    /// i18n key of the field name.
    pub label_key: String,
    pub before: ChangeValue,
    pub after: ChangeValue,
}

fn describe(change: ConfigChange, current: &AgentConfig, target: &AgentConfig) -> FieldChange {
    let (before, after) = match TraitField::from_path(&change.field) {
        Some(trait_field) => (
            ChangeValue::Trait(trait_field.read(current)),
            ChangeValue::Trait(trait_field.read(target)),
        ),
        None => (ChangeValue::from_json(&change.before), ChangeValue::from_json(&change.after)),
    };
    FieldChange {
        step: field_step(&change.field).unwrap_or(ForgeStep::Review),
        label_key: field_label_key(&change.field),
        field: change.field,
        before,
        after,
    }
}

/// Every field `class` would change on the draft, grouped by step in walking order.
pub fn preview_diff(draft: &AgentDraft, class: ArchetypeClass) -> Vec<FieldChange> {
    let current = &draft.profile().config;
    let target = &class.preset().config;
    let mut changes: Vec<FieldChange> = class
        .changes_for(current)
        .into_iter()
        .map(|change| describe(change, current, target))
        .collect();
    changes.sort_by_key(|change| change.step);
    changes
}

/// Applies `class` to the draft (and records it as the draft's class) as one undo step;
/// returns whether anything changed.
pub fn apply_class(draft: &mut AgentDraft, class: ArchetypeClass) -> bool {
    draft.transform(|profile| {
        class.apply_to(&mut profile.config);
        profile.config.archetype_class = Some(class);
    })
}

/// Records which class the draft started from (`None`: from scratch or a description).
pub fn set_class(draft: &mut AgentDraft, class: Option<ArchetypeClass>) -> bool {
    draft.transform(|profile| profile.config.archetype_class = class)
}
