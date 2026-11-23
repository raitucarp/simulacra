//! N5: a rendering through the post actions runner, against the local mock server only (no
//! live provider, no real key), kept as a version of the post.

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::data::history::PostRecord;
use crate::domain::post::depth::prompt::{clean_rendering, rendering_prompt};
use crate::domain::post::depth::Depth;
use crate::domain::post_actions::{ActionContext, AuthorContext, RequestPolicy, ThreadLine};
use crate::state::store::AppState;
use serde_json::json;
use std::time::Duration;

fn job(url: &str, prompt: crate::domain::post_actions::ActionPrompt) -> ActionJob {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let timeout = Duration::from_secs(5);
    ActionJob { client: LlmClient::new(settings).with_endpoint(url).with_timeout(timeout), prompt, policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() }
}

#[test]
fn a_simpler_rendering_comes_back_and_is_kept_as_a_version() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("Entropy measures how many microstates share one macrostate.").expect("posted");
    let text = state.get_post(&id).expect("post").content.clone();
    let context = ActionContext { thought: ThreadLine::new("Ada", "@ada", &text, 0), author: AuthorContext::default(), ancestors: Vec::new(), replies: Vec::new(), reader_language: crate::i18n::Language::English };
    let server = MockServer::start(vec![MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": "\"A messy room has more ways to be messy than tidy.\"" } }] }))]);
    let answer = run_action_blocking(&job(&server.url, rendering_prompt(Depth::Simpler, &context))).expect("answer");
    let sent = server.requests()[0].json();
    assert!(sent["messages"][1]["content"].as_str().unwrap_or_default().contains("microstates"), "the thought is sent as data");
    let rendering = clean_rendering(&answer).expect("text");
    let kept = state.keep_rendering(&id, Depth::Simpler, &rendering, jiff::Timestamp::now()).expect("kept");
    assert_eq!(kept.content, "A messy room has more ways to be messy than tidy.");
    assert_eq!(PostRecord::parse(&kept.record_json).and_then(|r| r.rendering), Some(Depth::Simpler));
    assert_eq!(state.get_post(&id).expect("post").content, text, "the post still shows what was written");
}
