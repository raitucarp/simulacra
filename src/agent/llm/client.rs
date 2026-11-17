//! The HTTP client for LLM providers and its provider dispatch.

use super::chat::{ChatReply, ChatRequest};
use super::wire::{anthropic, gemini, openai, ErrorLabels};
use super::{LlmFailure, LlmProvider, LlmSettings};
use serde_json::Value;
use std::time::Duration;

/// Per-request HTTP timeout.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
/// Error returned when no API key is configured.
const MISSING_API_KEY_ERROR: &str =
    "API key is not configured. Please open Settings to set your LLM API Key.";
const AUTHORIZATION_HEADER: &str = "Authorization";

/// Asynchronous HTTP client for LLM providers.
pub struct LlmClient {
    pub(super) settings: LlmSettings,
    pub(super) http: reqwest::Client,
    /// Replaces the provider URL (full chat URL; the `models` base URL for Gemini).
    endpoint: Option<String>,
}

impl LlmClient {
    /// Constructs a new client from settings.
    pub fn new(settings: LlmSettings) -> Self {
        let http = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { settings, http, endpoint: None }
    }

    /// Sends every request to `endpoint` instead of the provider URL (blank = no override).
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        let endpoint = endpoint.trim();
        self.endpoint = (!endpoint.is_empty()).then(|| endpoint.to_string());
        self
    }

    /// Returns whether a valid API key is configured.
    pub fn has_api_key(&self) -> bool {
        self.settings.has_api_key()
    }

    /// The configured provider.
    pub fn provider(&self) -> LlmProvider {
        self.settings.provider
    }

    /// The model settings this client sends.
    pub fn settings(&self) -> &LlmSettings {
        &self.settings
    }

    /// Wraps a raw error into a display-safe [`LlmFailure`] (API key redacted, shortened).
    pub fn describe_failure(&self, raw: &str) -> LlmFailure {
        LlmFailure::new(self.settings.provider, raw, &self.settings.api_key, !self.has_api_key())
    }

    /// A single system + user exchange; the reply text.
    pub async fn generate(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        self.chat(&ChatRequest::simple(system_prompt, user_prompt)).await.map(|reply| reply.text)
    }

    /// Sends `req` in the provider's wire format; errors when the reply is empty.
    pub async fn chat(&self, req: &ChatRequest) -> Result<ChatReply, String> {
        if !self.has_api_key() {
            return Err(MISSING_API_KEY_ERROR.to_string());
        }
        let key = self.settings.api_key.trim();
        let (request, labels, parse): (_, &ErrorLabels, fn(&Value) -> ChatReply) = match self.settings.provider {
            LlmProvider::Gemini => {
                let url = gemini::url(self.endpoint_or(gemini::GEMINI_MODELS_URL), &self.settings);
                let body = gemini::body(&self.settings, req);
                (self.http.post(url).header(gemini::API_KEY_HEADER, key).json(&body), &gemini::GEMINI_ERRORS, gemini::parse)
            }
            LlmProvider::Anthropic => {
                let request = self
                    .http
                    .post(self.endpoint_or(anthropic::ANTHROPIC_ENDPOINT))
                    .header(anthropic::API_KEY_HEADER, key)
                    .header(anthropic::VERSION_HEADER, anthropic::API_VERSION)
                    .json(&anthropic::body(&self.settings, req));
                (request, &anthropic::ANTHROPIC_ERRORS, anthropic::parse)
            }
            _ => {
                let request = self.http.post(self.openai_endpoint()).header(AUTHORIZATION_HEADER, format!("Bearer {key}"));
                (request.json(&openai::body(&self.settings, req)), &openai::OPENAI_ERRORS, openai::parse)
            }
        };
        let reply = parse(&send_json(request, labels).await?);
        if reply.is_empty() {
            return Err(labels.empty.to_string());
        }
        Ok(reply)
    }

    /// The override endpoint, else `default`.
    fn endpoint_or<'a>(&'a self, default: &'a str) -> &'a str {
        self.endpoint.as_deref().unwrap_or(default)
    }

    /// The chat-completions URL of an OpenAI-compatible provider.
    fn openai_endpoint(&self) -> &str {
        let default = match self.settings.provider {
            LlmProvider::OpenRouter => openai::OPENROUTER_ENDPOINT,
            LlmProvider::Groq => openai::GROQ_ENDPOINT,
            LlmProvider::Custom if !self.settings.custom_endpoint.trim().is_empty() => self.settings.custom_endpoint.trim(),
            LlmProvider::Custom => openai::CUSTOM_DEFAULT_ENDPOINT,
            _ => openai::OPENAI_ENDPOINT,
        };
        self.endpoint_or(default)
    }
}

/// Sends a prepared request and decodes a successful JSON response body.
pub(crate) async fn send_json(request: reqwest::RequestBuilder, labels: &ErrorLabels) -> Result<Value, String> {
    let resp = request.send().await.map_err(|e| format!("{}: {e}", labels.network))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let err_body = resp.text().await.unwrap_or_default();
        return Err(format!("{} ({status}): {err_body}", labels.api));
    }
    resp.json().await.map_err(|e| format!("{}: {e}", labels.parse))
}
