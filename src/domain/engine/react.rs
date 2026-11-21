//! Agents that may react (R8, off by default): the structured answer gets an optional
//! `reactions` field (emoji for the post being answered), the mapping reads it, and each
//! agent reacts at most [`AGENT_REACTIONS_PER_HOUR`] times an hour.

use super::output::StructuredOutput;
use crate::domain::emoji::is_pickable;
use crate::domain::reaction::{normalize_emoji, Reaction};
use crate::domain::ProfileId;
use jiff::{SignedDuration, Timestamp};
use serde_json::{json, Value};

/// The answer field (and default mapping path) that carries the reactions.
pub const REACTIONS_FIELD: &str = "reactions";
/// Emoji kept from one answer.
pub const MAX_REACTIONS_PER_ANSWER: usize = 3;
/// Reactions one agent may give within any hour (its rate limit).
pub const AGENT_REACTIONS_PER_HOUR: usize = 6;
const RATE_WINDOW: SignedDuration = SignedDuration::from_secs(3_600);
const PROPERTIES: &str = "properties";
/// What the schema tells the model about the field.
const REACTIONS_DESCRIPTION: &str = "Optional. Up to three emoji reacting to the post you answer, only when you genuinely react to it; leave it out otherwise.";

impl StructuredOutput {
    /// The output actually requested: unchanged while the agent may not react; otherwise
    /// the mapping reads `reactions` (unless mapped by hand) and the schema offers it as an
    /// optional property (unless it already has one, or the mapping reads it elsewhere).
    pub fn with_reactions_field(&self) -> Self {
        let mut output = self.clone();
        if !self.may_react {
            return output;
        }
        if output.mapping.reactions.trim().is_empty() {
            output.mapping.reactions = REACTIONS_FIELD.to_string();
        }
        let Some(mut schema) = output.schema_value() else { return output };
        if let (true, Some(Value::Object(properties))) = (output.mapping.reactions.trim() == REACTIONS_FIELD, schema.get_mut(PROPERTIES)) {
            properties.entry(REACTIONS_FIELD).or_insert_with(reactions_property);
            output.schema = serde_json::to_string_pretty(&schema).unwrap_or(output.schema);
        }
        output
    }
}

/// The optional property: an array of at most three emoji strings.
fn reactions_property() -> Value {
    json!({ "type": "array", "items": { "type": "string" }, "maxItems": MAX_REACTIONS_PER_ANSWER, "description": REACTIONS_DESCRIPTION })
}

/// The emoji an answer gives: each one the picker offers, once, at most
/// [`MAX_REACTIONS_PER_ANSWER`]; anything else is dropped.
pub fn parse_reactions(items: &[String]) -> Vec<String> {
    let mut kept: Vec<String> = Vec::new();
    for emoji in items.iter().filter_map(|item| normalize_emoji(item)) {
        if is_pickable(&emoji) && !kept.contains(&emoji) && kept.len() < MAX_REACTIONS_PER_ANSWER {
            kept.push(emoji);
        }
    }
    kept
}

/// How many more reactions `agent` may give at `now`, given every row it gave: rows
/// updated within the last hour count against [`AGENT_REACTIONS_PER_HOUR`].
pub fn reactions_left<'a>(agent: &ProfileId, given: impl Iterator<Item = &'a Reaction>, now: Timestamp) -> usize {
    let since = now - RATE_WINDOW;
    let recent = given.filter(|row| row.profile_id == *agent && row.updated_at > since && row.updated_at <= now).count();
    AGENT_REACTIONS_PER_HOUR.saturating_sub(recent)
}

#[cfg(test)]
mod tests;
