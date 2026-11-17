//! The tool loop and structured output over native providers, against a mock server.

use super::{openai_text, openai_tool_call, plan, tool_context, SECRET};
use crate::agent::llm::LlmProvider;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::run_engine;
use crate::domain::engine::{EngineConfig, OutputMode, SchemaPreset, StructuredOutput, ToolGrant};
use crate::domain::post::SourceKind;
use serde_json::json;

fn with_tools(tools: Vec<ToolGrant>, max_tool_steps: u8) -> EngineConfig {
    EngineConfig { tools, max_tool_steps, ..EngineConfig::default() }
}

fn structured(preset: SchemaPreset) -> OutputMode {
    OutputMode::JsonSchema(StructuredOutput::from_preset(preset))
}

#[tokio::test]
async fn openai_tool_results_are_threaded_back_until_the_model_answers() {
    let server = MockServer::start(vec![
        MockReply::json(openai_tool_call("call_v", "vault_search", json!({ "query": "monism" }))),
        MockReply::json(openai_text("Substance is one, friend.")),
    ]);
    let plan = plan(LlmProvider::OpenAi, &server.url, with_tools(vec![ToolGrant::VaultSearch], 3));
    let result = run_engine(&plan, "SYS", "USER", Some(tool_context()), 0).await.expect("engine runs");
    assert_eq!(result.draft.content, "Substance is one, friend.");
    assert_eq!(result.tools.len(), 1);
    assert_eq!((result.tools[0].name.as_str(), result.tools[0].summary.as_str()), ("vault_search", r#"{"query":"monism"}"#));
    assert_eq!((result.sources[0].kind, result.sources[0].id.as_str()), (SourceKind::VaultDoc, "doc_ethics"));

    let requests = server.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].header("authorization"), Some(format!("Bearer {SECRET}").as_str()));
    let first = requests[0].json();
    assert_eq!(first["model"], "test-model");
    assert_eq!(first["tools"][0]["function"]["name"], "vault_search");
    assert_eq!(first["tool_choice"], "auto");
    let second = requests[1].json();
    let messages = second["messages"].as_array().expect("messages");
    assert_eq!(messages.len(), 4);
    assert_eq!(messages[2]["tool_calls"][0]["id"], "call_v");
    assert_eq!(messages[3]["role"], "tool");
    assert_eq!(messages[3]["tool_call_id"], "call_v");
    assert!(messages[3]["content"].as_str().is_some_and(|c| c.contains("[doc_ethics] Ethics")));
}

#[tokio::test]
async fn the_loop_stops_at_the_step_budget() {
    let looping = || MockReply::json(openai_tool_call("again", "vault_search", json!({ "query": "x" })));
    let server = MockServer::start(vec![looping(), looping(), looping(), looping()]);
    let plan = plan(LlmProvider::OpenAi, &server.url, with_tools(vec![ToolGrant::VaultSearch], 2));
    let error = run_engine(&plan, "S", "U", Some(tool_context()), 0).await.expect_err("never answers");
    assert!(error.contains("empty post"), "{error}");
    let requests = server.requests();
    assert_eq!(requests.len(), 3, "two tool rounds plus the final forced answer");
    assert_eq!(requests[2].json()["tool_choice"], "none");
    assert_eq!(requests[1].json()["tool_choice"], "auto");
}

#[tokio::test]
async fn openai_structured_output_maps_onto_the_post() {
    let answer = json!({ "content": "One substance.", "thread_parts": ["Mind is extension.", ""], "hashtags": ["#Monism", "Ethics"] });
    let server = MockServer::start(vec![MockReply::json(openai_text(&answer.to_string()))]);
    let engine = EngineConfig { output: structured(SchemaPreset::Thread), ..EngineConfig::default() };
    let result = run_engine(&plan(LlmProvider::OpenAi, &server.url, engine), "S", "U", None, 0).await.expect("engine runs");
    assert_eq!(result.draft.body(), "One substance. #Monism #Ethics");
    assert_eq!(result.draft.thread_parts, vec!["Mind is extension."]);
    let request = server.requests()[0].json();
    assert_eq!(request["response_format"]["json_schema"]["name"], "emit_post");
    assert_eq!(request["response_format"]["json_schema"]["schema"]["required"], json!(["content", "thread_parts"]));
}

