//! Agents may post feelings (M5): only with the setting on, which is off by default.

use super::{media_dir, store, CRED_ID};
use crate::agent::llm::{LlmProvider, LlmSettings};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::execute::allowed_feeling;
use crate::agent::runtime::{run_engine, EnginePlan, PostDraft};
use crate::domain::engine::{CustomRequest, EngineConfig, OutputMapping, OutputMode, RequestMode, SchemaPreset, StructuredOutput, FEELING_FIELD};
use crate::domain::post::{Intensity, PostKind};
use serde_json::json;

fn feeling_plan(url: String, output: StructuredOutput) -> EnginePlan {
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

fn felt_answer() -> MockReply {
    let text = json!({ "content": "Rain on the roof.", "feeling": { "emojis": ["😌"], "intensity": 2, "note": "rain" } }).to_string();
    MockReply::json(json!({ "data": { "text": text } }))
}

fn rain() -> PostKind {
    PostKind::feeling(&["😌".to_string()], Intensity::new(2), "rain").expect("valid")
}

#[tokio::test]
async fn an_agent_allowed_to_post_feelings_is_asked_for_one_and_gets_it() {
    let server = MockServer::start(vec![felt_answer()]);
    let output = StructuredOutput { may_feel: true, ..StructuredOutput::from_preset(SchemaPreset::Post) };
    let result = run_engine(&feeling_plan(format!("{}/gen", server.url), output), "SYS", "USER", None, 0).await.expect("a post");
    assert_eq!(result.draft.feeling, Some(rain()));
    let schema = &server.requests()[0].json()["schema"];
    assert_eq!(schema["properties"][FEELING_FIELD]["type"], json!("object"));
}

#[tokio::test]
async fn by_default_an_agent_is_not_asked_and_a_feeling_is_dropped() {
    let server = MockServer::start(vec![felt_answer()]);
    let mut output = StructuredOutput::from_preset(SchemaPreset::Post);
    assert!(!output.may_feel, "off by default");
    output.mapping.feeling = FEELING_FIELD.into();
    let result = run_engine(&feeling_plan(format!("{}/gen", server.url), output), "SYS", "USER", None, 0).await.expect("a post");
    assert_eq!(result.draft.content, "Rain on the roof.");
    assert_eq!(result.draft.feeling, None, "a hand-mapped feeling is dropped while the setting is off");
    assert!(server.requests()[0].json()["schema"]["properties"].get(FEELING_FIELD).is_none());
}

#[test]
fn the_draft_reads_a_feeling_and_the_gate_needs_the_setting() {
    let mapping = OutputMapping { feeling: "mood.felt".into(), ..OutputMapping::top_level() };
    let draft = PostDraft::from_structured(&json!({ "content": "x", "mood": { "felt": { "emojis": ["😌"], "intensity": 2, "note": "rain" } } }), &mapping).expect("postable");
    assert_eq!(draft.feeling, Some(rain()));
    let allowed = StructuredOutput { may_feel: true, ..StructuredOutput::default() };
    assert_eq!(allowed_feeling(Some(&allowed), Some(rain())), Some(rain()));
    assert_eq!(allowed_feeling(Some(&StructuredOutput::default()), Some(rain())), None);
    assert_eq!(allowed_feeling(None, Some(rain())), None, "free text never posts a feeling");
}
