//! The answer's shape: free text, or JSON validated by a schema and mapped onto the post.

use super::presets::SchemaPreset;
use serde::{Deserialize, Serialize};

/// Output format of the text model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "format", rename_all = "snake_case")]
pub enum OutputMode {
    /// The reply text is the post.
    #[default]
    FreeText,
    /// The model answers JSON matching `schema`; `mapping` picks the post fields.
    JsonSchema(StructuredOutput),
}

impl OutputMode {
    /// Stable key of the variant (`free_text` / `json_schema`).
    pub fn key(&self) -> &'static str {
        match self {
            Self::FreeText => "free_text",
            Self::JsonSchema(_) => "json_schema",
        }
    }
}

/// A JSON schema (as text, so the editor can hold invalid drafts) and its post mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct StructuredOutput {
    /// The preset the schema came from (`Custom` once edited by hand).
    pub preset: SchemaPreset,
    pub schema: String,
    pub mapping: OutputMapping,
    /// Whether the agent may tint its posts (sb C4); off by default, and not written while off.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub may_tint: bool,
    /// Whether the agent may react to the posts it answers (R8); off by default, and not
    /// written while off.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub may_react: bool,
    /// Whether the agent may post feelings (M5); off by default, and not written while off.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub may_feel: bool,
}

impl Default for StructuredOutput {
    fn default() -> Self {
        Self::from_preset(SchemaPreset::default())
    }
}

impl StructuredOutput {
    /// The built-in schema and mapping of `preset`.
    pub fn from_preset(preset: SchemaPreset) -> Self {
        Self { preset, schema: preset.schema().to_string(), mapping: preset.mapping(), may_tint: false, may_react: false, may_feel: false }
    }

    /// The parsed schema, when it is a JSON object.
    pub fn schema_value(&self) -> Option<serde_json::Value> {
        serde_json::from_str::<serde_json::Value>(&self.schema).ok().filter(serde_json::Value::is_object)
    }
}

/// Dotted paths from the model's JSON answer to post fields; a blank path is unmapped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OutputMapping {
    /// The post body (string).
    pub content: String,
    /// A prompt for the image engine (string).
    pub image_prompt: String,
    /// Hashtags appended to the body (array of strings, or one string).
    pub hashtags: String,
    /// The author's mood while posting (string).
    pub mood: String,
    /// Follow-up posts chained under the reply (array of strings).
    pub thread_parts: String,
    /// The post's container tint (a tint key; sb C4, read only while the agent may tint).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tint: String,
    /// Emoji reacting to the answered post (array of strings; R8, read only while the agent
    /// may react).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reactions: String,
    /// A feeling: emojis, intensity and note (an object; M5, read only while the agent may
    /// post feelings).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub feeling: String,
}

impl OutputMapping {
    /// The mapping where every field reads the same-named top-level key (the tint stays
    /// unmapped until the agent may tint, see [`StructuredOutput::with_tint_field`]).
    pub fn top_level() -> Self {
        Self {
            content: "content".into(),
            image_prompt: "image_prompt".into(),
            hashtags: "hashtags".into(),
            mood: "mood".into(),
            thread_parts: "thread_parts".into(),
            tint: String::new(),
            reactions: String::new(),
            feeling: String::new(),
        }
    }
}
