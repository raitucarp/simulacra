//! Anthropic Messages API wire format (`tool_use` / `tool_result` blocks).

use super::super::chat::{ChatMessage, ChatReply, ChatRequest, Role, ToolCall, ToolChoice};
use super::super::LlmSettings;
use super::ErrorLabels;
use serde_json::{json, Value};

/// Anthropic Messages API endpoint.
pub(crate) const ANTHROPIC_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
/// Header carrying the API key.
pub(crate) const API_KEY_HEADER: &str = "x-api-key";
/// Header carrying the pinned API version.
pub(crate) const VERSION_HEADER: &str = "anthropic-version";
/// Pinned Anthropic API version.
pub(crate) const API_VERSION: &str = "2023-06-01";
/// Server-side web search tool type and its per-request use cap.
const WEB_SEARCH_TOOL_TYPE: &str = "web_search_20250305";
const WEB_SEARCH_TOOL_NAME: &str = "web_search";
const WEB_SEARCH_MAX_USES: u32 = 3;

pub(crate) const ANTHROPIC_ERRORS: ErrorLabels = ErrorLabels {
    network: "Anthropic request error",
    api: "Anthropic API error",
    parse: "Failed to parse Anthropic response JSON",
    empty: "Empty content from Anthropic",
};

/// The request body for `settings`' model. Structured output is requested by the runtime as
/// a forced tool, so `req.schema` is not used here.
pub(crate) fn body(settings: &LlmSettings, req: &ChatRequest) -> Value {
    let mut body = json!({
        "model": settings.model_or_default(),
        "system": req.system_text(),
        "max_tokens": settings.max_tokens,
        "temperature": settings.temperature,
        "messages": messages(&req.messages),
    });
    let mut tools: Vec<Value> = req
        .tools
        .iter()
        .map(|spec| json!({ "name": spec.name, "description": spec.description, "input_schema": spec.parameters }))
        .collect();
    if req.web_search {
        tools.push(json!({ "type": WEB_SEARCH_TOOL_TYPE, "name": WEB_SEARCH_TOOL_NAME, "max_uses": WEB_SEARCH_MAX_USES }));
    }
    if !tools.is_empty() {
        body["tools"] = Value::Array(tools);
    }
    if !req.tools.is_empty() {
        body["tool_choice"] = choice(&req.tool_choice);
    }
    body
}

fn choice(choice: &ToolChoice) -> Value {
    match choice {
        ToolChoice::Auto => json!({ "type": "auto" }),
        ToolChoice::Required => json!({ "type": "any" }),
        ToolChoice::Force(name) => json!({ "type": "tool", "name": name }),
        ToolChoice::Disabled => json!({ "type": "none" }),
    }
}

/// Non-system turns; consecutive tool results merge into one user turn.
fn messages(messages: &[ChatMessage]) -> Value {
    let mut out: Vec<Value> = Vec::new();
    for msg in messages.iter().filter(|m| m.role != Role::System) {
        match msg.role {
            Role::Tool => {
                let block = json!({ "type": "tool_result", "tool_use_id": msg.tool_call_id, "content": msg.content });
                match out.last_mut().filter(|last| is_tool_result_turn(last)) {
                    Some(last) => push_block(last, block),
                    None => out.push(json!({ "role": "user", "content": [block] })),
                }
            }
            Role::Assistant if !msg.tool_calls.is_empty() => out.push(assistant_with_calls(msg)),
            Role::Assistant => out.push(json!({ "role": "assistant", "content": msg.content })),
            _ => out.push(json!({ "role": "user", "content": msg.content })),
        }
    }
    Value::Array(out)
}

fn is_tool_result_turn(turn: &Value) -> bool {
    turn["content"][0]["type"] == "tool_result"
}

fn push_block(turn: &mut Value, block: Value) {
    if let Some(blocks) = turn["content"].as_array_mut() {
        blocks.push(block);
    }
}

fn assistant_with_calls(msg: &ChatMessage) -> Value {
    let mut blocks: Vec<Value> = Vec::new();
    if !msg.content.trim().is_empty() {
        blocks.push(json!({ "type": "text", "text": msg.content }));
    }
    for call in &msg.tool_calls {
        blocks.push(json!({ "type": "tool_use", "id": call.id, "name": call.name, "input": call.arguments }));
    }
    json!({ "role": "assistant", "content": blocks })
}

/// Text blocks joined, plus `tool_use` blocks as tool calls (server tool blocks are skipped).
pub(crate) fn parse(reply: &Value) -> ChatReply {
    let blocks = reply["content"].as_array().cloned().unwrap_or_default();
    let mut texts: Vec<&str> = Vec::new();
    let mut tool_calls = Vec::new();
    for block in &blocks {
        match block["type"].as_str() {
            Some("text") => texts.extend(block["text"].as_str()),
            Some("tool_use") => tool_calls.push(ToolCall {
                id: block["id"].as_str().unwrap_or_default().to_string(),
                name: block["name"].as_str().unwrap_or_default().to_string(),
                arguments: block["input"].clone(),
                signature: None,
            }),
            _ => {}
        }
    }
    ChatReply { text: texts.join("").trim().to_string(), tool_calls, ..ChatReply::default() }
}
