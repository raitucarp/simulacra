//! Literal technical examples of the Engine: model ids, URLs, headers and reply paths. They are
//! identifiers a server reads, so they are the same in every language.

use crate::agent::llm::LlmProvider;
use crate::domain::engine::{EngineConfig, ImageEngine, TextModel};
use crate::state::forge::engine::mutate::{A1111_DEFAULT_URL, COMFY_DEFAULT_URL};

/// Endpoint overrides: a local OpenAI-compatible server, a remote one, the Gemini models base.
pub const ENDPOINTS: [&str; 4] = [
    "http://localhost:8080/v1/chat/completions",
    "http://127.0.0.1:1234/v1/chat/completions",
    "https://api.openai.com/v1/chat/completions",
    "https://generativelanguage.googleapis.com/v1beta/models",
];
/// Custom request URLs.
pub const REQUEST_URLS: [&str; 4] = [
    "http://localhost:8080/v1/chat/completions",
    "https://api.openai.com/v1/chat/completions",
    "https://api.anthropic.com/v1/messages",
    "https://openrouter.ai/api/v1/chat/completions",
];
/// Request headers (`Name: value`; `{{api_key}}` is filled from the chosen key).
pub const HEADERS: [&str; 5] = [
    "Authorization: Bearer {{api_key}}",
    "x-api-key: {{api_key}}",
    "x-goog-api-key: {{api_key}}",
    "anthropic-version: 2023-06-01",
    "Content-Type: application/json",
];
/// Where the reply text sits in common answer shapes.
pub const REPLY_PATHS: [&str; 5] =
    ["choices.0.message.content", "candidates.0.content.parts.0.text", "content.0.text", "message.content", "output_text"];
/// Where tool calls sit in common answer shapes.
pub const TOOL_CALL_PATHS: [&str; 4] =
    ["choices.0.message.tool_calls", "candidates.0.content.parts", "content", "message.tool_calls"];
/// Where the image sits in common image replies.
pub const IMAGE_REPLY_PATHS: [&str; 5] =
    ["data.0.b64_json", "data.0.url", "predictions.0.bytesBase64Encoded", "images.0", "output.0"];
/// Local Stable Diffusion servers.
pub const SD_URLS: [&str; 4] = [COMFY_DEFAULT_URL, A1111_DEFAULT_URL, "http://localhost:8188", "http://localhost:7860"];
/// Custom tool names (the model calls them by these ids).
pub const TOOL_NAMES: [&str; 5] = ["dictionary_lookup", "weather_now", "library_search", "unit_convert", "quote_finder"];
/// Custom tool URL templates (`{{arg}}` takes an argument of the call).
pub const URL_TEMPLATES: [&str; 4] = [
    "https://api.example.com/search?q={{query}}",
    "https://example.org/define/{{query}}",
    "http://localhost:8000/tools/lookup?term={{query}}",
    "https://api.example.com/v1/items/{{query}}",
];
/// Most model ids offered.
const MAX_MODELS: usize = 6;
const OPENAI_IMAGES: [&str; 2] = ["gpt-image-1", "dall-e-3"];
const GOOGLE_IMAGES: [&str; 2] = ["imagen-4.0-generate-001", "gemini-2.5-flash-image"];
const LOCAL_IMAGES: [&str; 2] = ["sd_xl_base_1.0.safetensors", "v1-5-pruned-emaonly.safetensors"];

/// The own model's provider presets (every provider's when the agent uses the global model),
/// padded with the other providers' presets so there are always a few to choose from.
pub fn models(engine: &EngineConfig) -> Vec<&'static str> {
    let lead = match &engine.text {
        TextModel::Own(own) => own.provider,
        TextModel::UseGlobal => LlmProvider::default(),
    };
    let rest = LlmProvider::ALL.iter().filter(move |provider| **provider != lead).flat_map(|provider| provider.preset_models().iter());
    unique(lead.preset_models().iter().copied().chain(rest.copied())).into_iter().take(MAX_MODELS).collect()
}

/// Image model ids, the chosen engine's first.
pub fn image_models(engine: &EngineConfig) -> Vec<&'static str> {
    let lead: &[&'static str] = match engine.image.engine {
        ImageEngine::Google => &GOOGLE_IMAGES,
        ImageEngine::LocalSd { .. } => &LOCAL_IMAGES,
        _ => &OPENAI_IMAGES,
    };
    unique(lead.iter().chain(OPENAI_IMAGES.iter()).chain(GOOGLE_IMAGES.iter()).chain(LOCAL_IMAGES.iter()).copied())
}

/// `values` in order, each once.
fn unique(values: impl Iterator<Item = &'static str>) -> Vec<&'static str> {
    values.fold(Vec::new(), |mut seen, value| {
        if !seen.contains(&value) {
            seen.push(value);
        }
        seen
    })
}
