//! A whole "suggest from context" round trip through the post actions runner, against the
//! local mock server only (no live provider, no real key).

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::post_actions::{run_action_blocking, ActionJob, CancelToken};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post_actions::{ActionFailureKind, RequestPolicy};
use crate::state::forge::{ForgeStep, TraitField};
use crate::state::forge::suggest::ai::{parse_suggestions, suggest_prompt, targets, SuggestScope, SuggestedValue};
use serde_json::json;
use std::time::Duration;

const TEST_KEY: &str = "test-key-not-real";
const TIMEOUT: Duration = Duration::from_secs(5);

fn job(url: &str, prompt: crate::domain::post_actions::ActionPrompt, retries: u32) -> ActionJob {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: TEST_KEY.into(), model: "mock-model".into(), ..LlmSettings::default() };
    ActionJob {
        client: LlmClient::new(settings).with_endpoint(url).with_timeout(TIMEOUT),
        prompt,
        policy: RequestPolicy { timeout: TIMEOUT, max_retries: retries },
        cancel: CancelToken::new(),
    }
}

fn answer(text: &str) -> MockReply {
    MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": text } }] }))
}

#[test]
fn a_step_round_trip_yields_validated_choices() {
    let draft = agent();
    let ctx = SuggestContext::of(&draft);
    let wanted = targets(SuggestScope::Step(ForgeStep::Rhetoric), ForgeKind::Agent, ctx.config());
    let reply = r#"Here you go: {"rhetorical_mode": ["narrative", "bogus"], "post_format": ["dialogue"], "lexical_fields": [["music", "law"]]}"#;
    let busy = MockReply::raw(503, "application/json", "{}");
    let server = MockServer::start(vec![busy, answer(reply)]);
    let text = run_action_blocking(&job(&server.url, suggest_prompt(&wanted, &ctx, "English"), 1)).expect("an answer after a retry");
    let sent = server.requests()[1].json();
    assert!(sent["messages"][1]["content"].as_str().unwrap_or_default().contains("rhetorical_mode"));
    let parsed = parse_suggestions(&text, &wanted, &ctx).expect("JSON");
    let mode = Param::Trait(TraitField::RhetoricalMode);
    assert_eq!(parsed.values.iter().find(|(p, _)| *p == mode).map(|(_, v)| v.clone()), Some(vec![SuggestedValue::Option("narrative")]));
    assert_eq!(parsed.discarded, 1);
    assert!(!text.contains(TEST_KEY));
}

#[test]
fn no_key_means_no_request() {
    let draft = agent();
    let ctx = SuggestContext::of(&draft);
    let server = MockServer::start(vec![answer("{}")]);
    let mut job = job(&server.url, suggest_prompt(&[Param::Text(TextField::Bio)], &ctx, "English"), 0);
    job.client = LlmClient::new(LlmSettings { api_key: " ".into(), ..LlmSettings::default() }).with_endpoint(&server.url);
    assert_eq!(run_action_blocking(&job).unwrap_err().kind, ActionFailureKind::NoKey);
    assert!(server.requests().is_empty());
}
