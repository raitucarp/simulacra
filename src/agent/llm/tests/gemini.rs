//! Gemini 3+ reply details: thought signatures and call ids echoed back, grounding pages as
//! sources, and empty replies explained by their finish reason (mock server only).

use crate::agent::llm::wire::{finish, gemini, openai};
use crate::agent::llm::{ChatMessage, ChatRequest, LlmClient, LlmProvider, LlmSettings};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::post::SourceKind;
use crate::domain::post_actions::ActionFailureKind;
use serde_json::{json, Value};

fn client(server: &MockServer, provider: LlmProvider) -> LlmClient {
    let settings = LlmSettings { provider, api_key: "test-key-not-real".into(), model: "m".into(), ..LlmSettings::default() };
    LlmClient::new(settings).with_endpoint(&server.url)
}

/// The request contents after `reply`'s calls ran and each answered `"R"`.
fn next_contents(reply: &Value) -> Value {
    let parsed = gemini::parse(reply);
    let mut messages = vec![ChatMessage::system("S"), ChatMessage::user("U"), ChatMessage::assistant(parsed.text.clone(), parsed.tool_calls.clone())];
    messages.extend(parsed.tool_calls.iter().map(|call| ChatMessage::tool_result(call, "R")));
    let req = ChatRequest { messages, ..ChatRequest::default() };
    gemini::body(&LlmSettings::default(), &req)["contents"].clone()
}

#[test]
fn thought_signatures_and_model_ids_are_echoed_back() {
    let reply = json!({ "candidates": [{ "content": { "parts": [
        { "functionCall": { "id": "fc-77", "name": "vault_search", "args": { "query": "a" } }, "thoughtSignature": "SIG-A" },
        { "functionCall": { "name": "vault_search", "args": { "query": "b" } } }
    ] } }] });
    let parsed = gemini::parse(&reply);
    assert_eq!(parsed.tool_calls[0].signature.as_deref(), Some("SIG-A"));
    assert_eq!((parsed.tool_calls[1].id.as_str(), parsed.tool_calls[1].signature.as_deref()), ("call_1", None));
    let contents = next_contents(&reply);
    assert_eq!(contents[1]["parts"], json!([
        { "functionCall": { "id": "fc-77", "name": "vault_search", "args": { "query": "a" } }, "thoughtSignature": "SIG-A" },
        { "functionCall": { "name": "vault_search", "args": { "query": "b" } } }
    ]), "the signature rides on the call it came with; made-up ids are never sent");
    assert_eq!(contents[2]["parts"], json!([
        { "functionResponse": { "id": "fc-77", "name": "vault_search", "response": { "content": "R" } } },
        { "functionResponse": { "name": "vault_search", "response": { "content": "R" } } }
    ]));
}

#[test]
fn grounding_pages_become_web_sources() {
    let reply = json!({ "candidates": [{ "content": { "parts": [{ "text": "Rust 2.0 shipped." }] }, "groundingMetadata": {
        "webSearchQueries": ["rust release"],
        "groundingChunks": [
            { "web": { "uri": "https://grounding.test/a", "title": "rust-lang.org" } },
            { "web": { "uri": "https://grounding.test/a", "title": "rust-lang.org" } },
            { "web": { "uri": "https://grounding.test/b" } },
            { "retrievedContext": { "uri": "ignored" } }
        ] } }] });
    let sources = gemini::parse(&reply).sources;
    assert_eq!(sources.len(), 2, "each page once, non-web chunks skipped: {sources:?}");
    assert_eq!((sources[0].kind, sources[0].id.as_str(), sources[0].title.as_str()), (SourceKind::Web, "https://grounding.test/a", "rust-lang.org"));
    assert_eq!(sources[1].title, "https://grounding.test/b", "an untitled page is named by its URL");
    assert!(gemini::parse(&json!({ "candidates": [] })).sources.is_empty());
}

