//! Multi-provider LLM inference engine supporting Google Gemini, OpenAI, Anthropic, OpenRouter, and Groq.

use crate::storage::Database;
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

impl fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gemini => write!(f, "Google Gemini"),
            Self::OpenAi => write!(f, "OpenAI"),
            Self::Anthropic => write!(f, "Anthropic"),
            Self::OpenRouter => write!(f, "OpenRouter"),
            Self::Groq => write!(f, "Groq"),
            Self::Custom => write!(f, "Custom Endpoint"),
        }
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

    /// Returns the provider identifier string.
    pub fn id_str(&self) -> &'static str {
        match self {
            Self::Gemini => "gemini",
            Self::OpenAi => "openai",
            Self::Anthropic => "anthropic",
            Self::OpenRouter => "openrouter",
            Self::Groq => "groq",
            Self::Custom => "custom",
        }
    }

    /// Parses a provider identifier.
    pub fn from_id(id: &str) -> Self {
        match id.to_lowercase().as_str() {
            "gemini" => Self::Gemini,
            "openai" => Self::OpenAi,
            "anthropic" => Self::Anthropic,
            "openrouter" => Self::OpenRouter,
            "groq" => Self::Groq,
            "custom" => Self::Custom,
            _ => Self::Gemini,
        }
    }

    /// Recommended default model name for the provider.
    pub fn default_model(&self) -> &'static str {
        match self {
            Self::Gemini => "gemini-1.5-flash",
            Self::OpenAi => "gpt-4o-mini",
            Self::Anthropic => "claude-3-5-sonnet-20241022",
            Self::OpenRouter => "google/gemini-flash-1.5",
            Self::Groq => "llama-3.3-70b-versatile",
            Self::Custom => "default-model",
        }
    }

    /// Popular recommended model presets.
    pub fn preset_models(&self) -> &'static [&'static str] {
        match self {
            Self::Gemini => &["gemini-1.5-flash", "gemini-1.5-pro", "gemini-2.0-flash-exp"],
            Self::OpenAi => &["gpt-4o-mini", "gpt-4o", "o3-mini"],
            Self::Anthropic => &["claude-3-5-sonnet-20241022", "claude-3-5-haiku-20241022"],
            Self::OpenRouter => &["google/gemini-flash-1.5", "anthropic/claude-3.5-sonnet", "meta-llama/llama-3.3-70b-instruct"],
            Self::Groq => &["llama-3.3-70b-versatile", "mixtral-8x7b-32768"],
            Self::Custom => &["default-model"],
        }
    }
}

/// LLM Configuration Settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    pub provider: LlmProvider,
    pub api_key: String,
    pub model: String,
    pub custom_endpoint: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            provider: LlmProvider::Gemini,
            api_key: String::new(),
            model: "gemini-1.5-flash".to_string(),
            custom_endpoint: String::new(),
            temperature: 0.7,
            max_tokens: 300,
        }
    }
}

impl LlmSettings {
    /// Loads saved LLM settings from the SQLite database.
    pub fn load_from_db(db: &Database) -> Self {
        let mut settings = Self::default();

        if let Ok(Some(provider_str)) = db.get_setting("llm_provider") {
            settings.provider = LlmProvider::from_id(&provider_str);
        }

        if let Ok(Some(key)) = db.get_setting("llm_api_key") {
            settings.api_key = key;
        }

        if let Ok(Some(model)) = db.get_setting("llm_model") {
            if !model.trim().is_empty() {
                settings.model = model;
            } else {
                settings.model = settings.provider.default_model().to_string();
            }
        }

        if let Ok(Some(endpoint)) = db.get_setting("llm_endpoint") {
            settings.custom_endpoint = endpoint;
        }

        if let Ok(Some(temp_str)) = db.get_setting("llm_temperature") {
            if let Ok(val) = temp_str.parse::<f32>() {
                settings.temperature = val;
            }
        }

        if let Ok(Some(tokens_str)) = db.get_setting("llm_max_tokens") {
            if let Ok(val) = tokens_str.parse::<u32>() {
                settings.max_tokens = val;
            }
        }

        settings
    }

