//! Google Gemini `generateContent` wire format (`functionDeclarations` / `functionCall`).

use super::super::chat::{ChatMessage, ChatReply, ChatRequest, Role, ToolCall, ToolChoice};
use super::super::LlmSettings;
use super::schema::gemini_schema;
use super::ErrorLabels;
use serde_json::{json, Value};

/// Base URL of the Gemini `models` collection.
pub(crate) const GEMINI_MODELS_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
/// Header carrying the API key (kept out of URLs and logs).
pub(crate) const API_KEY_HEADER: &str = "x-goog-api-key";
/// MIME type that turns on JSON output.
const JSON_MIME: &str = "application/json";

pub(crate) const GEMINI_ERRORS: ErrorLabels = ErrorLabels {
    network: "Gemini network request error",
    api: "Gemini API error",
    parse: "Failed to parse Gemini response JSON",
    empty: "Gemini returned empty candidate text",
};

/// `{base}/{model}:generateContent`.
pub(crate) fn url(base: &str, settings: &LlmSettings) -> String {
    format!("{}/{}:generateContent", base.trim_end_matches('/'), settings.model_or_default())
}

/// The request body.
pub(crate) fn body(settings: &LlmSettings, req: &ChatRequest) -> Value {
    let mut config = json!({ "temperature": settings.temperature, "maxOutputTokens": settings.max_tokens });
    if let Some(schema) = &req.schema {
        config["responseMimeType"] = json!(JSON_MIME);
        config["responseSchema"] = gemini_schema(schema);
    }
    let mut body = json!({
        "systemInstruction": { "parts": [{ "text": req.system_text() }] },
        "contents": contents(&req.messages),
        "generationConfig": config,
    });
    let mut tools: Vec<Value> = Vec::new();
    if !req.tools.is_empty() {
        let declarations: Vec<Value> = req
            .tools
            .iter()
            .map(|spec| json!({ "name": spec.name, "description": spec.description, "parameters": gemini_schema(&spec.parameters) }))
            .collect();
        tools.push(json!({ "functionDeclarations": declarations }));
        body["toolConfig"] = json!({ "functionCallingConfig": calling_config(&req.tool_choice) });
    }
    if req.web_search {
        tools.push(json!({ "google_search": {} }));
    }
    if !tools.is_empty() {
        body["tools"] = Value::Array(tools);
    }
    body
}

fn calling_config(choice: &ToolChoice) -> Value {
    match choice {
        ToolChoice::Auto => json!({ "mode": "AUTO" }),
        ToolChoice::Required => json!({ "mode": "ANY" }),
        ToolChoice::Force(name) => json!({ "mode": "ANY", "allowedFunctionNames": [name] }),
        ToolChoice::Disabled => json!({ "mode": "NONE" }),
    }
}

/// Non-system turns; consecutive tool results merge into one user turn.
fn contents(messages: &[ChatMessage]) -> Value {
    let mut out: Vec<Value> = Vec::new();
    for msg in messages.iter().filter(|m| m.role != Role::System) {
        let (role, parts) = match msg.role {
            Role::Tool => ("user", vec![function_response(msg)]),
            Role::Assistant => ("model", model_parts(msg)),
            _ => ("user", vec![json!({ "text": msg.content })]),
        };
        let merge = msg.role == Role::Tool && out.last().is_some_and(|last| last["parts"][0].get("functionResponse").is_some());
        match out.last_mut().filter(|_| merge).and_then(|last| last["parts"].as_array_mut()) {
            Some(existing) => existing.extend(parts),
            None => out.push(json!({ "role": role, "parts": parts })),
        }
    }
    Value::Array(out)
}

fn function_response(msg: &ChatMessage) -> Value {
    json!({ "functionResponse": { "name": msg.tool_name, "response": { "content": msg.content } } })
}

fn model_parts(msg: &ChatMessage) -> Vec<Value> {
    let mut parts = Vec::new();
    if !msg.content.trim().is_empty() || msg.tool_calls.is_empty() {
        parts.push(json!({ "text": msg.content }));
    }
    parts.extend(msg.tool_calls.iter().map(|call| json!({ "functionCall": { "name": call.name, "args": call.arguments } })));
    parts
}

/// Text parts joined (thoughts skipped) plus `functionCall` parts as tool calls.
pub(crate) fn parse(reply: &Value) -> ChatReply {
    let parts = reply["candidates"][0]["content"]["parts"].as_array().cloned().unwrap_or_default();
    let mut texts: Vec<&str> = Vec::new();
    let mut tool_calls = Vec::new();
    for (idx, part) in parts.iter().enumerate() {
        if let Some(call) = part.get("functionCall") {
            let id = call["id"].as_str().map_or_else(|| format!("call_{idx}"), str::to_string);
            let name = call["name"].as_str().unwrap_or_default().to_string();
            tool_calls.push(ToolCall { id, name, arguments: call["args"].clone() });
        } else if part["thought"] != Value::Bool(true) {
            texts.extend(part["text"].as_str());
        }
    }
    ChatReply { text: texts.join("").trim().to_string(), tool_calls }
}
