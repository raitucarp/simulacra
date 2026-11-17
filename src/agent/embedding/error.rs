//! Classified embedding errors: a [`FailureKind`] for the user-facing notice, plus a short,
//! secret-free detail and the provider's retry hint.

use crate::agent::llm::redact_secrets;
use crate::domain::embedding::FailureKind;
use std::time::Duration;

/// Most characters of provider detail kept.
const MAX_DETAIL_CHARS: usize = 160;
const ELLIPSIS: char = '…';
/// Body fragments of a rejected key (Gemini answers some with 400).
const BAD_KEY_MARKERS: [&str; 4] = ["API_KEY_INVALID", "API key not valid", "API key expired", "PERMISSION_DENIED"];
/// Body fragments of an unknown model.
const MODEL_MARKERS: [&str; 3] = ["is not found", "not found", "is not supported for"];
const MODEL_WORD: &str = "model";
/// Gemini's `RetryInfo` field: `"retryDelay": "30s"`.
const RETRY_DELAY_FIELD: &str = "\"retryDelay\"";
const SECONDS_SUFFIX: char = 's';

/// Why a request failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddingError {
    pub kind: FailureKind,
    /// HTTP status, when the server answered.
    pub status: Option<u16>,
    /// Short provider message with every key redacted.
    pub detail: String,
    /// How long the provider asked to wait before retrying.
    pub retry_after: Option<Duration>,
}

impl EmbeddingError {
    /// An error that never reached a provider.
    pub fn local(kind: FailureKind) -> Self {
        Self { kind, status: None, detail: String::new(), retry_after: None }
    }

    /// A transport failure (unreachable host, timeout, refused connection).
    pub fn network(raw: &str, secrets: &[String]) -> Self {
        Self { detail: clean(raw, secrets), ..Self::local(FailureKind::Network) }
    }

    /// A reply that could not be read.
    pub fn unreadable(raw: &str, secrets: &[String]) -> Self {
        Self { detail: clean(raw, secrets), ..Self::local(FailureKind::InvalidResponse) }
    }

    /// A non-success HTTP reply.
    pub fn from_status(status: u16, body: &str, retry_after_header: Option<&str>, secrets: &[String]) -> Self {
        let retry_after = retry_after_header.and_then(|v| v.trim().parse::<u64>().ok()).map(Duration::from_secs).or_else(|| retry_delay(body));
        Self { kind: classify(status, body), status: Some(status), detail: clean(body, secrets), retry_after }
    }
}

/// The failure kind of HTTP `status` with reply `body`.
pub fn classify(status: u16, body: &str) -> FailureKind {
    let mentions = |markers: &[&str]| markers.iter().any(|marker| body.contains(marker));
    match status {
        401 | 403 => FailureKind::BadKey,
        400 if mentions(&BAD_KEY_MARKERS) => FailureKind::BadKey,
        404 if body.to_lowercase().contains(MODEL_WORD) || mentions(&MODEL_MARKERS) => FailureKind::ModelNotFound,
        400 if body.to_lowercase().contains(MODEL_WORD) && mentions(&MODEL_MARKERS) => FailureKind::ModelNotFound,
        408 | 429 => FailureKind::Quota,
        500..=599 => FailureKind::Server,
        _ => FailureKind::InvalidResponse,
    }
}

/// Gemini's `"retryDelay": "30s"` hint, if the body carries one.
fn retry_delay(body: &str) -> Option<Duration> {
    let after = &body[body.find(RETRY_DELAY_FIELD)? + RETRY_DELAY_FIELD.len()..];
    let value: String = after.chars().skip_while(|c| *c != '"').skip(1).take_while(|c| *c != '"').collect();
    let seconds: f64 = value.strip_suffix(SECONDS_SUFFIX)?.parse().ok()?;
    (seconds.is_finite() && seconds >= 0.0).then(|| Duration::from_secs_f64(seconds))
}

/// One line, secrets redacted, shortened.
fn clean(raw: &str, secrets: &[String]) -> String {
    let line = redact_secrets(raw, secrets).split_whitespace().collect::<Vec<_>>().join(" ");
    match line.char_indices().nth(MAX_DETAIL_CHARS) {
        Some((cut, _)) => format!("{}{ELLIPSIS}", &line[..cut]),
        None => line,
    }
}
