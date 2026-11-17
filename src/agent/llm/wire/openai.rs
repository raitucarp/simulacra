//! OpenAI-compatible chat-completions wire format (OpenAI, OpenRouter, Groq, Custom).

use super::super::chat::{parse_arguments, ChatMessage, ChatReply, ChatRequest, Role, ToolCall, ToolChoice, ToolSpec};
use super::super::{LlmProvider, LlmSettings};
use super::{ErrorLabels, STRUCTURED_OUTPUT_NAME};
use serde_json::{json, Value};

/// OpenAI chat-completions endpoint.
pub(crate) const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
/// OpenRouter chat-completions endpoint.
pub(crate) const OPENROUTER_ENDPOINT: &str = "https://openrouter.ai/api/v1/chat/completions";
/// Groq chat-completions endpoint.
pub(crate) const GROQ_ENDPOINT: &str = "https://api.groq.com/openai/v1/chat/completions";
/// Endpoint used by the Custom provider when none is configured.
pub(crate) const CUSTOM_DEFAULT_ENDPOINT: &str = "http://localhost:8000/v1/chat/completions";
/// OpenRouter's web-search plugin id.
const OPENROUTER_WEB_PLUGIN: &str = "web";

pub(crate) const OPENAI_ERRORS: ErrorLabels = ErrorLabels {
    network: "OpenAI-compatible request error",
    api: "API error",
    parse: "Failed to parse response JSON",
    empty: "Empty choices content from provider",
};

/// The request body for `settings`' model.
pub(crate) fn body(settings: &LlmSettings, req: &ChatRequest) -> Value {
    let mut body = json!({
        "model": settings.model_or_default(),
        "temperature": settings.temperature,
        "max_tokens": settings.max_tokens,
        "messages": messages(&req.messages),
    });
    if !req.tools.is_empty() {
        body["tools"] = tools(&req.tools);
        body["tool_choice"] = choice(&req.tool_choice);
    }
    if let Some(schema) = &req.schema {
        body["response_format"] =
            json!({ "type": "json_schema", "json_schema": { "name": STRUCTURED_OUTPUT_NAME, "schema": schema } });
    }
    if req.web_search {
        match settings.provider {
            LlmProvider::OpenRouter => body["plugins"] = json!([{ "id": OPENROUTER_WEB_PLUGIN }]),
            _ => body["web_search_options"] = json!({}),
        }
    }
    body
}

/// Chat messages in OpenAI form (also the `{{messages}}` value of custom requests).
pub(crate) fn messages(messages: &[ChatMessage]) -> Value {
    Value::Array(messages.iter().map(message).collect())
}

fn message(msg: &ChatMessage) -> Value {
    match msg.role {
        Role::System => json!({ "role": "system", "content": msg.content }),
        Role::User => json!({ "role": "user", "content": msg.content }),
        Role::Tool => json!({ "role": "tool", "tool_call_id": msg.tool_call_id, "content": msg.content }),
        Role::Assistant if msg.tool_calls.is_empty() => json!({ "role": "assistant", "content": msg.content }),
        Role::Assistant => {
            let calls: Vec<Value> = msg.tool_calls.iter().map(call).collect();
            let content = if msg.content.is_empty() { Value::Null } else { Value::String(msg.content.clone()) };
            json!({ "role": "assistant", "content": content, "tool_calls": calls })
        }
    }
}

fn call(call: &ToolCall) -> Value {
    let arguments = call.arguments.to_string();
    json!({ "id": call.id, "type": "function", "function": { "name": call.name, "arguments": arguments } })
}

/// Tool specs in OpenAI form (also the `{{tools}}` value of custom requests).
pub(crate) fn tools(specs: &[ToolSpec]) -> Value {
    let specs = specs.iter().map(|spec| {
        json!({ "type": "function", "function": {
            "name": spec.name, "description": spec.description, "parameters": spec.parameters } })
    });
    Value::Array(specs.collect())
}

fn choice(choice: &ToolChoice) -> Value {
    match choice {
        ToolChoice::Auto => json!("auto"),
        ToolChoice::Required => json!("required"),
        ToolChoice::Force(name) => json!({ "type": "function", "function": { "name": name } }),
        ToolChoice::Disabled => json!("none"),
    }
}

/// Text and tool calls of `choices[0].message`.
pub(crate) fn parse(reply: &Value) -> ChatReply {
    let message = &reply["choices"][0]["message"];
    let text = message["content"].as_str().unwrap_or_default().trim().to_string();
    ChatReply { text, tool_calls: parse_tool_calls(&message["tool_calls"]) }
}

/// An OpenAI-style tool-call array (`[{id, function: {name, arguments}}]`); flat
/// `{id, name, arguments}` items are accepted too.
pub(crate) fn parse_tool_calls(calls: &Value) -> Vec<ToolCall> {
    let Some(items) = calls.as_array() else { return Vec::new() };
    items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            let function = item.get("function").unwrap_or(item);
            let name = function["name"].as_str()?.to_string();
            let id = item["id"].as_str().map_or_else(|| format!("call_{idx}"), str::to_string);
            Some(ToolCall { id, name, arguments: parse_arguments(&function["arguments"]) })
        })
        .collect()
}
