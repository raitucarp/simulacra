//! N11: transcriptions against the local mock server only (no live provider, no real key): an
//! OpenAI-compatible upload, a local server named by the custom endpoint, Gemini's inline
//! audio, and an engine with no audio input.

use super::transcribe::{run_transcription_blocking, TranscribeJob};
use super::CancelToken;
use crate::agent::llm::transcribe::{transcription_url, AudioClip};
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post_actions::RequestPolicy;
use serde_json::json;
use std::time::Duration;

fn clip() -> AudioClip {
    AudioClip { bytes: b"RIFF fake wave".to_vec(), file_name: "take.wav".into(), mime: "audio/wav".into() }
}

fn job(settings: LlmSettings, endpoint: Option<&str>) -> TranscribeJob {
    let timeout = Duration::from_secs(5);
    let client = LlmClient::new(LlmSettings { api_key: "test-key-not-real".into(), ..settings }).with_timeout(timeout);
    let client = match endpoint {
        Some(url) => client.with_endpoint(url),
        None => client,
    };
    TranscribeJob { client, clip: clip(), policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() }
}

#[test]
fn an_openai_compatible_engine_gets_a_multipart_upload() {
    let server = MockServer::start(vec![MockReply::json(json!({ "text": "Walking home, I thought of entropy." }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, ..LlmSettings::default() };
    let text = run_transcription_blocking(&job(settings, Some(&format!("{}/v1/chat/completions", server.url)))).expect("text");
    assert_eq!(text, "Walking home, I thought of entropy.");
    let sent = &server.requests()[0];
    assert_eq!((sent.method.as_str(), sent.path.as_str()), ("POST", "/v1/audio/transcriptions"));
    assert!(sent.header("content-type").is_some_and(|value| value.starts_with("multipart/form-data; boundary=simulacra-")));
    assert!(sent.body.contains("name=\"model\"\r\n\r\nwhisper-1") && sent.body.contains("filename=\"take.wav\"") && sent.body.contains("RIFF fake wave"));
    assert_eq!(sent.header("authorization"), Some("Bearer test-key-not-real"));
}

#[test]
fn a_local_server_is_reached_through_the_custom_endpoint() {
    let server = MockServer::start(vec![MockReply::json(json!({ "text": "local words" }))]);
    let settings = LlmSettings { provider: LlmProvider::Custom, custom_endpoint: format!("{}/v1/chat/completions", server.url), ..LlmSettings::default() };
    assert_eq!(run_transcription_blocking(&job(settings, None)).expect("text"), "local words");
    assert_eq!(server.requests()[0].path, "/v1/audio/transcriptions");
    assert_eq!(transcription_url("http://localhost:9000/v1/"), "http://localhost:9000/v1/audio/transcriptions");
    assert_eq!(transcription_url("http://h/v1/audio/transcriptions"), "http://h/v1/audio/transcriptions");
}

#[test]
fn gemini_takes_the_audio_inline_and_some_engines_take_none() {
    let server = MockServer::start(vec![MockReply::json(json!({ "candidates": [{ "content": { "parts": [{ "text": "Gemini heard this." }] } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::Gemini, model: "mock-model".into(), ..LlmSettings::default() };
    assert_eq!(run_transcription_blocking(&job(settings, Some(&server.url))).expect("text"), "Gemini heard this.");
    let sent = server.requests()[0].json();
    assert_eq!(server.requests()[0].path, "/mock-model:generateContent");
    assert_eq!(sent["contents"][0]["parts"][0]["inline_data"]["mime_type"], "audio/wav");
    let anthropic = LlmClient::new(LlmSettings { provider: LlmProvider::Anthropic, api_key: "test-key-not-real".into(), ..LlmSettings::default() });
    assert!(!anthropic.can_transcribe());
    let failed = run_transcription_blocking(&job(LlmSettings { provider: LlmProvider::Anthropic, ..LlmSettings::default() }, Some("http://127.0.0.1:9")));
    assert!(failed.is_err(), "nothing is sent to an engine without audio input");
}
