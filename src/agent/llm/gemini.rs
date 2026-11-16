//! Google Gemini REST backend.

use super::client::{ErrorLabels, LlmClient};
use super::LlmProvider;

/// Base URL of the Gemini `models` collection.
const GEMINI_MODELS_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

const GEMINI_ERRORS: ErrorLabels = ErrorLabels {
    network: "Gemini network request error",
    api: "Gemini API error",
    parse: "Failed to parse Gemini response JSON",
};
const GEMINI_EMPTY_ERROR: &str = "Gemini returned empty candidate text";

impl LlmClient {
    /// Google Gemini REST API implementation.
    ///
    /// NOTE: the API key is sent as a `?key=` URL query parameter (kept as-is for now).
    pub(super) async fn generate_gemini(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, String> {
        let model = if self.settings.model.trim().is_empty() {
            LlmProvider::Gemini.default_model()
        } else {
            self.settings.model.as_str()
        };

        let url = format!(
            "{GEMINI_MODELS_URL}/{}:generateContent?key={}",
            model, self.settings.api_key
        );

        let payload = serde_json::json!({
            "systemInstruction": {
                "parts": [
                    { "text": system_prompt }
                ]
            },
            "contents": [
                {
                    "role": "user",
                    "parts": [
                        { "text": user_prompt }
                    ]
                }
            ],
            "generationConfig": {
                "temperature": self.settings.temperature,
                "maxOutputTokens": self.settings.max_tokens
            }
        });

        let request = self.http.post(&url).json(&payload);
        let json_val = Self::send_json(request, &GEMINI_ERRORS).await?;
        extract_text(&json_val)
    }
}

/// Extracts `candidates[0].content.parts[0].text`, trimmed.
fn extract_text(json_val: &serde_json::Value) -> Result<String, String> {
    let candidate_text = json_val
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c0| c0.get("content"))
        .and_then(|cnt| cnt.get("parts"))
        .and_then(|pts| pts.get(0))
        .and_then(|p0| p0.get("text"))
        .and_then(|t| t.as_str())
        .ok_or_else(|| GEMINI_EMPTY_ERROR.to_string())?;

    Ok(candidate_text.trim().to_string())
}
