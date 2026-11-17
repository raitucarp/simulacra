use super::{backoff_delay, FailureCause, FailureGate, BACKOFF_BASE_MINUTES, BACKOFF_MAX_MINUTES};
use crate::agent::{LlmFailure, LlmProvider};
use chrono::{DateTime, Duration, TimeZone, Utc};

fn at(minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 16, 12, minute, 0).single().expect("valid time")
}

fn missing_key(provider: LlmProvider) -> FailureCause {
    FailureCause::of(&LlmFailure::new(provider, "API key is not configured.", "", true))
}

fn request(raw: &str) -> FailureCause {
    FailureCause::of(&LlmFailure::new(LlmProvider::OpenAi, raw, "", false))
}

#[test]
fn the_same_cause_is_noticed_once_until_reset() {
    let mut gate = FailureGate::default();
    assert!(gate.notice(missing_key(LlmProvider::OpenAi)), "first time is shown");
    for _ in 0..5 {
        assert!(!gate.notice(missing_key(LlmProvider::OpenAi)), "repeats are suppressed");
    }
    gate.reset();
    assert!(gate.notice(missing_key(LlmProvider::OpenAi)), "shown again after a reset");
}

#[test]
fn distinct_causes_each_get_one_notice() {
    let mut gate = FailureGate::default();
    assert!(gate.notice(missing_key(LlmProvider::OpenAi)));
    assert!(gate.notice(missing_key(LlmProvider::Gemini)), "another provider is another cause");
    assert!(gate.notice(request("API error (401 Unauthorized): bad key")));
    assert!(gate.notice(request("API error (429 Too Many Requests): slow down")));
    assert!(!gate.notice(missing_key(LlmProvider::OpenAi)));
}

#[test]
fn request_causes_ignore_the_variable_details() {
    let first = request("API error (500 Internal Server Error): {\"id\":\"req_1\"}");
    let second = request("API error (500 Internal Server Error): {\"id\":\"req_2\"}");
    assert_eq!(first, second);
    assert_ne!(first, request("OpenAI-compatible request error: connection refused"));
    assert_ne!(first, missing_key(LlmProvider::OpenAi));
    assert_eq!(missing_key(LlmProvider::Groq), FailureCause::missing_key(LlmProvider::Groq));
}

#[test]
fn backoff_doubles_per_failure_and_is_capped() {
    assert_eq!(backoff_delay(1), Duration::minutes(BACKOFF_BASE_MINUTES));
    assert_eq!(backoff_delay(2), Duration::minutes(BACKOFF_BASE_MINUTES * 2));
    assert_eq!(backoff_delay(3), Duration::minutes(BACKOFF_BASE_MINUTES * 4));
    assert_eq!(backoff_delay(40), Duration::minutes(BACKOFF_MAX_MINUTES));
    assert_eq!(backoff_delay(u32::MAX), Duration::minutes(BACKOFF_MAX_MINUTES));
    let delays: Vec<Duration> = (1..12).map(backoff_delay).collect();
    assert!(delays.windows(2).all(|pair| pair[0] <= pair[1]), "never shrinks");
}

#[test]
fn failures_pause_generation_until_the_backoff_elapses() {
    let mut gate = FailureGate::default();
    assert!(gate.allows_generation(at(0)), "a fresh gate never blocks");
    assert!(gate.record_failure(request("API error (401 Unauthorized): x"), at(0)), "first failure is shown");
    let base = BACKOFF_BASE_MINUTES as u32;
    assert!(!gate.allows_generation(at(base - 1)));
    assert!(gate.allows_generation(at(base)));

    assert!(!gate.record_failure(request("API error (401 Unauthorized): y"), at(base)), "repeat is quiet");
    assert_eq!(gate.streak(), 2);
    assert!(!gate.allows_generation(at(base + base * 2 - 1)), "second pause is twice as long");
    assert!(gate.allows_generation(at(base + base * 2)));
}

#[test]
fn reset_clears_the_backoff_and_the_streak() {
    let mut gate = FailureGate::default();
    gate.record_failure(request("API error (503 Service Unavailable): z"), at(0));
    assert!(!gate.allows_generation(at(1)));
    gate.reset();
    assert!(gate.allows_generation(at(1)));
    assert_eq!(gate.streak(), 0);
}

#[test]
fn gates_back_off_per_engine_key() {
    use super::FailureGates;
    let mut gates = FailureGates::default();
    assert!(gates.allows_generation("cred_a", at(0)));
    assert!(gates.gate_mut("cred_a").record_failure(request("API error (401)"), at(0)));
    assert!(!gates.allows_generation("cred_a", at(1)), "the failing key backs off");
    assert!(gates.allows_generation("cred_b", at(1)), "other keys keep going");
    assert_eq!(gates.gate("cred_a").map(|g| g.streak()), Some(1));
    gates.reset("cred_a");
    assert!(gates.allows_generation("cred_a", at(1)) && gates.gate("cred_a").is_none());
    gates.gate_mut("x").record_failure(request("boom"), at(0));
    gates.reset_all();
    assert!(gates.allows_generation("x", at(0)));
}
