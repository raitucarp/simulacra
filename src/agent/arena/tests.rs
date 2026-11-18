//! The Arena runner against the local mock server only (no live provider, no real key).

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::post_actions::CancelToken;
use crate::agent::runtime::EnginePlan;
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt, RequestPolicy};
use serde_json::json;
use std::time::Duration;

const TEST_KEY: &str = "test-key-not-real";

fn plan(url: &str, key: &str) -> EnginePlan {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: key.into(), model: "mock-model".into(), ..LlmSettings::default() };
    EnginePlan::from_client(LlmClient::new(settings).with_endpoint(url))
}

fn call(url: &str, retries: u32) -> EngineCall {
    let prompt = ActionPrompt { system: "You are Ada.".into(), user: "DEBATE ARENA. Speak now.".into() };
    EngineCall::new(plan(url, TEST_KEY), prompt, RequestPolicy { timeout: Duration::from_secs(5), max_retries: retries }, CancelToken::new())
}

fn answer(text: &str) -> MockReply {
    MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": text } }] }))
}

#[test]
fn a_turn_goes_through_the_agents_engine_with_its_dossier() {
    let server = MockServer::start(vec![answer("  I argue for the motion.  ")]);
    assert_eq!(run_call_blocking(&call(&server.url, 0)).as_deref(), Ok("I argue for the motion."));
    let body = server.requests()[0].json();
    assert_eq!(body["messages"][0]["content"], "You are Ada.");
    assert!(body.get("tools").is_none_or(|tools| tools.as_array().is_none_or(Vec::is_empty)), "arena calls send no tools");
}

#[test]
fn transient_failures_retry_and_rejections_do_not() {
    let busy = MockReply::raw(503, "application/json", "{\"error\":\"busy\"}");
    let server = MockServer::start(vec![busy, answer("fine")]);
    assert_eq!(run_call_blocking(&call(&server.url, 1)).as_deref(), Ok("fine"));
    let refused = MockServer::start(vec![MockReply::raw(400, "application/json", "{\"error\":\"bad\"}"), answer("never")]);
    let failure = run_call_blocking(&call(&refused.url, 3)).expect_err("rejected");
    assert_eq!(failure.kind, ActionFailureKind::Rejected);
    assert_eq!(refused.requests().len(), 1);
}

#[test]
fn no_key_and_cancel_stop_before_sending() {
    let server = MockServer::start(vec![answer("unused")]);
    let keyless = EngineCall::new(plan(&server.url, ""), ActionPrompt { system: "s".into(), user: "u".into() }, RequestPolicy { timeout: Duration::from_secs(1), max_retries: 0 }, CancelToken::new());
    assert_eq!(run_call_blocking(&keyless).map_err(|f| f.kind), Err(ActionFailureKind::NoKey));
    let cancelled = call(&server.url, 0);
    cancelled.cancel.cancel();
    assert_eq!(run_call_blocking(&cancelled).map_err(|f| f.kind), Err(ActionFailureKind::Cancelled));
    assert!(server.requests().is_empty());
}
