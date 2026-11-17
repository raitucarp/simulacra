//! Applying a palette onto a config, and listing what that would change.
//!
//! A step palette overlays its keys and nothing else. A full palette first resets every field
//! a class may set (everything but [`CLASS_PRESERVED_FIELDS`]) and then overlays its keys, so it
//! can also set a body, an avatar, active days or an engine, but only when it says so. Objects
//! merge deeply (an engine palette changes the tools without forgetting the agent's model).
//! A palette naming a `form` starts from that form's own nature and a matching form avatar.

use super::Palette;
use crate::domain::agent_config::AgentConfig;
use crate::domain::archetype::{config_changes_in, config_map, ConfigChange, CLASS_PRESERVED_FIELDS};
use crate::domain::engine::{OutputMode, SchemaPreset, StructuredOutput};
use crate::domain::portrait::{AvatarMode, FormKind, FormSpec};
use crate::domain::traits::{BodyForm, Embodiment};
use crate::domain::TraitOption;
use serde_json::{Map, Value};

/// Record-keeping fields: which palette or class a config started from, never a difference.
pub const BOOKKEEPING_FIELDS: [&str; 2] = ["archetype_class", "palette"];
const ENGINE_FIELD: &str = "engine";
const FORM_FIELD: &str = "form";
const AVATAR_FIELD: &str = "avatar";
const OUTPUT_PRESET: &str = "/output/preset";
const OUTPUT_SCHEMA: &str = "/output/schema";
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;

/// Variant tags of the internally tagged enums inside a config (output format, avatar mode,
/// text-model source, image engine, tool, image trigger).
const TAG_KEYS: [&str; 6] = ["format", "mode", "source", "engine", "tool", "when"];

/// Whether `overlay` names another variant than `base` (then it replaces `base` whole).
fn switches_variant(base: &Map<String, Value>, overlay: &Map<String, Value>) -> bool {
    TAG_KEYS.iter().any(|tag| matches!((base.get(*tag), overlay.get(*tag)), (Some(Value::String(a)), Some(Value::String(b))) if a != b))
}

/// Merges `overlay` into `base`: objects key by key (recursively), anything else replaced; an
/// object naming another enum variant replaces the old variant instead of mixing with it.
pub fn merge_json(base: &mut Value, overlay: &Value) {
    match (base, overlay) {
        (Value::Object(base), Value::Object(overlay)) if !switches_variant(base, overlay) => {
            for (key, value) in overlay {
                merge_json(base.entry(key.clone()).or_insert(Value::Null), value);
            }
        }
        (base, overlay) => *base = overlay.clone(),
    }
}

impl Palette {
    /// A stable seed for the palette's generated avatar.
    fn seed(&self) -> u64 {
        self.key.bytes().fold(FNV_OFFSET, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME))
    }

    /// The form the palette names, if any.
    fn form(&self) -> Option<BodyForm> {
        let key = self.config.get(FORM_FIELD)?;
        serde_json::from_value(key.clone()).ok()
    }

    /// The config after applying the palette to `current`.
    pub fn applied(&self, current: &AgentConfig) -> AgentConfig {
        let mut map = config_map(current);
        if self.is_full() {
            let defaults = config_map(&AgentConfig::default());
            let resettable = |field: &str| !CLASS_PRESERVED_FIELDS.contains(&field) && !BOOKKEEPING_FIELDS.contains(&field);
            map.extend(defaults.into_iter().filter(|(field, _)| resettable(field)));
        }
        let mut merged = Value::Object(map);
        if let Some(form) = self.form() {
            merge_json(&mut merged, &serde_json::to_value(Embodiment::natural(form)).unwrap_or_default());
        }
        let mut overlay = self.config.clone();
        overlay.remove(AVATAR_FIELD);
        merge_json(&mut merged, &Value::Object(overlay));
        let mut next: AgentConfig = serde_json::from_value(merged).unwrap_or_else(|_| current.clone());
        if let Some(avatar) = self.avatar_for(&next.embodiment, current.avatar.as_ref()) {
            next.avatar = Some(avatar);
        }
        self.sync_schema(&mut next);
        if self.is_full() {
            next.palette = Some(self.key.clone());
            next.archetype_class = self.legacy_class();
        }
        next
    }

    /// A form avatar for `body` with the palette's own parts, unless the palette sets no body
    /// or avatar, the body is a person, or the user chose an emblem or a photo.
    fn avatar_for(&self, body: &Embodiment, current: Option<&AvatarMode>) -> Option<AvatarMode> {
        let chosen = matches!(current, Some(AvatarMode::Emblem(_) | AvatarMode::Photo { .. }));
        if chosen || !(self.sets(FORM_FIELD) || self.sets(AVATAR_FIELD)) {
            return None;
        }
        let kind = FormKind::from_key(body.body_kind.key())?;
        let mut avatar = serde_json::to_value(AvatarMode::Form(FormSpec::random(kind, body.form.key(), self.seed()))).ok()?;
        if let Some(parts) = self.config.get(AVATAR_FIELD) {
            merge_json(&mut avatar, parts);
        }
        serde_json::from_value(avatar).ok()
    }

    /// A palette naming a schema preset (without its own schema text) gets that preset's schema.
    fn sync_schema(&self, cfg: &mut AgentConfig) {
        let engine = self.config.get(ENGINE_FIELD);
        let names_preset = engine.and_then(|e| e.pointer(OUTPUT_PRESET)).is_some();
        if !names_preset || engine.and_then(|e| e.pointer(OUTPUT_SCHEMA)).is_some() {
            return;
        }
        if let OutputMode::JsonSchema(output) = &mut cfg.engine.output {
            if output.preset != SchemaPreset::Custom {
                *output = StructuredOutput::from_preset(output.preset);
            }
        }
    }

    /// What [`Palette::apply_to`] would change on `cfg`, in field-name order.
    pub fn changes_for(&self, cfg: &AgentConfig) -> Vec<ConfigChange> {
        config_changes_in(&config_map(cfg), &config_map(&self.applied(cfg)), &BOOKKEEPING_FIELDS)
    }

    /// Applies the palette to `cfg`.
    pub fn apply_to(&self, cfg: &mut AgentConfig) {
        *cfg = self.applied(cfg);
    }
}
