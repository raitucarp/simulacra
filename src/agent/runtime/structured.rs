//! Structured-output prompting, JSON extraction from free text and the repair turn.

use super::draft::PostDraft;
use super::transport::Transport;
use crate::agent::llm::{ChatMessage, ChatRequest, ToolChoice};
use crate::domain::engine::StructuredOutput;
use serde_json::Value;

/// Appended to the system prompt when JSON is requested in the prompt.
pub const JSON_INSTRUCTION: &str =
    "OUTPUT FORMAT: answer with only one JSON object matching this JSON schema. No prose, no code fences.";
/// Appended to the system prompt when the answer rides on the emit tool.
pub const EMIT_INSTRUCTION: &str =
    "OUTPUT FORMAT: when your post is ready, call the emit_post tool with it. Do not answer in plain text.";
/// The single repair turn after an unparseable answer.
pub const REPAIR_INSTRUCTION: &str =
    "That was not a valid JSON object matching the schema. Reply again with only the JSON object.";
/// Description of the emit tool.
pub const EMIT_DESCRIPTION: &str = "Publish the final post.";
const CODE_FENCE: &str = "```";

/// The JSON object in `text`: the whole text, a fenced block, or the outermost `{…}` span.
pub fn extract_json(text: &str) -> Option<Value> {
    let trimmed = text.trim();
    let candidates = [Some(trimmed), unfenced(trimmed), outer_braces(trimmed)];
    candidates
        .into_iter()
        .flatten()
        .find_map(|candidate| serde_json::from_str::<Value>(candidate).ok().filter(Value::is_object))
}

fn unfenced(text: &str) -> Option<&str> {
    let start = text.find(CODE_FENCE)? + CODE_FENCE.len();
    let body = &text[start..];
    let body = body.find('\n').map_or(body, |newline| &body[newline + 1..]);
    Some(body.rfind(CODE_FENCE).map_or(body, |end| &body[..end]).trim())
}

fn outer_braces(text: &str) -> Option<&str> {
    let (start, end) = (text.find('{')?, text.rfind('}')?);
    (start < end).then(|| &text[start..=end])
}

/// The schema text appended to the instruction.
pub fn instruction_with_schema(instruction: &str, structured: &StructuredOutput) -> String {
    format!("{instruction}\n{}", structured.schema.trim())
}

/// Maps a free-text answer that should hold JSON. On failure sends one repair turn; when
/// that fails too the raw text becomes the post.
pub async fn parse_or_repair(
    transport: &Transport,
    mut request: ChatRequest,
    text: String,
    structured: &StructuredOutput,
) -> Result<PostDraft, String> {
    if let Some(draft) = extract_json(&text).and_then(|json| PostDraft::from_structured(&json, &structured.mapping)) {
        return Ok(draft);
    }
    request.messages.push(ChatMessage::assistant(text.clone(), Vec::new()));
    request.messages.push(ChatMessage::user(instruction_with_schema(REPAIR_INSTRUCTION, structured)));
    if !request.tools.is_empty() {
        request.tool_choice = ToolChoice::Disabled;
    }
    let repaired = transport.send(&request).await?.text;
    let draft = extract_json(&repaired).and_then(|json| PostDraft::from_structured(&json, &structured.mapping));
    Ok(draft.unwrap_or_else(|| PostDraft::from_text(if repaired.trim().is_empty() { &text } else { &repaired })))
}
