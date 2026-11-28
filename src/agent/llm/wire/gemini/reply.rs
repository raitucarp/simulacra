//! Gemini `generateContent` replies: text, function calls with their thought signatures,
//! and the pages Google Search grounded the answer on.

use super::super::super::chat::{ChatReply, ToolCall};
use crate::domain::post::{SourceKind, SourceRef};
use serde_json::Value;

/// Part field carrying a thought signature (validated on function calls by Gemini 3+).
pub(super) const SIGNATURE_FIELD: &str = "thoughtSignature";
/// Prefix of the ids given to calls that arrive without one (never sent back as an id).
const SYNTHETIC_ID_PREFIX: &str = "call_";

/// Whether `id` was made up by [`parse`] rather than sent by the model.
pub(super) fn is_synthetic_id(id: &str) -> bool {
    id.strip_prefix(SYNTHETIC_ID_PREFIX).is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()))
}

/// Text parts joined (thoughts skipped), `functionCall` parts as tool calls (with their
/// thought signatures) and the Google Search grounding pages as sources.
pub(crate) fn parse(reply: &Value) -> ChatReply {
    let candidate = &reply["candidates"][0];
    let parts = candidate["content"]["parts"].as_array().cloned().unwrap_or_default();
    let mut texts: Vec<&str> = Vec::new();
    let mut tool_calls = Vec::new();
    for (idx, part) in parts.iter().enumerate() {
        if let Some(call) = part.get("functionCall") {
            let id = call["id"].as_str().map_or_else(|| format!("{SYNTHETIC_ID_PREFIX}{idx}"), str::to_string);
            let name = call["name"].as_str().unwrap_or_default().to_string();
            let signature = part[SIGNATURE_FIELD].as_str().map(str::to_string);
            tool_calls.push(ToolCall { id, name, arguments: call["args"].clone(), signature });
        } else if part["thought"] != Value::Bool(true) {
            texts.extend(part["text"].as_str());
        }
    }
    ChatReply { text: texts.join("").trim().to_string(), tool_calls, sources: grounding_sources(&candidate["groundingMetadata"]) }
}

/// The web pages of `groundingMetadata.groundingChunks`, each once (the title falls back to the URL).
fn grounding_sources(metadata: &Value) -> Vec<SourceRef> {
    let chunks = metadata["groundingChunks"].as_array().map(Vec::as_slice).unwrap_or_default();
    let mut sources: Vec<SourceRef> = Vec::new();
    for web in chunks.iter().map(|chunk| &chunk["web"]) {
        let Some(uri) = web["uri"].as_str().map(str::trim).filter(|uri| !uri.is_empty()) else { continue };
        let title = web["title"].as_str().map(str::trim).filter(|title| !title.is_empty()).unwrap_or(uri);
        let source = SourceRef::new(SourceKind::Web, uri, title);
        if !sources.contains(&source) {
            sources.push(source);
        }
    }
    sources
}
