use super::*;
use crate::agent::embedding::wire::{gemini, local, openai};
use serde_json::json;

const GEMINI_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";

#[test]
fn gemini_001_sends_task_type_title_and_dimensions_with_the_key_in_a_header() {
    let config = config(ProviderKind::Gemini, "gemini-embedding-001", GEMINI_BASE);
    let request = gemini::request(&config, TaskType::RetrievalDocument, &inputs());
    assert_eq!(request.url, format!("{GEMINI_BASE}/gemini-embedding-001:batchEmbedContents"));
    assert_eq!(request.headers, vec![("x-goog-api-key", "test-key-0000".to_string())]);
    assert!(!request.url.contains("key="), "the key never goes in the URL");
    let expected = json!({ "requests": [
        { "model": "models/gemini-embedding-001", "content": { "parts": [{ "text": "God is substance." }] },
          "outputDimensionality": 768, "taskType": "RETRIEVAL_DOCUMENT", "title": "Ethics" },
        { "model": "models/gemini-embedding-001", "content": { "parts": [{ "text": "Is the mind a graph?" }] },
          "outputDimensionality": 768, "taskType": "RETRIEVAL_DOCUMENT" },
    ]});
    assert_eq!(request.body, expected);
    let query = gemini::request(&config, TaskType::RetrievalQuery, &[EmbedInput { title: "ignored".into(), text: "q".into() }]);
    assert_eq!(query.body["requests"][0]["taskType"], "RETRIEVAL_QUERY");
    assert!(query.body["requests"][0].get("title").is_none(), "titles are for documents only");
    for task in [TaskType::SemanticSimilarity, TaskType::Clustering] {
        assert_eq!(gemini::request(&config, task, &inputs()).body["requests"][0]["taskType"], task.wire_name());
    }
}

#[test]
fn newer_gemini_models_and_gemma_read_the_task_from_a_prefix() {
    let config = config(ProviderKind::Gemini, "models/gemini-embedding-2", GEMINI_BASE);
    let request = gemini::request(&config, TaskType::RetrievalDocument, &inputs());
    assert_eq!(request.url, format!("{GEMINI_BASE}/gemini-embedding-2:batchEmbedContents"));
    let first = &request.body["requests"][0];
    assert_eq!(first["content"]["parts"][0]["text"], "title: Ethics | text: God is substance.");
    assert!(first.get("taskType").is_none());
    assert_eq!(request.body["requests"][1]["content"]["parts"][0]["text"], "title: none | text: Is the mind a graph?");
    let query = gemini::request(&config, TaskType::RetrievalQuery, &[EmbedInput::text("free will")]);
    assert_eq!(query.body["requests"][0]["content"]["parts"][0]["text"], "task: search result | query: free will");
    let similar = gemini::request(&config, TaskType::SemanticSimilarity, &[EmbedInput::text("x")]);
    assert_eq!(similar.body["requests"][0]["content"]["parts"][0]["text"], "task: sentence similarity | query: x");
}

#[test]
fn gemini_replies_parse_in_order_and_mismatches_are_errors() {
    let reply = json!({ "embeddings": [{ "values": [0.1, 0.2] }, { "values": [0.3, 0.4] }] });
    assert_eq!(gemini::parse(&reply, 2), Ok(vec![vec![0.1, 0.2], vec![0.3, 0.4]]));
    assert!(gemini::parse(&reply, 3).is_err());
    assert!(gemini::parse(&json!({ "error": {} }), 1).is_err());
    assert!(gemini::parse(&json!({ "embeddings": [{ "values": ["x"] }] }), 1).is_err());
}

#[test]
fn the_local_server_gets_prefixed_gemma_inputs_without_a_key() {
    let mut config = config(ProviderKind::LocalServer, "embeddinggemma", "http://localhost:11434");
    config.api_key = String::new();
    let request = local::request(&config, TaskType::RetrievalQuery, &inputs()[1..]);
    assert_eq!(request.url, "http://localhost:11434/api/embed");
    assert!(request.headers.is_empty());
    let expected = json!({ "model": "embeddinggemma", "input": ["task: search result | query: Is the mind a graph?"], "truncate": true, "dimensions": 768 });
    assert_eq!(request.body, expected);
    config.model = "nomic-embed-text".into();
    assert_eq!(local::request(&config, TaskType::RetrievalQuery, &inputs()[1..]).body["input"][0], "Is the mind a graph?");
    let reply = json!({ "model": "embeddinggemma", "embeddings": [[0.5, 0.5], [1.0, 0.0]] });
    assert_eq!(local::parse(&reply, 2), Ok(vec![vec![0.5, 0.5], vec![1.0, 0.0]]));
    assert!(local::parse(&json!({ "embeddings": [] }), 1).is_err());
}

#[test]
fn openai_compatible_requests_use_a_bearer_and_replies_are_reordered() {
    let config = config(ProviderKind::OpenAiCompatible, "text-embedding-3-small", "https://api.openai.com/v1");
    let request = openai::request(&config, TaskType::RetrievalDocument, &inputs());
    assert_eq!(request.url, "https://api.openai.com/v1/embeddings");
    assert_eq!(request.headers, vec![("Authorization", "Bearer test-key-0000".to_string())]);
    assert_eq!(request.body, json!({ "model": "text-embedding-3-small", "input": ["God is substance.", "Is the mind a graph?"], "dimensions": 768 }));
    let reply = json!({ "data": [{ "index": 1, "embedding": [0.0, 1.0] }, { "index": 0, "embedding": [1.0, 0.0] }] });
    assert_eq!(openai::parse(&reply, 2), Ok(vec![vec![1.0, 0.0], vec![0.0, 1.0]]));
}