    /// Persists LLM settings to the SQLite database.
    pub fn save_to_db(&self, db: &Database) -> Result<(), String> {
        db.set_setting("llm_provider", self.provider.id_str())
            .map_err(|e| e.to_string())?;
        db.set_setting("llm_api_key", &self.api_key)
            .map_err(|e| e.to_string())?;
        db.set_setting("llm_model", &self.model)
            .map_err(|e| e.to_string())?;
        db.set_setting("llm_endpoint", &self.custom_endpoint)
            .map_err(|e| e.to_string())?;
        db.set_setting("llm_temperature", &self.temperature.to_string())
            .map_err(|e| e.to_string())?;
        db.set_setting("llm_max_tokens", &self.max_tokens.to_string())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Asynchronous and synchronous HTTP client for LLM providers.
pub struct LlmClient {
    settings: LlmSettings,
    http: reqwest::Client,
}

impl LlmClient {
    /// Constructs a new client from settings.
    pub fn new(settings: LlmSettings) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { settings, http }
    }

    /// Returns whether a valid API key is configured.
    pub fn has_api_key(&self) -> bool {
        !self.settings.api_key.trim().is_empty()
    }

    /// Dispatches an inference request to the configured provider.
    pub async fn generate(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, String> {
        if !self.has_api_key() {
            return Err("API key is not configured. Please open Settings to set your LLM API Key.".to_string());
        }

        match self.settings.provider {
            LlmProvider::Gemini => self.generate_gemini(system_prompt, user_prompt).await,
            LlmProvider::OpenAi => {
                self.generate_openai_compatible("https://api.openai.com/v1/chat/completions", system_prompt, user_prompt).await
            }
            LlmProvider::OpenRouter => {
                self.generate_openai_compatible("https://openrouter.ai/api/v1/chat/completions", system_prompt, user_prompt).await
            }
            LlmProvider::Groq => {
                self.generate_openai_compatible("https://api.groq.com/openai/v1/chat/completions", system_prompt, user_prompt).await
            }
            LlmProvider::Custom => {
                let endpoint = if self.settings.custom_endpoint.trim().is_empty() {
                    "http://localhost:8000/v1/chat/completions"
                } else {
                    self.settings.custom_endpoint.as_str()
                };
                self.generate_openai_compatible(endpoint, system_prompt, user_prompt).await
            }
            LlmProvider::Anthropic => self.generate_anthropic(system_prompt, user_prompt).await,
        }
    }

    /// Google Gemini REST API implementation.
    async fn generate_gemini(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, String> {
        let model = if self.settings.model.trim().is_empty() {
            "gemini-1.5-flash"
        } else {
            self.settings.model.as_str()
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
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

        let resp = self
            .http
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Gemini network request error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("Gemini API error ({status}): {err_body}"));
        }

        let json_val: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Gemini response JSON: {e}"))?;

        let candidate_text = json_val
            .get("candidates")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("content"))
            .and_then(|cnt| cnt.get("parts"))
            .and_then(|pts| pts.get(0))
            .and_then(|p0| p0.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| "Gemini returned empty candidate text".to_string())?;

        Ok(candidate_text.trim().to_string())
    }

    /// OpenAI-compatible chat completion implementation (OpenAI, OpenRouter, Groq, Custom).
    async fn generate_openai_compatible(
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

        let resp = self
            .http
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.settings.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("OpenAI-compatible request error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("API error ({status}): {err_body}"));
        }

        let json_val: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {e}"))?;

        let text = json_val
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|cnt| cnt.as_str())
            .ok_or_else(|| "Empty choices content from provider".to_string())?;

        Ok(text.trim().to_string())
    }

    /// Anthropic Claude REST API implementation.
    async fn generate_anthropic(
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

        let resp = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.settings.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Anthropic request error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("Anthropic API error ({status}): {err_body}"));
        }

        let json_val: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Anthropic response JSON: {e}"))?;

        let text = json_val
            .get("content")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| "Empty content from Anthropic".to_string())?;

        Ok(text.trim().to_string())
    }
}
