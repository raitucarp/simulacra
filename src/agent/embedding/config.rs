//! The resolved embedding configuration: provider, model, endpoint, key and request limits,
//! taken from the preferences and the credential store (never stored on its own).

use super::task::{gemini_encoding, plain_encoding, TaskEncoding};
use crate::agent::credentials::{CredentialKind, CredentialStore};
use crate::agent::llm::wire::gemini::GEMINI_MODELS_URL;
use crate::agent::{LlmProvider, LlmSettings};
use crate::domain::embedding::FailureKind;
use crate::domain::preferences::{EmbeddingProvider, Preferences};
use std::fmt;
use std::time::Duration;

/// OpenAI's API base URL.
pub const OPENAI_BASE_URL: &str = "https://api.openai.com/v1";
/// Joins the parts of [`EmbeddingConfig::signature`].
const SIGNATURE_SEPARATOR: &str = "|";

/// Where vectors are computed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    /// `batchEmbedContents` of the Gemini API (key required).
    Gemini,
    /// A local server speaking `POST /api/embed` (no key).
    LocalServer,
    /// `POST /embeddings` of OpenAI or a compatible service.
    OpenAiCompatible,
    /// Offline, deterministic vectors (tests).
    Mock,
}

impl ProviderKind {
    /// Stable key, part of the content-hash signature.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Gemini => "gemini",
            Self::LocalServer => "local_server",
            Self::OpenAiCompatible => "openai_compatible",
            Self::Mock => "mock",
        }
    }
}

/// Everything one embedding request needs. `Debug` never prints the key.
#[derive(Clone, PartialEq, Eq)]
pub struct EmbeddingConfig {
    pub provider: ProviderKind,
    pub model: String,
    /// Base URL (the Gemini `models` collection, the local server root, or an `/v1` base).
    pub endpoint: String,
    pub api_key: String,
    pub dimensions: u32,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl fmt::Debug for EmbeddingConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EmbeddingConfig")
            .field("provider", &self.provider)
            .field("model", &self.model)
            .field("endpoint", &self.endpoint)
            .field("has_key", &!self.api_key.trim().is_empty())
            .field("dimensions", &self.dimensions)
            .finish()
    }
}

impl EmbeddingConfig {
    /// The configuration the preferences and stored keys describe.
    pub fn resolve(prefs: &Preferences, credentials: &CredentialStore, legacy: &LlmSettings) -> Self {
        let embeddings = &prefs.embeddings;
        let (provider, kind, default_endpoint) = match embeddings.provider {
            EmbeddingProvider::Gemini => (ProviderKind::Gemini, Some(CredentialKind::Gemini), GEMINI_MODELS_URL),
            EmbeddingProvider::LocalServer => (ProviderKind::LocalServer, None, embeddings.endpoint_or_default()),
            EmbeddingProvider::OpenAi => (ProviderKind::OpenAiCompatible, Some(CredentialKind::OpenAi), OPENAI_BASE_URL),
            EmbeddingProvider::Custom => (ProviderKind::OpenAiCompatible, Some(CredentialKind::Custom), embeddings.endpoint_or_default()),
        };
        let credential = kind.and_then(|kind| credentials.first_of(kind));
        let override_url = credential.map(|c| c.endpoint.trim()).filter(|url| !url.is_empty() && embeddings.provider != EmbeddingProvider::Custom);
        let legacy_key = (embeddings.provider == EmbeddingProvider::Gemini && legacy.provider == LlmProvider::Gemini).then(|| legacy.api_key.clone());
        let api_key = credential.map(|c| c.secret.trim().to_string()).filter(|key| !key.is_empty()).or(legacy_key).unwrap_or_default();
        Self {
            provider,
            model: embeddings.model.trim().to_string(),
            endpoint: override_url.unwrap_or(default_endpoint).trim_end_matches('/').to_string(),
            api_key,
            dimensions: embeddings.dimensions,
            timeout: Duration::from_secs(u64::from(prefs.ai.request_timeout_seconds)),
            max_retries: prefs.ai.max_retries,
        }
    }

    /// An offline configuration producing `dimensions`-long vectors.
    pub fn mock(dimensions: u32) -> Self {
        let timeout = Duration::from_secs(1);
        Self { provider: ProviderKind::Mock, model: "mock".into(), endpoint: String::new(), api_key: String::new(), dimensions, timeout, max_retries: 0 }
    }

    /// Whether the provider is ready to be called: a key is configured where one is needed.
    pub fn readiness(&self) -> Result<(), FailureKind> {
        let needs_key = matches!(self.provider, ProviderKind::Gemini) || (self.provider == ProviderKind::OpenAiCompatible && self.endpoint == OPENAI_BASE_URL);
        if needs_key && self.api_key.trim().is_empty() {
            return Err(FailureKind::MissingKey);
        }
        Ok(())
    }

    /// How the task reaches the model.
    pub fn encoding(&self) -> TaskEncoding {
        match self.provider {
            ProviderKind::Gemini => gemini_encoding(&self.model),
            ProviderKind::LocalServer | ProviderKind::OpenAiCompatible => plain_encoding(&self.model),
            ProviderKind::Mock => TaskEncoding::Plain,
        }
    }

    /// What besides the text shapes a vector: provider, dimensions and task encoding.
    pub fn signature(&self) -> String {
        let encoding = format!("{:?}", self.encoding()).to_lowercase();
        [self.provider.key(), &self.dimensions.to_string(), &encoding].join(SIGNATURE_SEPARATOR)
    }
}
