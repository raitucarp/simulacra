//! N11: speech to text through the configured AI engine. OpenAI, Groq and a custom
//! OpenAI-compatible server (a local transcription server, say) take the audio as a
//! multipart upload to `…/audio/transcriptions`; Gemini takes it inline in a
//! `generateContent` request. Anthropic and OpenRouter have no audio input: the app says so
//! before anything is sent.

use super::wire::{gemini, openai, ErrorLabels};
use super::{send_json, LlmClient, LlmProvider};
use base64::Engine as _;
use serde_json::json;

const TRANSCRIPTIONS_PATH: &str = "/audio/transcriptions";
const CHAT_PATH: &str = "/chat/completions";
const OPENAI_MODEL: &str = "whisper-1";
const GROQ_MODEL: &str = "whisper-large-v3-turbo";
const MULTIPART_PREFIX: &str = "multipart/form-data; boundary=";
const AUTHORIZATION_HEADER: &str = "Authorization";
const GEMINI_ASK: &str = "Transcribe this voice note word for word, in the language spoken. Answer with the transcript only.";
const TRANSCRIBE_ERRORS: ErrorLabels = ErrorLabels {
    network: "Transcription request error",
    api: "Transcription API error",
    parse: "Failed to parse transcription response",
    empty: "The transcription was empty",
};

/// One audio file to transcribe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioClip {
    pub bytes: Vec<u8>,
    pub file_name: String,
    pub mime: String,
}

/// `…/audio/transcriptions` next to an OpenAI-compatible chat URL (or a base URL).
pub fn transcription_url(endpoint: &str) -> String {
    let endpoint = endpoint.trim().trim_end_matches('/');
    if endpoint.ends_with(TRANSCRIPTIONS_PATH) {
        return endpoint.to_string();
    }
    format!("{}{TRANSCRIPTIONS_PATH}", endpoint.strip_suffix(CHAT_PATH).unwrap_or(endpoint))
}

/// The multipart form OpenAI-compatible servers take: the model and the file.
pub fn multipart_body(boundary: &str, model: &str, clip: &AudioClip) -> Vec<u8> {
    let mut body = format!("--{boundary}\r\nContent-Disposition: form-data; name=\"model\"\r\n\r\n{model}\r\n").into_bytes();
    let name = clip.file_name.replace(['"', '\r', '\n'], "_");
    body.extend(format!("--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{name}\"\r\nContent-Type: {}\r\n\r\n", clip.mime).into_bytes());
    body.extend_from_slice(&clip.bytes);
    body.extend(format!("\r\n--{boundary}--\r\n").into_bytes());
    body
}

impl LlmClient {
    /// Whether the configured engine can turn speech into text.
    pub fn can_transcribe(&self) -> bool {
        !matches!(self.provider(), LlmProvider::Anthropic | LlmProvider::OpenRouter)
    }

    /// The upload URL and model of an OpenAI-compatible engine.
    fn transcription_route(&self) -> (String, &'static str) {
        let chat = match self.settings.provider {
            LlmProvider::Groq => openai::GROQ_ENDPOINT,
            LlmProvider::Custom if !self.settings.custom_endpoint.trim().is_empty() => self.settings.custom_endpoint.trim(),
            LlmProvider::Custom => openai::CUSTOM_DEFAULT_ENDPOINT,
            _ => openai::OPENAI_ENDPOINT,
        };
        let model = if self.settings.provider == LlmProvider::Groq { GROQ_MODEL } else { OPENAI_MODEL };
        (transcription_url(self.endpoint_or(chat)), model)
    }

    /// What was said in `clip`.
    pub async fn transcribe(&self, clip: &AudioClip) -> Result<String, String> {
        if !self.has_api_key() {
            return Err("API key is not configured.".to_string());
        }
        if !self.can_transcribe() {
            return Err(format!("{} (400): this engine takes no audio", TRANSCRIBE_ERRORS.api));
        }
        let key = self.settings.api_key.trim();
        let reply = if self.settings.provider == LlmProvider::Gemini {
            let data = base64::engine::general_purpose::STANDARD.encode(&clip.bytes);
            let body = json!({ "contents": [{ "role": "user", "parts": [{ "inline_data": { "mime_type": clip.mime, "data": data } }, { "text": GEMINI_ASK }] }] });
            let url = gemini::url(self.endpoint_or(gemini::GEMINI_MODELS_URL), &self.settings);
            let reply = send_json(self.http.post(url).header(gemini::API_KEY_HEADER, key).json(&body), &TRANSCRIBE_ERRORS).await?;
            gemini::parse(&reply).text
        } else {
            let (url, model) = self.transcription_route();
            let boundary = format!("simulacra-{}", uuid::Uuid::new_v4().simple());
            let request = self.http.post(url).header(AUTHORIZATION_HEADER, format!("Bearer {key}")).header(reqwest::header::CONTENT_TYPE, format!("{MULTIPART_PREFIX}{boundary}"));
            let reply = send_json(request.body(multipart_body(&boundary, model, clip)), &TRANSCRIBE_ERRORS).await?;
            reply["text"].as_str().unwrap_or_default().to_string()
        };
        if reply.trim().is_empty() {
            return Err(TRANSCRIBE_ERRORS.empty.to_string());
        }
        Ok(reply)
    }
}
