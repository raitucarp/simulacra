use super::*;
use crate::agent::embedding::backoff::retry_delay;
use crate::agent::embedding::error::classify;
use crate::domain::embedding::FailureKind;

#[test]
fn statuses_and_bodies_map_to_failure_kinds() {
    let bad_key = r#"{"error":{"code":400,"message":"API key not valid. Please pass a valid API key.","status":"INVALID_ARGUMENT","details":[{"reason":"API_KEY_INVALID"}]}}"#;
    assert_eq!(classify(400, bad_key), FailureKind::BadKey);
    assert_eq!(classify(401, ""), FailureKind::BadKey);
    assert_eq!(classify(403, "PERMISSION_DENIED"), FailureKind::BadKey);
    let missing_model = r#"{"error":{"code":404,"message":"models/embeddinggemma is not found for API version v1beta, or is not supported for embedContent."}}"#;
    assert_eq!(classify(404, missing_model), FailureKind::ModelNotFound);
    assert_eq!(classify(404, r#"{"error":"model \"x\" not found, try pulling it first"}"#), FailureKind::ModelNotFound);
    assert_eq!(classify(429, r#"{"error":{"status":"RESOURCE_EXHAUSTED"}}"#), FailureKind::Quota);
    assert_eq!(classify(503, "overloaded"), FailureKind::Server);
    assert_eq!(classify(400, r#"{"error":{"message":"Invalid JSON payload"}}"#), FailureKind::InvalidResponse);
    assert_eq!(classify(404, "<html>nothing here</html>"), FailureKind::InvalidResponse);
}

#[test]
fn only_rate_limits_servers_and_networks_are_retried() {
    let retryable: Vec<FailureKind> = [
        FailureKind::MissingKey, FailureKind::BadKey, FailureKind::Quota, FailureKind::Budget, FailureKind::Network,
        FailureKind::ModelNotFound, FailureKind::Server, FailureKind::InvalidResponse,
    ]
    .into_iter()
    .filter(|kind| kind.is_retryable())
    .collect();
    assert_eq!(retryable, vec![FailureKind::Quota, FailureKind::Network, FailureKind::Server]);
}

#[test]
fn errors_redact_the_key_and_read_retry_hints() {
    let secrets = ["test-key-0000".to_string()];
    let body = r#"{"error":{"message":"bad test-key-0000 at ?key=test-key-0000","details":[{"retryDelay": "12s"}]}}"#;
    let error = EmbeddingError::from_status(429, body, None, &secrets);
    assert!(!error.detail.contains("test-key-0000"), "{}", error.detail);
    assert_eq!(error.retry_after, Some(Duration::from_secs(12)));
    assert_eq!(error.status, Some(429));
    let header = EmbeddingError::from_status(429, "", Some("7"), &secrets);
    assert_eq!(header.retry_after, Some(Duration::from_secs(7)));
    let long = EmbeddingError::network(&"x".repeat(500), &secrets);
    assert!(long.detail.chars().count() <= 161 && long.kind == FailureKind::Network);
}

#[test]
fn backoff_doubles_honours_hints_and_is_capped() {
    assert_eq!(retry_delay(0, None), Duration::from_secs(1));
    assert_eq!(retry_delay(1, None), Duration::from_secs(2));
    assert_eq!(retry_delay(3, None), Duration::from_secs(8));
    assert_eq!(retry_delay(30, None), Duration::from_secs(60));
    assert_eq!(retry_delay(0, Some(Duration::from_secs(5))), Duration::from_secs(5));
    assert_eq!(retry_delay(0, Some(Duration::from_secs(600))), Duration::from_secs(60));
}
