//! Per-provider embedding requests and reply parsing. Every builder is pure, so the request
//! and reply JSON are golden-tested without a network.

pub mod gemini;
pub mod local;
pub mod openai;

use serde_json::Value;

/// Header carrying a bearer token.
pub(crate) const AUTHORIZATION_HEADER: &str = "Authorization";

/// One text to embed (`title` is used for documents, blank otherwise).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedInput {
    pub title: String,
    pub text: String,
}

impl EmbedInput {
    pub fn text(text: impl Into<String>) -> Self {
        Self { title: String::new(), text: text.into() }
    }
}

/// A prepared HTTP POST.
#[derive(Debug, Clone, PartialEq)]
pub struct WireRequest {
    pub url: String,
    /// `(name, value)`; values may hold the key and are never logged.
    pub headers: Vec<(&'static str, String)>,
    pub body: Value,
}

/// A JSON number array as `f32`s (`None` when it is not one).
pub(crate) fn float_array(value: &Value) -> Option<Vec<f32>> {
    value.as_array()?.iter().map(|x| x.as_f64().map(|x| x as f32)).collect()
}

/// `expected` vectors, else why the reply does not fit.
pub(crate) fn exactly(vectors: Option<Vec<Vec<f32>>>, expected: usize) -> Result<Vec<Vec<f32>>, String> {
    match vectors {
        Some(vectors) if vectors.len() == expected && vectors.iter().all(|v| !v.is_empty()) => Ok(vectors),
        Some(vectors) => Err(format!("expected {expected} vectors, got {}", vectors.len())),
        None => Err("the reply holds no embedding vectors".to_string()),
    }
}
