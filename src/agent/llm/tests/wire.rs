//! Golden request bodies and reply parsing of every wire format.

use crate::agent::llm::wire::{anthropic, gemini, openai};
use crate::agent::llm::{ChatMessage, ChatRequest, LlmProvider, LlmSettings, ToolCall, ToolChoice, ToolSpec};
use serde_json::json;

fn settings(provider: LlmProvider, model: &str) -> LlmSettings {
    LlmSettings { provider, model: model.into(), temperature: 0.5, max_tokens: 200, ..LlmSettings::default() }
}

fn call() -> ToolCall {
    ToolCall { id: "call_1".into(), name: "vault_search".into(), arguments: json!({ "query": "monism" }) }
}

/// system, user, an assistant tool call, two tool results.
fn conversation() -> ChatRequest {
    let second = ToolCall { id: "call_2".into(), ..call() };
    ChatRequest {
        messages: vec![
            ChatMessage::system("SYS"),
            ChatMessage::user("USER"),
            ChatMessage::assistant("", vec![call(), second.clone()]),
            ChatMessage::tool_result(&call(), "R1"),
            ChatMessage::tool_result(&second, "R2"),
        ],
        tools: vec![ToolSpec { name: "vault_search".into(), description: "Search".into(), parameters: json!({ "type": "object" }) }],
        tool_choice: ToolChoice::Auto,
        ..ChatRequest::default()
    }
}

#[test]
fn openai_body_threads_tool_calls_and_results() {
    let body = openai::body(&settings(LlmProvider::OpenAi, "gpt-4o-mini"), &conversation());
    let call_json = json!({ "id": "call_1", "type": "function", "function": { "name": "vault_search", "arguments": "{\"query\":\"monism\"}" } });
    let expected = json!({
        "model": "gpt-4o-mini", "temperature": 0.5, "max_tokens": 200,
        "messages": [
            { "role": "system", "content": "SYS" },
            { "role": "user", "content": "USER" },
            { "role": "assistant", "content": null, "tool_calls": [call_json, {
                "id": "call_2", "type": "function", "function": { "name": "vault_search", "arguments": "{\"query\":\"monism\"}" } }] },
            { "role": "tool", "tool_call_id": "call_1", "content": "R1" },
            { "role": "tool", "tool_call_id": "call_2", "content": "R2" }
        ],
        "tools": [{ "type": "function", "function": { "name": "vault_search", "description": "Search", "parameters": { "type": "object" } } }],
        "tool_choice": "auto"
    });
    assert_eq!(body, expected);
}

#[test]
fn openai_body_adds_schema_search_and_forced_choices() {
    let schema = json!({ "type": "object", "properties": { "content": { "type": "string" } } });
    let mut req = ChatRequest { schema: Some(schema.clone()), web_search: true, ..ChatRequest::simple("S", "U") };
    let body = openai::body(&settings(LlmProvider::OpenAi, " "), &req);
    assert_eq!(body["model"], "gpt-4o-mini", "a blank model falls back to the provider default");
    assert_eq!(body["response_format"], json!({ "type": "json_schema", "json_schema": { "name": "emit_post", "schema": schema } }));
    assert_eq!(body["web_search_options"], json!({}));
    assert!(body.get("tools").is_none() && body.get("tool_choice").is_none());
    let routed = openai::body(&settings(LlmProvider::OpenRouter, "x"), &req);
    assert_eq!(routed["plugins"], json!([{ "id": "web" }]));
    req.tools = conversation().tools;
    for (choice, wire) in [
        (ToolChoice::Required, json!("required")),
        (ToolChoice::Disabled, json!("none")),
        (ToolChoice::Force("emit_post".into()), json!({ "type": "function", "function": { "name": "emit_post" } })),
    ] {
        req.tool_choice = choice;
        assert_eq!(openai::body(&settings(LlmProvider::Groq, "m"), &req)["tool_choice"], wire);
    }
}

#[test]
fn openai_replies_parse_text_and_tool_calls() {
    let reply = json!({ "choices": [{ "message": { "content": "  hello  ", "tool_calls": [
        { "id": "a", "function": { "name": "web_fetch", "arguments": "{\"url\":\"https://x.test\"}" } },
        { "function": { "name": "odd", "arguments": "not json" } }
    ] } }] });
    let parsed = openai::parse(&reply);
    assert_eq!(parsed.text, "hello");
    assert_eq!(parsed.tool_calls[0], ToolCall { id: "a".into(), name: "web_fetch".into(), arguments: json!({ "url": "https://x.test" }) });
    assert_eq!(parsed.tool_calls[1].id, "call_1");
    assert_eq!(parsed.tool_calls[1].arguments, json!("not json"));
    let flat = openai::parse_tool_calls(&json!([{ "id": "f", "name": "n", "arguments": { "k": 1 } }]));
    assert_eq!(flat[0].arguments, json!({ "k": 1 }));
    assert!(openai::parse(&json!({ "choices": [] })).is_empty());
}

