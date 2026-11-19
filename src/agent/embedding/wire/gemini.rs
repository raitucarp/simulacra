//! Gemini API `models/{model}:batchEmbedContents`.
//!
//! Request: `{"requests": [{"model": "models/{model}", "content": {"parts": [{"text": …}]},
//! "taskType": …, "title": …, "outputDimensionality": …}]}`; reply:
//! `{"embeddings": [{"values": [...]}]}`. The key travels in the `x-goog-api-key` header.

use super::super::config::EmbeddingConfig;
use super::super::task::{prefixed, TaskEncoding, TaskType};
use super::{exactly, float_array, EmbedInput, WireRequest};
use serde_json::{json, Value};

/// Header carrying the API key (kept out of URLs and logs).
pub const API_KEY_HEADER: &str = "x-goog-api-key";
/// Most requests one `batchEmbedContents` call accepts.
pub const MAX_BATCH: usize = 100;
/// Prefix of a model resource name.
const MODEL_RESOURCE_PREFIX: &str = "models/";

/// The batch request for `inputs`.
pub fn request(config: &EmbeddingConfig, task: TaskType, inputs: &[EmbedInput]) -> WireRequest {
    let model = config.model.trim_start_matches(MODEL_RESOURCE_PREFIX);
    let url = format!("{}/{model}:batchEmbedContents", config.endpoint);
    let requests: Vec<Value> = inputs.iter().map(|input| entry(config, model, task, input)).collect();
    WireRequest { url, headers: vec![(API_KEY_HEADER, config.api_key.trim().to_string())], body: json!({ "requests": requests }) }
}

fn entry(config: &EmbeddingConfig, model: &str, task: TaskType, input: &EmbedInput) -> Value {
    let encoding = config.encoding();
    let text = match encoding {
        TaskEncoding::Prefix => prefixed(task, &input.title, &input.text),
        TaskEncoding::Field | TaskEncoding::Plain => input.text.clone(),
    };
    let mut entry = json!({
        "model": format!("{MODEL_RESOURCE_PREFIX}{model}"),
        "content": { "parts": [{ "text": text }] },
        "outputDimensionality": config.dimensions,
    });
    if encoding == TaskEncoding::Field {
        entry["taskType"] = json!(task.wire_name());
        if task == TaskType::RetrievalDocument && !input.title.trim().is_empty() {
            entry["title"] = json!(input.title.trim());
        }
    }
    entry
}

/// The vectors of a reply to `expected` inputs, in input order.
pub fn parse(reply: &Value, expected: usize) -> Result<Vec<Vec<f32>>, String> {
    let vectors = reply["embeddings"].as_array().map(|items| items.iter().filter_map(|item| float_array(&item["values"])).collect());
    exactly(vectors, expected)
}
