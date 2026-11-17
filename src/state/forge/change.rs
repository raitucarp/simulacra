//! One field a palette would change, in a shape the "what this sets" lists can show.

use super::fields::{field_label_key, field_step, TraitField, TraitSet, TraitSetValue, TraitValue};
use super::step::{ForgeStep, FORGE_I18N_ROOT};
use crate::domain::agent_config::AgentConfig;
use crate::domain::archetype::ConfigChange;
use crate::domain::engine::{EngineConfig, ImageTrigger, OutputMode, SchemaPreset, ToolKind};
use crate::domain::portrait::AvatarMode;
use serde_json::Value;

const AVATAR_FIELD: &str = "avatar";
const ENGINE_FIELD: &str = "engine";
/// Engine parts a palette changes, each shown as its own row.
const ENGINE_PARTS: [&str; 4] = ["output", "tools", "image", "max_tool_steps"];
const ENGINE_LABEL_GROUP: &str = "palette.engine";
const ENGINE_VALUE_GROUP: &str = "palette.engine_value";
const PORTRAIT_GROUP: &str = "portrait";
const FORM_MODE_KEY: &str = "portrait.mode_form";

/// A field value as the UI should show it in a "what will change" list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeValue {
    Text(String),
    Number(i64),
    Flag(bool),
    /// A trait option; show it through `TraitValue::label_key`.
    Trait(TraitValue),
    /// Several options of a multi-select; show them through `TraitSetValue::label_keys`.
    Set(TraitSetValue),
    /// An i18n key to translate.
    Key(String),
    /// Several i18n keys, shown as a list (empty: "none").
    Keys(Vec<String>),
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

/// One field a palette would overwrite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldChange {
    /// Serde path of the field (`engine.tools` for an engine part).
    pub field: String,
    /// Step that owns the field (for grouping the preview).
    pub step: ForgeStep,
    /// i18n key of the field name.
    pub label_key: String,
    pub before: ChangeValue,
    pub after: ChangeValue,
}

fn engine_value_key(name: &str) -> String {
    format!("{FORGE_I18N_ROOT}.{ENGINE_VALUE_GROUP}.{name}")
}

/// How an engine part reads in a change row.
fn engine_part(engine: &EngineConfig, part: &str) -> ChangeValue {
    match part {
        "output" => ChangeValue::Key(engine_value_key(match &engine.output {
            OutputMode::FreeText => "free_text",
            OutputMode::JsonSchema(output) => output.preset.key(),
        })),
        "tools" => ChangeValue::Keys(engine.tools.iter().map(|tool| engine_value_key(tool.kind().key())).collect()),
        "image" => ChangeValue::Key(engine_value_key(&format!("image_{}", engine.image.trigger.key()))),
        _ => ChangeValue::Number(i64::from(engine.max_tool_steps)),
    }
}

/// One row per engine part that differs.
fn engine_rows(change: &ConfigChange, current: &AgentConfig, target: &AgentConfig) -> Vec<FieldChange> {
    let step = field_step(ENGINE_FIELD).unwrap_or(ForgeStep::Grimoire);
    ENGINE_PARTS
        .into_iter()
        .filter(|part| change.before.get(part) != change.after.get(part))
        .map(|part| FieldChange {
            field: format!("{ENGINE_FIELD}.{part}"),
            step,
            label_key: format!("{FORGE_I18N_ROOT}.{ENGINE_LABEL_GROUP}.{part}"),
            before: engine_part(&current.engine, part),
            after: engine_part(&target.engine, part),
        })
        .collect()
}

/// The avatar kind's name (a form, a portrait, an emblem or a photo).
fn avatar_value(avatar: Option<&AvatarMode>) -> ChangeValue {
    ChangeValue::Key(match avatar {
        Some(AvatarMode::Form(_)) => FORM_MODE_KEY.to_string(),
        Some(AvatarMode::Pcg(_)) => format!("{PORTRAIT_GROUP}.mode_pcg"),
        Some(AvatarMode::Emblem(_)) => format!("{PORTRAIT_GROUP}.mode_emblem"),
        Some(AvatarMode::Photo { .. }) => format!("{PORTRAIT_GROUP}.mode_photo"),
        None => engine_value_key("none"),
    })
}

fn values(change: &ConfigChange, current: &AgentConfig, target: &AgentConfig) -> (ChangeValue, ChangeValue) {
    if let Some(field) = TraitField::from_path(&change.field) {
        (ChangeValue::Trait(field.read(current)), ChangeValue::Trait(field.read(target)))
    } else if let Some(field) = TraitSet::from_path(&change.field) {
        (ChangeValue::Set(field.read(current)), ChangeValue::Set(field.read(target)))
    } else if change.field == AVATAR_FIELD {
        (avatar_value(current.avatar.as_ref()), avatar_value(target.avatar.as_ref()))
    } else {
        (ChangeValue::from_json(&change.before), ChangeValue::from_json(&change.after))
    }
}

/// The rows of one raw change (an engine change becomes one row per changed part).
pub(super) fn describe(change: ConfigChange, current: &AgentConfig, target: &AgentConfig) -> Vec<FieldChange> {
    if change.field == ENGINE_FIELD {
        return engine_rows(&change, current, target);
    }
    let (before, after) = values(&change, current, target);
    vec![FieldChange {
        step: field_step(&change.field).unwrap_or(ForgeStep::Review),
        label_key: field_label_key(&change.field),
        field: change.field,
        before,
        after,
    }]
}

/// i18n keys of every engine part label and value a change row can show.
pub fn engine_change_keys() -> Vec<String> {
    let mut keys: Vec<String> = ENGINE_PARTS.iter().map(|part| format!("{FORGE_I18N_ROOT}.{ENGINE_LABEL_GROUP}.{part}")).collect();
    let presets = SchemaPreset::ALL.map(|preset| preset.key().to_string());
    let tools = ToolKind::ALL.map(|tool| tool.key().to_string());
    let triggers = [ImageTrigger::Never, ImageTrigger::Chance { percent: 0 }, ImageTrigger::Always, ImageTrigger::WhenModelAsks]
        .map(|trigger| format!("image_{}", trigger.key()));
    let names = ["free_text", "none"].into_iter().map(str::to_string).chain(presets).chain(tools).chain(triggers);
    keys.extend(names.map(|name| engine_value_key(&name)));
    keys
}