#[test]
fn anthropic_body_groups_tool_results_into_one_user_turn() {
    let mut req = conversation();
    req.web_search = true;
    req.tool_choice = ToolChoice::Force("emit_post".into());
    let body = anthropic::body(&settings(LlmProvider::Anthropic, "claude-x"), &req);
    let expected = json!({
        "model": "claude-x", "system": "SYS", "max_tokens": 200, "temperature": 0.5,
        "messages": [
            { "role": "user", "content": "USER" },
            { "role": "assistant", "content": [
                { "type": "tool_use", "id": "call_1", "name": "vault_search", "input": { "query": "monism" } },
                { "type": "tool_use", "id": "call_2", "name": "vault_search", "input": { "query": "monism" } } ] },
            { "role": "user", "content": [
                { "type": "tool_result", "tool_use_id": "call_1", "content": "R1" },
                { "type": "tool_result", "tool_use_id": "call_2", "content": "R2" } ] }
        ],
        "tools": [
            { "name": "vault_search", "description": "Search", "input_schema": { "type": "object" } },
            { "type": "web_search_20250305", "name": "web_search", "max_uses": 3 }
        ],
        "tool_choice": { "type": "tool", "name": "emit_post" }
    });
    assert_eq!(body, expected);
}

#[test]
fn anthropic_replies_parse_text_blocks_and_tool_use() {
    let reply = json!({ "content": [
        { "type": "text", "text": "Let me look. " },
        { "type": "server_tool_use", "id": "s", "name": "web_search", "input": {} },
        { "type": "tool_use", "id": "tu_1", "name": "emit_post", "input": { "content": "Hi" } },
        { "type": "text", "text": "Done" }
    ] });
    let parsed = anthropic::parse(&reply);
    assert_eq!(parsed.text, "Let me look. Done");
    assert_eq!(parsed.tool_calls, vec![ToolCall { id: "tu_1".into(), name: "emit_post".into(), arguments: json!({ "content": "Hi" }) }]);
}

#[test]
fn gemini_body_uses_function_declarations_and_sanitized_schemas() {
    let mut req = conversation();
    req.tools[0].parameters = json!({ "type": "object", "additionalProperties": false, "$schema": "x",
        "properties": { "q": { "type": "string", "title": "Q" } }, "required": ["q"] });
    req.tool_choice = ToolChoice::Required;
    req.web_search = true;
    let body = gemini::body(&settings(LlmProvider::Gemini, "gemini-x"), &req);
    assert_eq!(body["systemInstruction"], json!({ "parts": [{ "text": "SYS" }] }));
    assert_eq!(body["contents"], json!([
        { "role": "user", "parts": [{ "text": "USER" }] },
        { "role": "model", "parts": [
            { "functionCall": { "name": "vault_search", "args": { "query": "monism" } } },
            { "functionCall": { "name": "vault_search", "args": { "query": "monism" } } } ] },
        { "role": "user", "parts": [
            { "functionResponse": { "name": "vault_search", "response": { "content": "R1" } } },
            { "functionResponse": { "name": "vault_search", "response": { "content": "R2" } } } ] }
    ]));
    assert_eq!(body["tools"], json!([
        { "functionDeclarations": [{ "name": "vault_search", "description": "Search",
            "parameters": { "type": "object", "properties": { "q": { "type": "string" } }, "required": ["q"] } }] },
        { "google_search": {} }
    ]));
    assert_eq!(body["toolConfig"], json!({ "functionCallingConfig": { "mode": "ANY" } }));
    assert_eq!(body["generationConfig"], json!({ "temperature": 0.5, "maxOutputTokens": 200 }));
    assert_eq!(gemini::url("https://g.test/models/", &settings(LlmProvider::Gemini, "")), "https://g.test/models/gemini-1.5-flash:generateContent");
}

#[test]
fn gemini_schema_output_and_reply_parsing() {
    let req = ChatRequest { schema: Some(json!({ "type": "object", "title": "T" })), ..ChatRequest::simple("S", "U") };
    let body = gemini::body(&settings(LlmProvider::Gemini, "g"), &req);
    assert_eq!(body["generationConfig"]["responseMimeType"], "application/json");
    assert_eq!(body["generationConfig"]["responseSchema"], json!({ "type": "object" }));
    assert!(body.get("tools").is_none() && body.get("toolConfig").is_none());
    let reply = json!({ "candidates": [{ "content": { "parts": [
        { "text": "thinking", "thought": true }, { "text": "Answer " }, { "functionCall": { "name": "read_network", "args": { "action": "recent" } } }
    ] } }] });
    let parsed = gemini::parse(&reply);
    assert_eq!(parsed.text, "Answer");
    assert_eq!(parsed.tool_calls, vec![ToolCall { id: "call_2".into(), name: "read_network".into(), arguments: json!({ "action": "recent" }) }]);
}

#[test]
fn providers_declare_search_and_schema_strategies() {
    use crate::agent::llm::SchemaStrategy as S;
    assert_eq!(LlmProvider::OpenAi.schema_strategy(true), S::ResponseFormat);
    assert_eq!(LlmProvider::Gemini.schema_strategy(false), S::ResponseFormat);
    assert_eq!(LlmProvider::Gemini.schema_strategy(true), S::EmitTool);
    assert_eq!(LlmProvider::Anthropic.schema_strategy(false), S::EmitTool);
    assert_eq!(LlmProvider::Groq.schema_strategy(false), S::Instruct);
    assert!(LlmProvider::Anthropic.native_web_search() && !LlmProvider::Custom.native_web_search());
}
