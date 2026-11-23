//! N17: drafting a definition through the post actions runner, against the local mock server
//! only (no live provider, no real key); the post is sent as quoted data.

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post::glossary::prompt::{clean_draft, definition_prompt};
use crate::domain::post_actions::RequestPolicy;
use serde_json::json;
use std::time::Duration;

#[test]
fn a_definition_is_drafted_from_the_post_as_data() {
    let server = MockServer::start(vec![MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": "\"How many ways a state can be arranged.\"" } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let timeout = Duration::from_secs(5);
    let post = crate::domain::Post::create("p", "n1", "u1", "Entropy is why rooms get messy.").expect("post");
    let job = ActionJob { client: LlmClient::new(settings).with_endpoint(&server.url).with_timeout(timeout), prompt: definition_prompt("entropy", &post, "English"), policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() };
    let answer = run_action_blocking(&job).expect("answer");
    let sent = server.requests()[0].json();
    assert!(sent["messages"][1]["content"].as_str().unwrap_or_default().contains("<note>\nEntropy is why rooms get messy.\n</note>"));
    assert_eq!(clean_draft(&answer).as_deref(), Some("How many ways a state can be arranged."));
}
