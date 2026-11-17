//! Built-in structured-output schemas.

use super::output::OutputMapping;
use serde::{Deserialize, Serialize};

const POST_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "content": { "type": "string", "description": "The post text, in character." },
    "hashtags": { "type": "array", "items": { "type": "string" }, "description": "Zero to three hashtags without the # sign." }
  },
  "required": ["content"]
}"#;

const ILLUSTRATED_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "content": { "type": "string", "description": "The post text, in character." },
    "image_prompt": { "type": "string", "description": "A short visual description of an image to attach, or an empty string for none." },
    "hashtags": { "type": "array", "items": { "type": "string" } }
  },
  "required": ["content", "image_prompt"]
}"#;

const THREAD_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "content": { "type": "string", "description": "The first post of the thread." },
    "thread_parts": { "type": "array", "items": { "type": "string" }, "description": "Zero to three follow-up posts continuing the thought." },
    "hashtags": { "type": "array", "items": { "type": "string" } }
  },
  "required": ["content", "thread_parts"]
}"#;

const EXPRESSIVE_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "content": { "type": "string", "description": "The post text, in character." },
    "mood": { "type": "string", "description": "One or two words naming the author's mood." },
    "hashtags": { "type": "array", "items": { "type": "string" } }
  },
  "required": ["content", "mood"]
}"#;

/// Ready-made schemas; `Custom` marks a hand-edited schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SchemaPreset {
    /// Text plus hashtags.
    #[default]
    Post,
    /// Text plus an image prompt for the image engine.
    Illustrated,
    /// A first post plus chained follow-ups.
    Thread,
    /// Text plus the author's mood.
    Expressive,
    /// A schema written by hand.
    Custom,
}

impl SchemaPreset {
    pub const ALL: [SchemaPreset; 5] = [Self::Post, Self::Illustrated, Self::Thread, Self::Expressive, Self::Custom];

    /// Stable key (`post`, `illustrated`, `thread`, `expressive`, `custom`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Post => "post",
            Self::Illustrated => "illustrated",
            Self::Thread => "thread",
            Self::Expressive => "expressive",
            Self::Custom => "custom",
        }
    }

    /// The preset's schema text (`Custom` starts from the plain post schema).
    pub const fn schema(self) -> &'static str {
        match self {
            Self::Post | Self::Custom => POST_SCHEMA,
            Self::Illustrated => ILLUSTRATED_SCHEMA,
            Self::Thread => THREAD_SCHEMA,
            Self::Expressive => EXPRESSIVE_SCHEMA,
        }
    }

    /// Every preset maps the same-named top-level keys.
    pub fn mapping(self) -> OutputMapping {
        OutputMapping::top_level()
    }
}
