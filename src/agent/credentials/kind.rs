//! What a credential authenticates against.

use crate::agent::llm::LlmProvider;
use serde::{Deserialize, Serialize};

/// The provider or service a credential belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CredentialKind {
    #[default]
    Gemini,
    OpenAi,
    Anthropic,
    OpenRouter,
    Groq,
    /// An OpenAI-compatible endpoint (`endpoint` holds its URL).
    Custom,
    /// Brave Search API.
    Brave,
    /// Tavily Search API.
    Tavily,
    /// A bearer token for custom HTTP tools, custom requests or local image servers.
    Http,
}

impl CredentialKind {
    pub const ALL: [CredentialKind; 9] = [
        Self::Gemini,
        Self::OpenAi,
        Self::Anthropic,
        Self::OpenRouter,
        Self::Groq,
        Self::Custom,
        Self::Brave,
        Self::Tavily,
        Self::Http,
    ];

    /// Stable key (`gemini`, `open_ai`, …, `http`), also the i18n suffix.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Gemini => "gemini",
            Self::OpenAi => "open_ai",
            Self::Anthropic => "anthropic",
            Self::OpenRouter => "open_router",
            Self::Groq => "groq",
            Self::Custom => "custom",
            Self::Brave => "brave",
            Self::Tavily => "tavily",
            Self::Http => "http",
        }
    }

    /// The kind holding keys for `provider`.
    pub const fn from_provider(provider: LlmProvider) -> Self {
        match provider {
            LlmProvider::Gemini => Self::Gemini,
            LlmProvider::OpenAi => Self::OpenAi,
            LlmProvider::Anthropic => Self::Anthropic,
            LlmProvider::OpenRouter => Self::OpenRouter,
            LlmProvider::Groq => Self::Groq,
            LlmProvider::Custom => Self::Custom,
        }
    }

    /// The text provider of this kind, if it is one.
    pub const fn provider(self) -> Option<LlmProvider> {
        match self {
            Self::Gemini => Some(LlmProvider::Gemini),
            Self::OpenAi => Some(LlmProvider::OpenAi),
            Self::Anthropic => Some(LlmProvider::Anthropic),
            Self::OpenRouter => Some(LlmProvider::OpenRouter),
            Self::Groq => Some(LlmProvider::Groq),
            Self::Custom => Some(LlmProvider::Custom),
            Self::Brave | Self::Tavily | Self::Http => None,
        }
    }

    /// Display name: the provider label, or the service name.
    pub fn label(self) -> String {
        match self.provider() {
            Some(provider) => provider.to_string(),
            None => match self {
                Self::Brave => "Brave Search".to_string(),
                Self::Tavily => "Tavily".to_string(),
                _ => "HTTP".to_string(),
            },
        }
    }
}
