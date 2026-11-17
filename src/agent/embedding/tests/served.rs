//! The client end to end against the local mock HTTP server (no real provider).

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::embedding::{cosine, FailureKind};
use serde_json::json;

const JSON: &str = "application/json";

fn gemini_reply(vectors: &[[f32; 2]]) -> MockReply {
    MockReply::json(json!({ "embeddings": vectors.iter().map(|v| json!({ "values": v })).collect::<Vec<_>>() }))
}

#[tokio::test]
async fn gemini_batches_return_unit_vectors_in_order() {
    let server = MockServer::start(vec![gemini_reply(&[[3.0, 4.0], [0.0, 2.0]])]);
    let client = EmbeddingClient::new(config(ProviderKind::Gemini, "gemini-embedding-001", &server.url));
    let vectors = client.embed(TaskType::RetrievalDocument, &inputs()).await.expect("embedded");
    assert_eq!(vectors, vec![vec![0.6, 0.8], vec![0.0, 1.0]]);
    let request = &server.requests()[0];
    assert_eq!(request.path, "/gemini-embedding-001:batchEmbedContents");
    assert_eq!(request.header("x-goog-api-key"), Some("test-key-0000"));
    assert_eq!(request.json()["requests"].as_array().map(Vec::len), Some(2));
}

#[tokio::test]
async fn a_rate_limit_is_retried_after_the_hinted_delay() {
    let limited = MockReply::raw(429, JSON, r#"{"error":{"status":"RESOURCE_EXHAUSTED","details":[{"retryDelay":"0s"}]}}"#);
    let server = MockServer::start(vec![limited, gemini_reply(&[[1.0, 0.0], [0.0, 1.0]])]);
    let client = EmbeddingClient::new(config(ProviderKind::Gemini, "gemini-embedding-001", &server.url));
    assert!(client.embed(TaskType::RetrievalDocument, &inputs()).await.is_ok());
    assert_eq!(server.requests().len(), 2);
}

#[tokio::test]
async fn a_rejected_key_fails_at_once_with_a_clear_kind() {
    let rejected = MockReply::raw(400, JSON, r#"{"error":{"message":"API key not valid.","details":[{"reason":"API_KEY_INVALID"}]}}"#);
    let server = MockServer::start(vec![rejected]);
    let client = EmbeddingClient::new(config(ProviderKind::Gemini, "gemini-embedding-001", &server.url));
    let error = client.embed(TaskType::RetrievalQuery, &inputs()).await.unwrap_err();
    assert_eq!((error.kind, error.status), (FailureKind::BadKey, Some(400)));
    assert_eq!(server.requests().len(), 1, "a bad key is not retried");
}

#[tokio::test]
async fn an_unknown_local_model_and_an_unreachable_server_are_classified() {
    let missing = MockReply::raw(404, JSON, r#"{"error":"model \"embeddinggemma\" not found, try pulling it first"}"#);
    let server = MockServer::start(vec![missing]);
    let client = EmbeddingClient::new(config(ProviderKind::LocalServer, "embeddinggemma", &server.url));
    let error = client.embed(TaskType::RetrievalQuery, &inputs()).await.unwrap_err();
    assert_eq!(error.kind, FailureKind::ModelNotFound);
    assert_eq!(server.requests()[0].path, "/api/embed");
    let unreachable = EmbeddingConfig { max_retries: 0, ..config(ProviderKind::LocalServer, "embeddinggemma", "http://127.0.0.1:9") };
    let error = EmbeddingClient::new(unreachable).embed(TaskType::RetrievalQuery, &inputs()).await.unwrap_err();
    assert_eq!(error.kind, FailureKind::Network);
}

#[tokio::test]
async fn the_mock_provider_is_offline_and_meaningful() {
    let client = EmbeddingClient::new(EmbeddingConfig::mock(64));
    let texts = [EmbedInput::text("spinoza substance ethics"), EmbedInput::text("ethics of spinoza"), EmbedInput::text("rocket engines")];
    let vectors = client.embed(TaskType::RetrievalDocument, &texts).await.expect("offline");
    assert!(vectors.iter().all(|v| v.len() == 64));
    assert!(cosine(&vectors[0], &vectors[1]) > cosine(&vectors[0], &vectors[2]), "shared words are closer");
    let missing = EmbeddingConfig { api_key: String::new(), ..config(ProviderKind::Gemini, "gemini-embedding-001", "http://127.0.0.1:9") };
    let error = EmbeddingClient::new(missing).embed(TaskType::RetrievalQuery, &texts).await.unwrap_err();
    assert_eq!(error.kind, FailureKind::MissingKey, "nothing is sent without a key");
}
