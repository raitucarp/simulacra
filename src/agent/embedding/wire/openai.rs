//! OpenAI-compatible `POST {base}/embeddings`.
//!
//! Request: `{"model": …, "input": [...]}` (plus `dimensions` for models that shorten);
//! reply: `{"data": [{"index": 0, "embedding": [...]}, ...]}`, put back in input order.

use super::super::config::EmbeddingConfig;
use super::super::task::{prefixed, TaskEncoding, TaskType};
use super::{exactly, float_array, EmbedInput, WireRequest, AUTHORIZATION_HEADER};
use serde_json::{json, Value};

/// Path below the `/v1` base.
pub const EMBEDDINGS_PATH: &str = "/embeddings";
/// Inputs per call.
pub const MAX_BATCH: usize = 96;
/// Model id fragment of the models that accept `dimensions`.
const SHORTENING_MODEL_MARKER: &str = "text-embedding-3";

/// The batch request for `inputs`.
pub fn request(config: &EmbeddingConfig, task: TaskType, inputs: &[EmbedInput]) -> WireRequest {
    let texts: Vec<String> = inputs
        .iter()
        .map(|input| match config.encoding() {
            TaskEncoding::Prefix => prefixed(task, &input.title, &input.text),
            TaskEncoding::Field | TaskEncoding::Plain => input.text.clone(),
        })
        .collect();
    let mut body = json!({ "model": config.model, "input": texts });
    if config.model.contains(SHORTENING_MODEL_MARKER) {
        body["dimensions"] = json!(config.dimensions);
    }
    let key = config.api_key.trim();
    let headers = if key.is_empty() { Vec::new() } else { vec![(AUTHORIZATION_HEADER, format!("Bearer {key}"))] };
    WireRequest { url: format!("{}{EMBEDDINGS_PATH}", config.endpoint), headers, body }
}

/// The vectors of a reply to `expected` inputs, in input order.
pub fn parse(reply: &Value, expected: usize) -> Result<Vec<Vec<f32>>, String> {
    let vectors = reply["data"].as_array().map(|items| {
        let mut indexed: Vec<(u64, Vec<f32>)> = items
            .iter()
            .enumerate()
            .filter_map(|(at, item)| Some((item["index"].as_u64().unwrap_or(at as u64), float_array(&item["embedding"])?)))
            .collect();
        indexed.sort_by_key(|(index, _)| *index);
        indexed.into_iter().map(|(_, vector)| vector).collect()
    });
    exactly(vectors, expected)
}
