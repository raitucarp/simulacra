//! The AI draft against the local mock server only (no live provider, no real key).

use crate::agent::llm::LlmProvider;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmSettings};
use crate::domain::review::DraftKind;
use crate::ui::features::review::draft_request::{request_drafts, MAX_DRAFT_SOURCE_CHARS};
use serde_json::json;

fn client(url: &str) -> LlmClient {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key".into(), ..LlmSettings::default() };
    LlmClient::new(settings).with_endpoint(url)
}

fn chat_reply(content: &str) -> MockReply {
    MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": content } }] }))
}

#[tokio::test]
async fn a_draft_is_requested_and_parsed() {
    let content = r#"{"cards":[{"type":"qa","question":"What is conatus?","answer":"The striving to persist"},{"type":"cloze","text":"Each thing strives to {{c1::persist}}."}]}"#;
    let server = MockServer::start(vec![chat_reply(content)]);
    let long = "Conatus. ".repeat(MAX_DRAFT_SOURCE_CHARS);
    let drafts = request_drafts(&client(&server.url), &long, "English").await.unwrap();
    assert_eq!(drafts.len(), 2);
    assert!(matches!(&drafts[0], DraftKind::Basic { front, .. } if front == "What is conatus?"));
    let sent = server.requests()[0].json();
    let prompt = sent["messages"].to_string();
    assert!(prompt.contains("write in English"), "{prompt}");
    assert!(prompt.len() < MAX_DRAFT_SOURCE_CHARS * 2, "the source text is capped");
}

#[tokio::test]
async fn failures_and_unusable_replies_become_messages() {
    let server = MockServer::start(vec![chat_reply("I cannot do that."), MockReply::raw(500, "text/plain", "boom")]);
    let unreadable = request_drafts(&client(&server.url), "text", "English").await.unwrap_err();
    assert!(!unreadable.is_empty());
    let failed = request_drafts(&client(&server.url), "text", "English").await.unwrap_err();
    assert!(failed.contains("500"), "{failed}");
    let no_key = LlmClient::new(LlmSettings { api_key: String::new(), ..LlmSettings::default() });
    assert!(!request_drafts(&no_key, "text", "English").await.unwrap_err().is_empty());
}
