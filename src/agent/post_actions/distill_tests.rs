//! N15: the summary that tops a distilled thread, through the post actions runner against the
//! local mock server only; only the posts that may go to the vault are sent.

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post::distill::prompt::{clean_summary, summary_prompt};
use crate::domain::post_actions::RequestPolicy;
use serde_json::json;
use std::time::Duration;

#[test]
fn the_thread_is_summarised_as_data_and_the_summary_tops_the_document() {
    let server = MockServer::start(vec![MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": "```\nBridges hum when wind sheds vortices.\n```" } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let timeout = Duration::from_secs(5);
    let lines = vec![("Ada".to_string(), "Why do bridges hum?".to_string()), ("Bo".to_string(), "Vortex shedding.".to_string())];
    let job = ActionJob { client: LlmClient::new(settings).with_endpoint(&server.url).with_timeout(timeout), prompt: summary_prompt(&lines, "English"), policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() };
    let answer = run_action_blocking(&job).expect("answer");
    let sent = server.requests()[0].json();
    assert!(sent["messages"][1]["content"].as_str().unwrap_or_default().contains("<thread>\nAda: Why do bridges hum?\n\nBo: Vortex shedding.\n</thread>"));
    assert_eq!(clean_summary(&answer).as_deref(), Some("Bridges hum when wind sheds vortices."));
    let mut state = crate::state::store::AppState::new_in_memory();
    let root = state.submit_post("Why do bridges hum?").expect("root");
    let words = |_| crate::domain::post::distill::DistillWords { summary_heading: "Summary".into(), ..Default::default() };
    let distilled = state.distill_thread(&root, words, clean_summary(&answer).as_deref(), |_| String::new()).expect("distilled");
    let doc = state.documents.iter().find(|doc| doc.id == distilled.doc_id).expect("doc");
    assert!(doc.content.contains("## Summary\n\nBridges hum when wind sheds vortices."));
}
