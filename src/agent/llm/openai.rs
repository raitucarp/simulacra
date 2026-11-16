//! OpenAI-compatible chat-completions backend (OpenAI, OpenRouter, Groq, Custom).

use super::client::{ErrorLabels, LlmClient};

/// OpenAI chat-completions endpoint.
pub(super) const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
/// OpenRouter chat-completions endpoint.
pub(super) const OPENROUTER_ENDPOINT: &str = "https://openrouter.ai/api/v1/chat/completions";
/// Groq chat-completions endpoint.
pub(super) const GROQ_ENDPOINT: &str = "https://api.groq.com/openai/v1/chat/completions";
/// Endpoint used by the Custom provider when none is configured.
pub(super) const CUSTOM_DEFAULT_ENDPOINT: &str = "http://localhost:8000/v1/chat/completions";

const OPENAI_ERRORS: ErrorLabels = ErrorLabels {
    network: "OpenAI-compatible request error",
    api: "API error",
    parse: "Failed to parse response JSON",
};
const OPENAI_EMPTY_ERROR: &str = "Empty choices content from provider";

impl LlmClient {
    /// OpenAI-compatible chat completion implementation (OpenAI, OpenRouter, Groq, Custom).
    pub(super) async fn generate_openai_compatible(
        &self,
        endpoint: &str,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, String> {
        let payload = serde_json::json!({
            "model": self.settings.model,
            "temperature": self.settings.temperature,
            "max_tokens": self.settings.max_tokens,
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_prompt }
            ]
        });

        let request = self
            .http
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.settings.api_key))
            .json(&payload);
        let json_val = Self::send_json(request, &OPENAI_ERRORS).await?;

        let text = json_val
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|cnt| cnt.as_str())
            .ok_or_else(|| OPENAI_EMPTY_ERROR.to_string())?;

        Ok(text.trim().to_string())
    }
}
