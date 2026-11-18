//! The runner against the local mock server only (no live provider, no real key).

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt, RequestPolicy};
use serde_json::json;
use std::net::TcpListener;
use std::time::{Duration, Instant};

const TEST_KEY: &str = "test-key-not-real";

fn client(url: &str, key: &str) -> LlmClient {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: key.into(), model: "mock-model".into(), ..LlmSettings::default() };
    LlmClient::new(settings).with_endpoint(url)
}

fn job(url: &str, retries: u32, timeout: Duration) -> ActionJob {
    ActionJob {
        client: client(url, TEST_KEY).with_timeout(timeout),
        prompt: ActionPrompt { system: "system rules".into(), user: "Task: steelman\n<thread>x</thread>".into() },
        policy: RequestPolicy { timeout, max_retries: retries },
        cancel: CancelToken::new(),
    }
}

fn answer(text: &str) -> MockReply {
    MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": text } }] }))
}

/// An address that accepts connections but never answers.
fn silent_endpoint() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let url = format!("http://{}", listener.local_addr().expect("addr"));
    (listener, url)
}

#[test]
fn a_reply_comes_back_with_both_prompts_sent() {
    let server = MockServer::start(vec![answer("## Strongest version\nIt holds.")]);
    let text = run_action_blocking(&job(&server.url, 0, Duration::from_secs(5))).expect("answer");
    assert_eq!(text, "## Strongest version\nIt holds.");
    let body = server.requests()[0].json();
    let messages = body["messages"].as_array().expect("messages").clone();
    assert_eq!(messages[0]["content"], "system rules");
    assert!(messages[1]["content"].as_str().unwrap_or_default().contains("<thread>x</thread>"));
}

#[test]
fn transient_failures_are_retried_then_succeed() {
    let busy = MockReply::raw(503, "application/json", "{\"error\":\"busy\"}");
    let server = MockServer::start(vec![busy, answer("fine")]);
    assert_eq!(run_action_blocking(&job(&server.url, 1, Duration::from_secs(5))).as_deref(), Ok("fine"));
    assert_eq!(server.requests().len(), 2);
}

#[test]
fn retries_stop_at_the_limit_and_rejections_are_not_retried() {
    let busy = || MockReply::raw(429, "application/json", "{}");
    let server = MockServer::start(vec![busy(), busy()]);
    let failure = run_action_blocking(&job(&server.url, 1, Duration::from_secs(5))).unwrap_err();
    assert_eq!(failure.kind, ActionFailureKind::RateLimited);
    assert_eq!(server.requests().len(), 2);

    let denied = MockServer::start(vec![MockReply::raw(401, "application/json", format!("{{\"error\":\"{TEST_KEY} invalid\"}}"))]);
    let failure = run_action_blocking(&job(&denied.url, 3, Duration::from_secs(5))).unwrap_err();
    assert_eq!(failure.kind, ActionFailureKind::Rejected);
    assert!(!failure.detail.contains(TEST_KEY), "the key is redacted: {}", failure.detail);
    assert_eq!(denied.requests().len(), 1);
}

#[test]
fn a_blank_reply_is_empty() {
    let server = MockServer::start(vec![answer("   ")]);
    let failure = run_action_blocking(&job(&server.url, 0, Duration::from_secs(5))).unwrap_err();
    assert_eq!(failure.kind, ActionFailureKind::Empty);
}

#[test]
fn a_silent_provider_times_out() {
    let (_listener, url) = silent_endpoint();
    let started = Instant::now();
    let failure = run_action_blocking(&job(&url, 0, Duration::from_millis(300))).unwrap_err();
    assert!(matches!(failure.kind, ActionFailureKind::Timeout | ActionFailureKind::Network), "{failure:?}");
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[test]
fn cancelling_stops_a_waiting_request() {
    let (_listener, url) = silent_endpoint();
    let job = job(&url, 3, Duration::from_secs(30));
    let cancel = job.cancel.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(150));
        cancel.cancel();
    });
    let started = Instant::now();
    let failure = run_action_blocking(&job).unwrap_err();
    assert_eq!(failure.kind, ActionFailureKind::Cancelled);
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[test]
fn no_key_fails_without_a_request() {
    let server = MockServer::start(vec![answer("never")]);
    let job = ActionJob { client: client(&server.url, "  "), ..job(&server.url, 2, Duration::from_secs(5)) };
    assert_eq!(run_action_blocking(&job).unwrap_err().kind, ActionFailureKind::NoKey);
    assert!(server.requests().is_empty());
}
