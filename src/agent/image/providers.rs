//! Hosted image engines: OpenAI Images and Google Imagen / Gemini image models.

use super::save::ImageData;
use super::ImageJob;
use crate::agent::llm::send_json;
use crate::agent::llm::wire::ErrorLabels;
use serde_json::{json, Value};

const OPENAI_IMAGES_URL: &str = "https://api.openai.com/v1/images/generations";
const OPENAI_DEFAULT_MODEL: &str = "gpt-image-1";
const GOOGLE_MODELS_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const GOOGLE_DEFAULT_MODEL: &str = "imagen-3.0-generate-002";
const GOOGLE_KEY_HEADER: &str = "x-goog-api-key";
/// Model-id prefix of Imagen models (`:predict`); others use `:generateContent`.
const IMAGEN_PREFIX: &str = "imagen";

const OPENAI_IMAGE_ERRORS: ErrorLabels = ErrorLabels {
    network: "OpenAI image request error",
    api: "OpenAI image API error",
    parse: "Failed to parse OpenAI image response",
    empty: "OpenAI returned no image",
};
const GOOGLE_IMAGE_ERRORS: ErrorLabels = ErrorLabels {
    network: "Google image request error",
    api: "Google image API error",
    parse: "Failed to parse Google image response",
    empty: "Google returned no image",
};

/// The OpenAI Images request body.
pub fn openai_body(job: &ImageJob, prompt: &str) -> Value {
    json!({ "model": job.model_or(OPENAI_DEFAULT_MODEL), "prompt": prompt, "size": job.config.size, "n": 1 })
}

/// Generates with the OpenAI Images API (`b64_json`, or a `url` to download).
pub async fn openai(http: &reqwest::Client, job: &ImageJob, prompt: &str) -> Result<ImageData, String> {
    let url = job.endpoint_or(OPENAI_IMAGES_URL);
    let request = http.post(url).bearer_auth(job.secret.trim()).json(&openai_body(job, prompt));
    let reply = send_json(request, &OPENAI_IMAGE_ERRORS).await?;
    let first = &reply["data"][0];
    first["b64_json"]
        .as_str()
        .map(|data| ImageData::Base64(data.to_string()))
        .or_else(|| first["url"].as_str().map(|url| ImageData::Url(url.to_string())))
        .ok_or_else(|| OPENAI_IMAGE_ERRORS.empty.to_string())
}

/// `(url, body)` of a Google image request for the job's model.
pub fn google_request(job: &ImageJob, prompt: &str) -> (String, Value) {
    let model = job.model_or(GOOGLE_DEFAULT_MODEL);
    let base = job.endpoint_or(GOOGLE_MODELS_URL).trim_end_matches('/').to_string();
    if model.starts_with(IMAGEN_PREFIX) {
        let body = json!({ "instances": [{ "prompt": prompt }], "parameters": { "sampleCount": 1 } });
        (format!("{base}/{model}:predict"), body)
    } else {
        let body = json!({
            "contents": [{ "role": "user", "parts": [{ "text": prompt }] }],
            "generationConfig": { "responseModalities": ["TEXT", "IMAGE"] }
        });
        (format!("{base}/{model}:generateContent"), body)
    }
}

/// Generates with Imagen (`predictions[0].bytesBase64Encoded`) or a Gemini image model
/// (the first `inlineData` part).
pub async fn google(http: &reqwest::Client, job: &ImageJob, prompt: &str) -> Result<ImageData, String> {
    let (url, body) = google_request(job, prompt);
    let request = http.post(url).header(GOOGLE_KEY_HEADER, job.secret.trim()).json(&body);
    let reply = send_json(request, &GOOGLE_IMAGE_ERRORS).await?;
    let predicted = reply["predictions"][0]["bytesBase64Encoded"].as_str();
    let parts = reply["candidates"][0]["content"]["parts"].as_array().cloned().unwrap_or_default();
    let inline = parts.iter().find_map(|part| part["inlineData"]["data"].as_str().map(str::to_string));
    predicted
        .map(str::to_string)
        .or(inline)
        .map(ImageData::Base64)
        .ok_or_else(|| GOOGLE_IMAGE_ERRORS.empty.to_string())
}
