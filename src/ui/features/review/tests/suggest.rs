//! The AI suggestion against the local mock server only (no live provider, no real key).

use crate::agent::llm::LlmProvider;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmSettings};
use crate::domain::review::AnchorSide;
use crate::ui::features::review::suggest_request::{request_suggestion, MAX_SUGGEST_SOURCE_CHARS};
use serde_json::json;

fn client(url: &str) -> LlmClient {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key".into(), ..LlmSettings::default() };
    LlmClient::new(settings).with_endpoint(url)
}

fn chat_reply(content: &str) -> MockReply {
    MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": content } }] }))
}

#[tokio::test]
async fn a_question_is_suggested_for_a_post() {
    let server = MockServer::start(vec![chat_reply(r#"{"text":"What does every thing strive for?"}"#)]);
    let long = "Conatus. ".repeat(MAX_SUGGEST_SOURCE_CHARS);
    let question = request_suggestion(&client(&server.url), &long, AnchorSide::Back, "English").await.unwrap();
    assert_eq!(question, "What does every thing strive for?");
    let prompt = server.requests()[0].json()["messages"].to_string();
    assert!(prompt.contains("write in English") && prompt.contains("question"), "{prompt}");
    assert!(prompt.len() < MAX_SUGGEST_SOURCE_CHARS * 2, "the source text is capped");
}

#[tokio::test]
async fn failures_and_unusable_replies_become_messages() {
    let server = MockServer::start(vec![chat_reply("I cannot do that."), MockReply::raw(500, "text/plain", "boom")]);
    let unreadable = request_suggestion(&client(&server.url), "text", AnchorSide::Front, "English").await.unwrap_err();
    assert!(!unreadable.is_empty());
    let failed = request_suggestion(&client(&server.url), "text", AnchorSide::Front, "English").await.unwrap_err();
    assert!(failed.contains("500"), "{failed}");
    let no_key = LlmClient::new(LlmSettings { api_key: String::new(), ..LlmSettings::default() });
    assert!(!request_suggestion(&no_key, "text", AnchorSide::Back, "English").await.unwrap_err().is_empty());
}
