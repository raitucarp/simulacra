//! Agents that may tint their posts (sb C4, off by default): the structured answer gets an
//! optional `tint` field naming one of the container tints, and the mapping reads it.

use super::output::StructuredOutput;
use crate::domain::post::Tint;
use serde_json::{json, Value};

/// The answer field (and default mapping path) that carries the tint.
pub const TINT_FIELD: &str = "tint";
const PROPERTIES: &str = "properties";
/// What the schema tells the model about the field.
const TINT_DESCRIPTION: &str = "Optional. A colour for the post's container, only when it suits the post; leave it out otherwise.";

impl StructuredOutput {
    /// The output actually requested: unchanged while the agent may not tint; otherwise the
    /// mapping reads `tint` (unless mapped by hand) and the schema offers it as an optional
    /// property (unless it already has one, or the mapping reads the tint elsewhere).
    pub fn with_tint_field(&self) -> Self {
        let mut output = self.clone();
        if !self.may_tint {
            return output;
        }
        if output.mapping.tint.trim().is_empty() {
            output.mapping.tint = TINT_FIELD.to_string();
        }
        let Some(mut schema) = self.schema_value() else { return output };
        if let (true, Some(Value::Object(properties))) = (output.mapping.tint.trim() == TINT_FIELD, schema.get_mut(PROPERTIES)) {
            properties.entry(TINT_FIELD).or_insert_with(tint_property);
            output.schema = serde_json::to_string_pretty(&schema).unwrap_or(output.schema);
        }
        output
    }
}

/// The optional string property listing every tint key.
fn tint_property() -> Value {
    let keys: Vec<&str> = Tint::ALL.iter().map(|tint| tint.key()).collect();
    json!({ "type": "string", "enum": keys, "description": TINT_DESCRIPTION })
}

/// The tint an answer names: a known key, any case and spacing; anything else is no tint.
pub fn parse_tint(text: &str) -> Option<Tint> {
    Tint::from_key(&text.trim().to_lowercase())
}

#[cfg(test)]
mod tests;
