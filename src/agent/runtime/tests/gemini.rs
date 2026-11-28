//! Gemini 3+ through the engine: a tool loop keeps the thought signature of each call, and
//! native Google Search grounding pages end up as the post's web sources.

use super::{plan, tool_context};
use crate::agent::llm::LlmProvider;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::run_engine;
use crate::domain::engine::{EngineConfig, SearchBackend, ToolGrant, WebSearchConfig};
use crate::domain::post::SourceKind;
use serde_json::json;

fn text(text: &str) -> serde_json::Value {
    json!({ "candidates": [{ "content": { "role": "model", "parts": [{ "text": text, "thoughtSignature": "SIG-TEXT" }] }, "finishReason": "STOP" }] })
}

#[tokio::test]
async fn a_gemini_tool_loop_sends_each_thought_signature_back() {
    let call = json!({ "candidates": [{ "content": { "role": "model", "parts": [
        { "functionCall": { "name": "vault_search", "args": { "query": "monism" } }, "thoughtSignature": "SIG-CALL" } ] } }] });
    let server = MockServer::start(vec![MockReply::json(call), MockReply::json(text("Substance is one."))]);
    let engine = EngineConfig { tools: vec![ToolGrant::VaultSearch], ..EngineConfig::default() };
    let plan = plan(LlmProvider::Gemini, &format!("{}/models", server.url), engine);
    let result = run_engine(&plan, "S", "U", Some(tool_context()), 0).await.expect("engine runs");
    assert_eq!(result.draft.content, "Substance is one.");
    assert_eq!(result.tools[0].name, "vault_search");
    let second = server.requests()[1].json();
    let model_turn = &second["contents"][1];
    assert_eq!(model_turn["role"], "model");
    assert_eq!(model_turn["parts"][0]["thoughtSignature"], "SIG-CALL");
    assert_eq!(second["contents"][2]["parts"][0]["functionResponse"]["name"], "vault_search");
    assert!(second["contents"][2]["parts"][0]["functionResponse"].get("id").is_none(), "no made-up id is sent");
}

#[tokio::test]
async fn native_search_grounding_becomes_web_sources() {
    let mut reply = text("Rust 2.0 is out.");
    reply["candidates"][0]["groundingMetadata"] = json!({ "groundingChunks": [{ "web": { "uri": "https://grounding.test/r", "title": "rust-lang.org" } }] });
    let server = MockServer::start(vec![MockReply::json(reply)]);
    let search = ToolGrant::WebSearch(WebSearchConfig { backend: SearchBackend::ProviderNative, credential: None });
    let engine = EngineConfig { tools: vec![search], ..EngineConfig::default() };
    let plan = plan(LlmProvider::Gemini, &format!("{}/models", server.url), engine);
    let result = run_engine(&plan, "S", "U", Some(tool_context()), 0).await.expect("engine runs");
    let request = server.requests()[0].json();
    assert_eq!(request["tools"], json!([{ "google_search": {} }]));
    assert_eq!(result.sources.len(), 1);
    assert_eq!((result.sources[0].kind, result.sources[0].id.as_str(), result.sources[0].title.as_str()), (SourceKind::Web, "https://grounding.test/r", "rust-lang.org"));
    assert_eq!(result.extras().sources, result.sources, "the post keeps them");
}
