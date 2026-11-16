//! Supported LLM providers and their static metadata.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    #[default]
    Gemini,
    OpenAi,
    Anthropic,
    OpenRouter,
    Groq,
    Custom,
}

/// Static metadata describing one provider.
pub(super) struct ProviderMeta {
    pub provider: LlmProvider,
    /// Stable identifier persisted to the settings table.
    pub id: &'static str,
    /// Human-readable display label.
    pub label: &'static str,
    /// Recommended default model.
    pub default_model: &'static str,
    /// Popular model presets offered in the settings UI.
    pub presets: &'static [&'static str],
}

/// Provider metadata, indexed by enum discriminant (same order as [`LlmProvider::ALL`]).
pub(super) const PROVIDER_META: [ProviderMeta; 6] = [
    ProviderMeta {
        provider: LlmProvider::Gemini,
        id: "gemini",
        label: "Google Gemini",
        default_model: "gemini-1.5-flash",
        presets: &["gemini-1.5-flash", "gemini-1.5-pro", "gemini-2.0-flash-exp"],
    },
    ProviderMeta {
        provider: LlmProvider::OpenAi,
        id: "openai",
        label: "OpenAI",
        default_model: "gpt-4o-mini",
        presets: &["gpt-4o-mini", "gpt-4o", "o3-mini"],
    },
    ProviderMeta {
        provider: LlmProvider::Anthropic,
        id: "anthropic",
        label: "Anthropic",
        default_model: "claude-3-5-sonnet-20241022",
        presets: &["claude-3-5-sonnet-20241022", "claude-3-5-haiku-20241022"],
    },
    ProviderMeta {
        provider: LlmProvider::OpenRouter,
        id: "openrouter",
        label: "OpenRouter",
        default_model: "google/gemini-flash-1.5",
        presets: &[
            "google/gemini-flash-1.5",
            "anthropic/claude-3.5-sonnet",
            "meta-llama/llama-3.3-70b-instruct",
        ],
    },
    ProviderMeta {
        provider: LlmProvider::Groq,
        id: "groq",
        label: "Groq",
        default_model: "llama-3.3-70b-versatile",
        presets: &["llama-3.3-70b-versatile", "mixtral-8x7b-32768"],
    },
    ProviderMeta {
        provider: LlmProvider::Custom,
        id: "custom",
        label: "Custom Endpoint",
        default_model: "default-model",
        presets: &["default-model"],
    },
];

impl fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.meta().label)
    }
}

impl LlmProvider {
    pub const ALL: [LlmProvider; 6] = [
        LlmProvider::Gemini,
        LlmProvider::OpenAi,
        LlmProvider::Anthropic,
        LlmProvider::OpenRouter,
        LlmProvider::Groq,
        LlmProvider::Custom,
    ];

    /// Static metadata row for this provider.
    pub(super) fn meta(&self) -> &'static ProviderMeta {
        &PROVIDER_META[*self as usize]
    }

    /// Returns the provider identifier string.
    pub fn id_str(&self) -> &'static str {
        self.meta().id
    }

    /// Parses a provider identifier (case-insensitive); unknown ids fall back to the default.
    pub fn from_id(id: &str) -> Self {
        let id = id.to_lowercase();
        PROVIDER_META
            .iter()
            .find(|meta| meta.id == id)
            .map_or_else(Self::default, |meta| meta.provider)
    }

    /// Recommended default model name for the provider.
    pub fn default_model(&self) -> &'static str {
        self.meta().default_model
    }

    /// Popular recommended model presets.
    pub fn preset_models(&self) -> &'static [&'static str] {
        self.meta().presets
    }
}
