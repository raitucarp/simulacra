//! Agents react (#171): only with the setting on, which is off by default.

use super::{media_dir, store, CRED_ID};
use crate::agent::llm::{LlmProvider, LlmSettings};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::execute::allowed_reactions;
use crate::agent::runtime::{run_engine, EnginePlan, PostDraft};
use crate::domain::engine::{CustomRequest, EngineConfig, OutputMapping, OutputMode, RequestMode, SchemaPreset, StructuredOutput, REACTIONS_FIELD};
use serde_json::json;

/// A custom-request plan answering through `output`, against `url`.
fn react_plan(url: String, output: StructuredOutput) -> EnginePlan {
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

fn reacting_answer() -> MockReply {
    MockReply::json(json!({ "data": { "text": "{\"content\": \"Agreed.\", \"reactions\": [\"\u{1F525}\", \"nope\", \"\u{1F914}\"]}" } }))
}

#[tokio::test]
async fn an_agent_allowed_to_react_is_asked_for_reactions_and_gets_them() {
    let server = MockServer::start(vec![reacting_answer()]);
    let output = StructuredOutput { may_react: true, ..StructuredOutput::from_preset(SchemaPreset::Post) };
    let plan = react_plan(format!("{}/gen", server.url), output);
    let result = run_engine(&plan, "SYS", "USER", None, 0).await.expect("a post");
    assert_eq!(result.draft.reactions, vec!["\u{1F525}", "\u{1F914}"], "text that is not an emoji is dropped");
    let schema = &server.requests()[0].json()["schema"];
    assert_eq!(schema["properties"][REACTIONS_FIELD]["type"], json!("array"));
    assert!(!schema["required"].as_array().expect("required").contains(&json!(REACTIONS_FIELD)));
}

#[tokio::test]
async fn by_default_an_agent_is_not_asked_and_reactions_are_dropped() {
    let server = MockServer::start(vec![reacting_answer()]);
    let mut output = StructuredOutput::from_preset(SchemaPreset::Post);
    assert!(!output.may_react, "off by default");
    output.mapping.reactions = REACTIONS_FIELD.into();
    let plan = react_plan(format!("{}/gen", server.url), output);
    let result = run_engine(&plan, "SYS", "USER", None, 0).await.expect("a post");
    assert_eq!(result.draft.content, "Agreed.");
    assert!(result.draft.reactions.is_empty(), "hand-mapped reactions are dropped while the setting is off");
    assert!(server.requests()[0].json()["schema"]["properties"].get(REACTIONS_FIELD).is_none());
}

#[test]
fn the_draft_reads_reactions_and_the_gate_needs_the_setting() {
    let mapping = OutputMapping { reactions: "meta.emoji".into(), ..OutputMapping::top_level() };
    let draft = PostDraft::from_structured(&json!({ "content": "x", "meta": { "emoji": ["\u{1F44F}", "\u{1F44F}"] } }), &mapping).expect("postable");
    assert_eq!(draft.reactions, vec!["\u{1F44F}"], "an emoji counts once");
    let allowed = StructuredOutput { may_react: true, ..StructuredOutput::default() };
    assert_eq!(allowed_reactions(Some(&allowed), draft.reactions.clone()), draft.reactions);
    assert!(allowed_reactions(Some(&StructuredOutput::default()), draft.reactions.clone()).is_empty());
    assert!(allowed_reactions(None, draft.reactions).is_empty(), "free text never reacts");
}
