//! Agents may set a tint (#186): only with the setting on, which is off by default.

use super::{media_dir, store, CRED_ID};
use crate::agent::llm::{LlmProvider, LlmSettings};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::execute::allowed_tint;
use crate::agent::runtime::{run_engine, EnginePlan, PostDraft};
use crate::domain::engine::{CustomRequest, EngineConfig, OutputMapping, OutputMode, RequestMode, SchemaPreset, StructuredOutput, TINT_FIELD};
use crate::domain::post::Tint;
use serde_json::json;

/// A custom-request plan answering through `output`, against `url`.
fn tint_plan(url: String, output: StructuredOutput) -> EnginePlan {
    let request = CustomRequest {
        url,
        body_template: r#"{"sys": {{system}}, "schema": {{schema}}}"#.into(),
        reply_path: "data.text".into(),
        credential: Some(CRED_ID.into()),
        ..CustomRequest::default()
    };
    let engine = EngineConfig { request: RequestMode::Custom(request), output: OutputMode::JsonSchema(output), ..EngineConfig::default() };
    EnginePlan::resolve(&engine, &LlmSettings::default(), &store(LlmProvider::Custom, ""), media_dir())
}

fn tinted_answer() -> MockReply {
    MockReply::json(json!({ "data": { "text": "{\"content\": \"Dusk again.\", \"tint\": \"Mint\"}" } }))
}

#[tokio::test]
async fn an_agent_allowed_to_tint_is_asked_for_a_tint_and_gets_it() {
    let server = MockServer::start(vec![tinted_answer()]);
    let output = StructuredOutput { may_tint: true, ..StructuredOutput::from_preset(SchemaPreset::Post) };
    let plan = tint_plan(format!("{}/gen", server.url), output);
    let result = run_engine(&plan, "SYS", "USER", None, 0).await.expect("a post");
    assert_eq!(result.extras().tint, Some(Tint::Mint));
    let schema = &server.requests()[0].json()["schema"];
    assert_eq!(schema["properties"][TINT_FIELD]["enum"][0], json!("amber"));
    assert!(!schema["required"].as_array().expect("required").contains(&json!(TINT_FIELD)));
}

#[tokio::test]
async fn by_default_an_agent_is_not_asked_and_a_tint_is_dropped() {
    let server = MockServer::start(vec![tinted_answer()]);
    let mut output = StructuredOutput::from_preset(SchemaPreset::Post);
    assert!(!output.may_tint, "off by default");
    output.mapping.tint = TINT_FIELD.into();
    let plan = tint_plan(format!("{}/gen", server.url), output);
    let result = run_engine(&plan, "SYS", "USER", None, 0).await.expect("a post");
    assert_eq!(result.draft.content, "Dusk again.");
    assert_eq!(result.extras().tint, None, "a hand-mapped tint is dropped while the setting is off");
    assert!(server.requests()[0].json()["schema"]["properties"].get(TINT_FIELD).is_none());
}

#[test]
fn the_draft_reads_a_known_tint_and_the_gate_needs_the_setting() {
    let mapping = OutputMapping { tint: "look.tint".into(), ..OutputMapping::top_level() };
    let draft = PostDraft::from_structured(&json!({ "content": "x", "look": { "tint": " ROSE " } }), &mapping).expect("postable");
    assert_eq!(draft.tint, Some(Tint::Rose));
    let unknown = PostDraft::from_structured(&json!({ "content": "x", "look": { "tint": "teal" } }), &mapping).expect("postable");
    assert_eq!(unknown.tint, None);
    let allowed = StructuredOutput { may_tint: true, ..StructuredOutput::default() };
    assert_eq!(allowed_tint(Some(&allowed), Some(Tint::Rose)), Some(Tint::Rose));
    assert_eq!(allowed_tint(Some(&StructuredOutput::default()), Some(Tint::Rose)), None);
    assert_eq!(allowed_tint(None, Some(Tint::Rose)), None, "free text never tints");
}
