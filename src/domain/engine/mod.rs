//! Per-agent AI engine: which model answers, how the request is built, what shape the
//! answer takes, which image engine illustrates it and which tools the agent may use.
//!
//! Persisted inside `AgentConfig.engine` (JSON). Every struct is `#[serde(default)]` and the
//! whole engine deserializes leniently ([`lenient`]): a malformed engine resets only the
//! engine, never the rest of the agent. API keys never live here; agents reference
//! credential ids from the settings credential store.
//!
//! - [`text`]: the text model (global default or the agent's own).
//! - [`request`]: native provider requests or a custom raw request template.
//! - [`output`] / [`presets`]: free text or a JSON schema mapped onto the post.
//! - [`image`]: the image engine and when it fires.
//! - [`tools`]: tool grants (vault search, network reading, web, custom HTTP).
//! - [`path`]: dotted-path lookups into JSON (`choices.0.message.content`).

mod image;
mod output;
mod path;
mod presets;
mod request;
mod text;
mod tools;

#[cfg(test)]
mod tests;

pub use image::{ImageEngine, ImageEngineConfig, ImageEngineKind, ImageTrigger, SdFlavor, IMAGE_PLACEHOLDERS};
pub use output::{OutputMapping, OutputMode, StructuredOutput};
pub use path::{value_at, value_text};
pub use presets::SchemaPreset;
pub use request::{CustomRequest, HeaderPair, HttpMethod, RequestMode, API_KEY_PLACEHOLDER, REQUEST_PLACEHOLDERS};
pub use text::{OwnModel, TextModel};
pub use tools::{HttpToolDef, SearchBackend, ToolGrant, ToolKind, WebSearchConfig};

use serde::{Deserialize, Deserializer, Serialize};

/// Tool-loop rounds allowed by default before the model must answer.
pub const DEFAULT_MAX_TOOL_STEPS: u8 = 4;
/// Hard upper bound on tool-loop rounds, whatever the config says.
pub const MAX_TOOL_STEPS_LIMIT: u8 = 12;

/// The complete per-agent engine. The default reproduces the classic behaviour: the global
/// model, free text, no image and no tools.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EngineConfig {
    pub text: TextModel,
    pub request: RequestMode,
    pub output: OutputMode,
    pub image: ImageEngineConfig,
    pub tools: Vec<ToolGrant>,
    /// Tool-loop rounds before the model must answer (clamped to [`MAX_TOOL_STEPS_LIMIT`]).
    pub max_tool_steps: u8,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            text: TextModel::default(),
            request: RequestMode::default(),
            output: OutputMode::default(),
            image: ImageEngineConfig::default(),
            tools: Vec::new(),
            max_tool_steps: DEFAULT_MAX_TOOL_STEPS,
        }
    }
}

impl EngineConfig {
    /// Whether this engine behaves exactly like the classic global free-text engine.
    pub fn is_classic(&self) -> bool {
        self == &Self::default()
    }

    /// Tool-loop rounds actually allowed (`1..=MAX_TOOL_STEPS_LIMIT`).
    pub fn tool_steps(&self) -> u8 {
        self.max_tool_steps.clamp(1, MAX_TOOL_STEPS_LIMIT)
    }

    /// The structured-output settings, when the agent answers in JSON.
    pub fn structured(&self) -> Option<&StructuredOutput> {
        match &self.output {
            OutputMode::JsonSchema(structured) => Some(structured),
            OutputMode::FreeText => None,
        }
    }

    /// Every credential id this engine references (text, request, image and tools).
    pub fn credential_ids(&self) -> Vec<&str> {
        let mut ids: Vec<Option<&String>> = vec![self.text.credential(), self.image.credential.as_ref()];
        if let RequestMode::Custom(custom) = &self.request {
            ids.push(custom.credential.as_ref());
        }
        ids.extend(self.tools.iter().map(ToolGrant::credential));
        ids.into_iter().flatten().map(String::as_str).collect()
    }
}

/// Deserializes an [`EngineConfig`], falling back to the default when the JSON is malformed,
/// so a bad engine never resets the rest of `AgentConfig`.
pub fn lenient<'de, D: Deserializer<'de>>(deserializer: D) -> Result<EngineConfig, D::Error> {
    let raw = serde_json::Value::deserialize(deserializer)?;
    Ok(serde_json::from_value(raw).unwrap_or_default())
}
