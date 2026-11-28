//! Google Gemini `generateContent` wire format (`functionDeclarations` / `functionCall`).

mod reply;

pub(crate) use reply::parse;

use super::super::chat::{ChatMessage, ChatRequest, Role, ToolCall, ToolChoice};
use super::super::LlmSettings;
use super::schema::gemini_schema;
use super::ErrorLabels;
use reply::{is_synthetic_id, SIGNATURE_FIELD};
use serde_json::{json, Value};

/// Base URL of the Gemini `models` collection.
pub(crate) const GEMINI_MODELS_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
/// Header carrying the API key (kept out of URLs and logs).
pub(crate) const API_KEY_HEADER: &str = "x-goog-api-key";
/// MIME type that turns on JSON output.
const JSON_MIME: &str = "application/json";
/// Output tokens allowed for thinking on top of the answer budget: Gemini counts thinking
/// against `maxOutputTokens`, so a short answer budget alone is spent before any text.
pub(crate) const THINKING_HEADROOM_TOKENS: u32 = 4096;

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
    let max_output = settings.max_tokens.saturating_add(THINKING_HEADROOM_TOKENS);
    let mut config = json!({ "temperature": settings.temperature, "maxOutputTokens": max_output });
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

/// The model's own call id, when it sent one.
fn provider_id(id: Option<&str>) -> Option<&str> {
    id.filter(|id| !id.is_empty() && !is_synthetic_id(id))
}

fn function_response(msg: &ChatMessage) -> Value {
    let mut response = json!({ "name": msg.tool_name, "response": { "content": msg.content } });
    if let Some(id) = provider_id(msg.tool_call_id.as_deref()) {
        response["id"] = json!(id);
    }
    json!({ "functionResponse": response })
}

fn model_parts(msg: &ChatMessage) -> Vec<Value> {
    let mut parts = Vec::new();
    if !msg.content.trim().is_empty() || msg.tool_calls.is_empty() {
        parts.push(json!({ "text": msg.content }));
    }
    parts.extend(msg.tool_calls.iter().map(function_call_part));
    parts
}

/// A `functionCall` part with the call's id and thought signature echoed as received.
fn function_call_part(call: &ToolCall) -> Value {
    let mut function_call = json!({ "name": call.name, "args": call.arguments });
    if let Some(id) = provider_id(Some(&call.id)) {
        function_call["id"] = json!(id);
    }
    let mut part = json!({ "functionCall": function_call });
    if let Some(signature) = &call.signature {
        part[SIGNATURE_FIELD] = json!(signature);
    }
    part
}
