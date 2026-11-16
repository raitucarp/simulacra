//! Anthropic Messages API backend.

use super::client::{ErrorLabels, LlmClient};

/// Anthropic Messages API endpoint.
const ANTHROPIC_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
/// Header carrying the API key.
const API_KEY_HEADER: &str = "x-api-key";
/// Header carrying the pinned API version.
const VERSION_HEADER: &str = "anthropic-version";
/// Pinned Anthropic API version.
const API_VERSION: &str = "2023-06-01";

const ANTHROPIC_ERRORS: ErrorLabels = ErrorLabels {
    network: "Anthropic request error",
    api: "Anthropic API error",
    parse: "Failed to parse Anthropic response JSON",
};
const ANTHROPIC_EMPTY_ERROR: &str = "Empty content from Anthropic";

impl LlmClient {
    /// Anthropic Claude REST API implementation.
    pub(super) async fn generate_anthropic(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, String> {
        let payload = serde_json::json!({
            "model": self.settings.model,
            "system": system_prompt,
            "max_tokens": self.settings.max_tokens,
            "temperature": self.settings.temperature,
            "messages": [
                { "role": "user", "content": user_prompt }
            ]
        });

        let request = self
            .http
            .post(ANTHROPIC_ENDPOINT)
            .header(API_KEY_HEADER, &self.settings.api_key)
            .header(VERSION_HEADER, API_VERSION)
            .json(&payload);
        let json_val = Self::send_json(request, &ANTHROPIC_ERRORS).await?;

        let text = json_val
            .get("content")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| ANTHROPIC_EMPTY_ERROR.to_string())?;

        Ok(text.trim().to_string())
    }
}
