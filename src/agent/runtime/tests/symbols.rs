//! Engine output mapping ignores symbols (S2): an answer that names symbols posts without them.

use super::{media_dir, store, CRED_ID};
use crate::agent::llm::{LlmProvider, LlmSettings};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::{run_engine, EnginePlan, PostDraft};
use crate::domain::engine::{CustomRequest, EngineConfig, OutputMapping, OutputMode, RequestMode, SchemaPreset, StructuredOutput};
use serde_json::json;

const SYMBOLS_JSON: &str = r#"{"entry": "task", "state": "open", "signifiers": ["priority"]}"#;

#[tokio::test]
async fn an_answer_that_names_symbols_posts_without_them() {
    let server = MockServer::start(vec![MockReply::json(json!({ "data": { "text": format!("{{\"content\": \"Ship it.\", \"symbols\": {SYMBOLS_JSON}}}") } }))]);
    let request = CustomRequest {
        url: format!("{}/gen", server.url),
        body_template: r#"{"sys": {{system}}, "schema": {{schema}}}"#.into(),
        reply_path: "data.text".into(),
        credential: Some(CRED_ID.into()),
        ..CustomRequest::default()
    };
    let output = StructuredOutput { may_tint: true, may_react: true, ..StructuredOutput::from_preset(SchemaPreset::Post) };
    let engine = EngineConfig { request: RequestMode::Custom(request), output: OutputMode::JsonSchema(output), ..EngineConfig::default() };
    let plan = EnginePlan::resolve(&engine, &LlmSettings::default(), &store(LlmProvider::Custom, ""), media_dir());
    let result = run_engine(&plan, "sys", "user", None, 0).await.expect("generation");
    assert_eq!(result.draft.content, "Ship it.");
    assert!(result.extras().symbols.is_empty(), "the mapping has no symbols field");
    let schema = plan.structured.expect("structured").schema;
    assert!(!schema.contains("symbol"), "agents are never asked for symbols");
}

#[test]
fn every_mapping_drops_a_symbols_key() {
    let answer = json!({ "content": "x", "symbols": serde_json::from_str::<serde_json::Value>(SYMBOLS_JSON).unwrap() });
    let draft = PostDraft::from_structured(&answer, &OutputMapping::top_level()).expect("postable");
    assert_eq!(draft.content, "x");
    assert!(!serde_json::to_string(&OutputMapping::top_level()).unwrap().contains("symbol"));
}