#[tokio::test]
async fn empty_gemini_replies_say_why() {
    let replies = [
        (json!({ "candidates": [{ "content": { "parts": [] }, "finishReason": "MAX_TOKENS" }] }), "raise Max tokens (MAX_TOKENS)"),
        (json!({ "promptFeedback": { "blockReason": "SAFETY" } }), "the prompt was blocked (SAFETY)"),
        (json!({ "candidates": [{ "finishReason": "SAFETY", "safetyRatings": [
            { "category": "HARM_CATEGORY_HARASSMENT", "probability": "LOW" },
            { "category": "HARM_CATEGORY_DANGEROUS_CONTENT", "probability": "HIGH", "blocked": true } ] }] }), "withheld (SAFETY: DANGEROUS_CONTENT)"),
        (json!({ "candidates": [{ "finishReason": "RECITATION" }] }), "the reply was withheld (RECITATION)"),
        (json!({ "candidates": [{ "finishReason": "MALFORMED_FUNCTION_CALL" }] }), "the reply ended early (MALFORMED_FUNCTION_CALL)"),
    ];
    for (reply, expected) in replies {
        let server = MockServer::start(vec![MockReply::json(reply)]);
        let error = client(&server, LlmProvider::Gemini).generate("S", "U").await.expect_err("empty");
        assert!(error.starts_with("Gemini returned empty candidate text: ") && error.contains(expected), "{error}");
        assert_eq!(ActionFailureKind::classify(&error), ActionFailureKind::Empty);
        let failure = client(&server, LlmProvider::Gemini).describe_failure(&error);
        assert!(failure.message.contains(expected), "the reason survives shortening: {}", failure.message);
    }
    let server = MockServer::start(vec![MockReply::json(json!({ "candidates": [{ "finishReason": "STOP", "content": { "parts": [] } }] }))]);
    let error = client(&server, LlmProvider::Gemini).generate("S", "U").await.expect_err("empty");
    assert_eq!(error, "Gemini returned empty candidate text", "a normal stop adds nothing");
}

#[test]
fn other_providers_explain_empty_replies_too() {
    let openai_reply = |choice: Value| json!({ "choices": [choice] });
    let reason = |provider, reply: &Value| finish::empty_reason(provider, reply).unwrap_or_default();
    assert!(reason(LlmProvider::OpenAi, &openai_reply(json!({ "finish_reason": "length", "message": { "content": "" } }))).contains("(length)"));
    assert!(reason(LlmProvider::Custom, &openai_reply(json!({ "finish_reason": "content_filter" }))).contains("content filter"));
    assert!(reason(LlmProvider::Groq, &openai_reply(json!({ "message": { "refusal": "No." } }))).contains("refused: No."));
    assert!(reason(LlmProvider::Anthropic, &json!({ "stop_reason": "max_tokens", "content": [] })).contains("raise Max tokens"));
    assert_eq!(finish::empty_reason(LlmProvider::OpenAi, &openai_reply(json!({ "finish_reason": "stop" }))), None);
    assert_eq!(finish::empty_reason(LlmProvider::Anthropic, &json!({})), None);
}

#[test]
fn gemini_openai_compatible_signatures_round_trip() {
    let calls = json!([{ "id": "c9", "type": "function", "function": { "name": "vault_search", "arguments": "{\"query\":\"x\"}" },
        "extra_content": { "google": { "thought_signature": "SIG-O" } } }]);
    let parsed = openai::parse_tool_calls(&calls);
    assert_eq!(parsed[0].signature.as_deref(), Some("SIG-O"));
    let messages = vec![ChatMessage::user("U"), ChatMessage::assistant("", parsed.clone()), ChatMessage::tool_result(&parsed[0], "R")];
    let body = openai::body(&LlmSettings { provider: LlmProvider::Custom, ..LlmSettings::default() }, &ChatRequest { messages, ..ChatRequest::default() });
    assert_eq!(body["messages"][1]["tool_calls"][0]["extra_content"], json!({ "google": { "thought_signature": "SIG-O" } }));
    assert!(openai::parse_tool_calls(&json!([{ "id": "p", "function": { "name": "n", "arguments": "{}" } }]))[0].signature.is_none());
}
