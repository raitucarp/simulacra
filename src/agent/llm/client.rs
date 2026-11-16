//! The HTTP client for LLM providers and its provider dispatch.

use super::{openai, LlmFailure, LlmProvider, LlmSettings};
use std::time::Duration;

/// Per-request HTTP timeout.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// Error returned by [`LlmClient::generate`] when no API key is configured.
const MISSING_API_KEY_ERROR: &str =
    "API key is not configured. Please open Settings to set your LLM API Key.";

/// Asynchronous and synchronous HTTP client for LLM providers.
pub struct LlmClient {
    pub(super) settings: LlmSettings,
    pub(super) http: reqwest::Client,
}

/// Provider-specific error message prefixes for [`LlmClient::send_json`].
pub(super) struct ErrorLabels {
    /// Prefix for transport failures: `"{network}: {error}"`.
    pub network: &'static str,
    /// Prefix for non-success HTTP statuses: `"{api} ({status}): {body}"`.
    pub api: &'static str,
    /// Prefix for JSON decoding failures: `"{parse}: {error}"`.
    pub parse: &'static str,
}

impl LlmClient {
    /// Constructs a new client from settings.
    pub fn new(settings: LlmSettings) -> Self {
        let http = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { settings, http }
    }

    /// Returns whether a valid API key is configured.
    pub fn has_api_key(&self) -> bool {
        !self.settings.api_key.trim().is_empty()
    }

    /// The configured provider.
    pub fn provider(&self) -> LlmProvider {
        self.settings.provider
    }

    /// Wraps a raw error from [`Self::generate`] into a display-safe [`LlmFailure`]
    /// (API key redacted, message shortened).
    pub fn describe_failure(&self, raw: &str) -> LlmFailure {
        LlmFailure::new(self.settings.provider, raw, &self.settings.api_key, !self.has_api_key())
    }

    /// Dispatches an inference request to the configured provider.
    pub async fn generate(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, String> {
        if !self.has_api_key() {
            return Err(MISSING_API_KEY_ERROR.to_string());
        }

        let endpoint = match self.settings.provider {
            LlmProvider::Gemini => return self.generate_gemini(system_prompt, user_prompt).await,
            LlmProvider::Anthropic => {
                return self.generate_anthropic(system_prompt, user_prompt).await
            }
            LlmProvider::OpenAi => openai::OPENAI_ENDPOINT,
            LlmProvider::OpenRouter => openai::OPENROUTER_ENDPOINT,
            LlmProvider::Groq => openai::GROQ_ENDPOINT,
            LlmProvider::Custom => self.custom_endpoint(),
        };
        self.generate_openai_compatible(endpoint, system_prompt, user_prompt)
            .await
    }

    /// The configured custom endpoint, or the local default when blank.
    fn custom_endpoint(&self) -> &str {
        if self.settings.custom_endpoint.trim().is_empty() {
            openai::CUSTOM_DEFAULT_ENDPOINT
        } else {
            self.settings.custom_endpoint.as_str()
        }
    }

    /// Sends a prepared request and decodes a successful JSON response body.
    pub(super) async fn send_json(
        request: reqwest::RequestBuilder,
        labels: &ErrorLabels,
    ) -> Result<serde_json::Value, String> {
        let resp = request
            .send()
            .await
            .map_err(|e| format!("{}: {e}", labels.network))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("{} ({status}): {err_body}", labels.api));
        }

        resp.json()
            .await
            .map_err(|e| format!("{}: {e}", labels.parse))
    }
}
