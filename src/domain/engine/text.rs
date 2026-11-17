//! The text model an agent speaks through.

use crate::agent::llm::{LlmProvider, DEFAULT_MAX_TOKENS, DEFAULT_TEMPERATURE};
use serde::{Deserialize, Serialize};

/// Where the agent's text model comes from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum TextModel {
    /// The global default from Settings › AI.
    #[default]
    UseGlobal,
    /// The agent's own provider, model and sampling.
    Own(OwnModel),
}

impl TextModel {
    /// Stable key of the variant (`use_global` / `own`).
    pub fn key(&self) -> &'static str {
        match self {
            Self::UseGlobal => "use_global",
            Self::Own(_) => "own",
        }
    }

    /// The referenced credential id, if any.
    pub fn credential(&self) -> Option<&String> {
        match self {
            Self::Own(own) => own.credential.as_ref(),
            Self::UseGlobal => None,
        }
    }
}

/// An agent-specific model. `credential` is a credential-store id (never a key); when
/// `None` the first credential of the provider's kind is used. A non-blank `endpoint`
/// replaces the provider URL (full chat URL; the `models` base URL for Gemini).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OwnModel {
    pub provider: LlmProvider,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub endpoint: String,
    pub credential: Option<String>,
}

/// Temperatures are finite configuration values, so equality is total in practice.
impl Eq for OwnModel {}

/// A blank model means "the provider's default", so JSON that only names a provider works.
impl Default for OwnModel {
    fn default() -> Self {
        Self { model: String::new(), ..Self::for_provider(LlmProvider::default()) }
    }
}

impl OwnModel {
    /// The provider's default model with the default sampling settings.
    pub fn for_provider(provider: LlmProvider) -> Self {
        Self {
            provider,
            model: provider.default_model().to_string(),
            temperature: DEFAULT_TEMPERATURE,
            max_tokens: DEFAULT_MAX_TOKENS,
            endpoint: String::new(),
            credential: None,
        }
    }

    /// The model id, or the provider default when blank.
    pub fn model_or_default(&self) -> &str {
        if self.model.trim().is_empty() {
            self.provider.default_model()
        } else {
            self.model.trim()
        }
    }
}
