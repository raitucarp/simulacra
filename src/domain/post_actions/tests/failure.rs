use super::*;
use crate::domain::preferences::AiPrefs;
use std::time::Duration;

#[test]
fn client_errors_are_classified() {
    let cases = [
        ("API key is not configured. Please open Settings to set your LLM API Key.", ActionFailureKind::NoKey),
        ("API error (429 Too Many Requests): slow down", ActionFailureKind::RateLimited),
        ("Gemini API error (503 Service Unavailable): busy", ActionFailureKind::Server),
        ("API error (401 Unauthorized): bad key", ActionFailureKind::Rejected),
        ("OpenAI-compatible request error: connection refused (os error 10061)", ActionFailureKind::Network),
        ("Failed to parse the reply: EOF", ActionFailureKind::Empty),
        ("The model returned an empty reply", ActionFailureKind::Empty),
        ("something odd", ActionFailureKind::Rejected),
    ];
    for (raw, expected) in cases {
        assert_eq!(ActionFailureKind::classify(raw), expected, "{raw}");
    }
}

#[test]
fn only_transient_failures_are_retried() {
    let retried: Vec<_> = ActionFailureKind::ALL.iter().filter(|kind| kind.is_retryable()).collect();
    assert_eq!(retried, [&ActionFailureKind::Timeout, &ActionFailureKind::Network, &ActionFailureKind::RateLimited, &ActionFailureKind::Server]);
}

#[test]
fn policy_follows_the_ai_preferences_and_backs_off() {
    let prefs = AiPrefs { request_timeout_seconds: 30, max_retries: 3, ..AiPrefs::default() };
    let policy = RequestPolicy::from_prefs(&prefs);
    assert_eq!(policy, RequestPolicy { timeout: Duration::from_secs(30), max_retries: 3 });
    assert!(policy.backoff(1) > policy.backoff(0));
    assert_eq!(policy.backoff(40), policy.backoff(41), "capped");
}
