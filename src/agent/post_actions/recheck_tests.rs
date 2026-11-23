//! N13: asking whether a claim still holds, through the post actions runner against the local
//! mock server only (no live provider, no real key); a locked claim is never checked.

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post::still_true::checkable;
use crate::domain::post::still_true::prompt::{clean_verdict, recheck_prompt};
use crate::domain::post_actions::RequestPolicy;
use serde_json::json;
use std::time::Duration;

#[test]
fn a_claim_is_rechecked_as_data_and_the_verdict_kept_short() {
    let server = MockServer::start(vec![MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": "OUTDATED:\n the record was broken in 2025." } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let timeout = Duration::from_secs(5);
    let job = ActionJob { client: LlmClient::new(settings).with_endpoint(&server.url).with_timeout(timeout), prompt: recheck_prompt("The marathon record is 2:01:09.", "2024-01-02", "English"), policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() };
    let answer = run_action_blocking(&job).expect("answer");
    let sent = server.requests()[0].json();
    assert!(sent["messages"][1]["content"].as_str().unwrap_or_default().contains("<claim>\nThe marathon record is 2:01:09.\n</claim>"));
    assert_eq!(clean_verdict(&answer).as_deref(), Some("OUTDATED: the record was broken in 2025."));
    let mut locked = crate::domain::Post::create("l", "n1", "u1", "secret claim").expect("post");
    locked.lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    assert!(!checkable(&locked), "a locked claim is never sent");
}
