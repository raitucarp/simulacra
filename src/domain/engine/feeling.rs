//! Agents that may post feelings (M5, off by default): the structured answer gets an optional
//! `feeling` field (one to three emojis, an intensity from 1 to 5 and a short note), the
//! mapping reads it, and an answer that gives one is posted as a feeling.

use super::output::StructuredOutput;
use crate::domain::emoji::is_pickable;
use crate::domain::post::{Intensity, PostKind, MAX_FEELING_EMOJIS, MAX_FEELING_NOTE_CHARS};
use crate::domain::reaction::normalize_emoji;
use serde_json::{json, Value};

/// The answer field (and default mapping path) that carries the feeling.
pub const FEELING_FIELD: &str = "feeling";
const PROPERTIES: &str = "properties";
/// What the schema tells the model about the field.
const FEELING_DESCRIPTION: &str = "Optional. How you feel right now, only when the post is about a feeling: one to three emoji, an intensity from 1 (faint) to 5 (overwhelming) and a short note. Leave it out otherwise.";

impl StructuredOutput {
    /// The output actually requested: unchanged while the agent may not post feelings;
    /// otherwise the mapping reads `feeling` (unless mapped by hand) and the schema offers it
    /// as an optional property (unless it already has one, or the mapping reads it elsewhere).
    pub fn with_feeling_field(&self) -> Self {
        let mut output = self.clone();
        if !self.may_feel {
            return output;
        }
        if output.mapping.feeling.trim().is_empty() {
            output.mapping.feeling = FEELING_FIELD.to_string();
        }
        let Some(mut schema) = output.schema_value() else { return output };
        if let (true, Some(Value::Object(properties))) = (output.mapping.feeling.trim() == FEELING_FIELD, schema.get_mut(PROPERTIES)) {
            properties.entry(FEELING_FIELD).or_insert_with(feeling_property);
            output.schema = serde_json::to_string_pretty(&schema).unwrap_or(output.schema);
        }
        output
    }
}

/// The optional object property.
fn feeling_property() -> Value {
    let emojis = json!({ "type": "array", "items": { "type": "string" }, "minItems": 1, "maxItems": MAX_FEELING_EMOJIS });
    let intensity = json!({ "type": "integer", "minimum": Intensity::MIN.level(), "maximum": Intensity::MAX.level() });
    let note = json!({ "type": "string", "maxLength": MAX_FEELING_NOTE_CHARS });
    json!({ "type": "object", "properties": { "emojis": emojis, "intensity": intensity, "note": note }, "required": ["emojis"], "description": FEELING_DESCRIPTION })
}

/// The feeling an answer gives: its emojis the picker offers (at most three), its intensity
/// clamped to the scale (the middle when missing), its note cut to length; `None` without a
/// usable emoji.
pub fn parse_feeling(value: &Value) -> Option<PostKind> {
    let emojis: Vec<String> = value.get("emojis")?.as_array()?.iter().filter_map(Value::as_str).filter_map(normalize_emoji).filter(|emoji| is_pickable(emoji)).take(MAX_FEELING_EMOJIS).collect();
    let intensity = value.get("intensity").and_then(Value::as_u64).map_or_else(Intensity::default, |level| Intensity::new(level.min(u64::from(u8::MAX)) as u8));
    let note: String = value.get("note").and_then(Value::as_str).unwrap_or_default().trim().chars().take(MAX_FEELING_NOTE_CHARS).collect();
    PostKind::feeling(&emojis, intensity, &note).ok()
}

#[cfg(test)]
mod tests;
