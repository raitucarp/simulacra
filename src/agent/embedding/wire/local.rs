//! A local embedding server's `POST /api/embed` (it serves `embeddinggemma` locally).
//!
//! Request: `{"model": …, "input": [...], "truncate": true, "dimensions": …}`; reply:
//! `{"embeddings": [[...], ...]}`. No key is sent unless one is configured.

use super::super::config::EmbeddingConfig;
use super::super::task::{prefixed, TaskEncoding, TaskType};
use super::{exactly, float_array, EmbedInput, WireRequest, AUTHORIZATION_HEADER};
use serde_json::{json, Value};

/// Path of the batch embedding endpoint below the server root.
pub const EMBED_PATH: &str = "/api/embed";
/// Inputs per call (the server embeds them in one pass).
pub const MAX_BATCH: usize = 64;

/// The batch request for `inputs`.
pub fn request(config: &EmbeddingConfig, task: TaskType, inputs: &[EmbedInput]) -> WireRequest {
    let texts: Vec<String> = inputs
        .iter()
        .map(|input| match config.encoding() {
            TaskEncoding::Prefix => prefixed(task, &input.title, &input.text),
            TaskEncoding::Field | TaskEncoding::Plain => input.text.clone(),
        })
        .collect();
    let body = json!({ "model": config.model, "input": texts, "truncate": true, "dimensions": config.dimensions });
    let key = config.api_key.trim();
    let headers = if key.is_empty() { Vec::new() } else { vec![(AUTHORIZATION_HEADER, format!("Bearer {key}"))] };
    WireRequest { url: format!("{}{EMBED_PATH}", config.endpoint), headers, body }
}

/// The vectors of a reply to `expected` inputs, in input order.
pub fn parse(reply: &Value, expected: usize) -> Result<Vec<Vec<f32>>, String> {
    let vectors = reply["embeddings"].as_array().map(|items| items.iter().filter_map(float_array).collect());
    exactly(vectors, expected)
}