#[tokio::test]
async fn anthropic_structured_output_rides_on_a_forced_emit_tool() {
    let tool_use = |name: &str, input| json!({ "content": [{ "type": "tool_use", "id": "tu", "name": name, "input": input }] });
    let server = MockServer::start(vec![
        MockReply::json(tool_use("vault_search", json!({ "query": "bread" }))),
        MockReply::json(tool_use("emit_post", json!({ "content": "Bake it.", "mood": "warm", "hashtags": "bread, craft" }))),
    ]);
    let engine = EngineConfig { output: structured(SchemaPreset::Expressive), ..with_tools(vec![ToolGrant::VaultSearch], 3) };
    let plan = plan(LlmProvider::Anthropic, &format!("{}/v1/messages", server.url), engine);
    let result = run_engine(&plan, "SYS", "U", Some(tool_context()), 0).await.expect("engine runs");
    assert_eq!(result.draft.body(), "Bake it. #bread #craft");
    assert_eq!(result.draft.mood.as_deref(), Some("warm"));
    assert_eq!(result.extras().mood.as_deref(), Some("warm"));
    let requests = server.requests();
    assert_eq!(requests[0].path, "/v1/messages");
    assert_eq!(requests[0].header("x-api-key"), Some(SECRET));
    let first = requests[0].json();
    assert_eq!(first["tool_choice"], json!({ "type": "any" }));
    assert_eq!(first["tools"][1]["name"], "emit_post");
    assert!(first["system"].as_str().is_some_and(|s| s.contains("emit_post")));
    assert_eq!(requests[1].json()["messages"][2]["content"][0]["type"], "tool_result");
}

#[tokio::test]
async fn gemini_uses_response_schema_without_tools() {
    let text = json!({ "content": "Look closely.", "image_prompt": "an owl at dusk" }).to_string();
    let reply = json!({ "candidates": [{ "content": { "parts": [{ "text": text }] } }] });
    let server = MockServer::start(vec![MockReply::json(reply)]);
    let engine = EngineConfig { output: structured(SchemaPreset::Illustrated), ..EngineConfig::default() };
    let result = run_engine(&plan(LlmProvider::Gemini, &format!("{}/models", server.url), engine), "S", "U", None, 0)
        .await
        .expect("engine runs");
    assert_eq!(result.draft.image_prompt.as_deref(), Some("an owl at dusk"));
    let request = &server.requests()[0];
    assert_eq!(request.path, "/models/test-model:generateContent");
    assert_eq!(request.header("x-goog-api-key"), Some(SECRET));
    assert!(!request.path.contains("key="), "the key never travels in the URL");
    assert_eq!(request.json()["generationConfig"]["responseMimeType"], "application/json");
}

#[tokio::test]
async fn instructed_json_gets_one_repair_turn() {
    let server = MockServer::start(vec![
        MockReply::json(openai_text("Sure! Here is my post about time.")),
        MockReply::json(openai_text("```json\n{\"content\": \"Time flows.\"}\n```")),
    ]);
    let engine = EngineConfig { output: structured(SchemaPreset::Post), ..EngineConfig::default() };
    let result = run_engine(&plan(LlmProvider::Groq, &server.url, engine), "S", "U", None, 0).await.expect("engine runs");
    assert_eq!(result.draft.content, "Time flows.");
    let requests = server.requests();
    assert!(requests[0].json()["messages"][0]["content"].as_str().is_some_and(|s| s.contains("OUTPUT FORMAT")));
    assert!(requests[0].json().get("response_format").is_none());
    let repair = requests[1].json();
    assert_eq!(repair["messages"][2]["content"], "Sure! Here is my post about time.");
    assert!(repair["messages"][3]["content"].as_str().is_some_and(|s| s.starts_with("That was not a valid JSON")));
}

#[tokio::test]
async fn provider_failures_are_redacted() {
    let server = MockServer::start(vec![MockReply::raw(401, "application/json", format!("{{\"error\":\"bad key {SECRET}\"}}"))]);
    let plan = plan(LlmProvider::OpenAi, &server.url, EngineConfig::default());
    let raw = run_engine(&plan, "S", "U", None, 0).await.expect_err("401");
    let failure = plan.transport.describe_failure(&raw, &plan.credentials.secrets());
    assert!(raw.contains(SECRET), "the raw error echoes the key");
    assert!(!failure.message.contains(SECRET) && failure.message.contains("401"), "{}", failure.message);
    assert_eq!(failure.provider, LlmProvider::OpenAi);
    assert_eq!(plan.gate_key, super::CRED_ID);
}
