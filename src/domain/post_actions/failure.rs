//! Why an action produced no answer, read from the LLM client's error text, and which of
//! those reasons are worth another attempt. Also the request policy (timeout, retries,
//! backoff) taken from Settings › AI & Models.

use crate::domain::document::keyed::keyed_enum;
use crate::domain::preferences::AiPrefs;
use std::time::Duration;

keyed_enum! {
    /// The reason an action failed; the key names its locale message.
    ActionFailureKind {
        /// No API key is configured.
        NoKey => "no_key",
        /// Today's AI budget is spent.
        Budget => "budget",
        /// The request took longer than the configured timeout.
        Timeout => "timeout",
        /// The provider could not be reached.
        Network => "network",
        /// Rate limit or quota (HTTP 408 / 429).
        RateLimited => "rate_limited",
        /// The provider had a server error (HTTP 5xx).
        Server => "server",
        /// The provider refused the request (bad key, unknown model, bad request).
        Rejected => "rejected",
        /// The reply was empty or unreadable.
        Empty => "empty",
        /// The user stopped it.
        Cancelled => "cancelled",
    }
}

const NO_KEY_MARK: &str = "API key is not configured";
const NETWORK_MARK: &str = "request error";
const PARSE_MARKS: &[&str] = &["parse", "empty", "decode"];
const STATUS_OPEN: &str = "(";
const RATE_LIMIT_CODES: &[u16] = &[408, 429];
const SERVER_ERROR_CLASS: u16 = 5;
const STATUS_CLASS_DIVISOR: u16 = 100;

impl ActionFailureKind {
    /// Whether another attempt may succeed.
    pub const fn is_retryable(self) -> bool {
        matches!(self, Self::Timeout | Self::Network | Self::RateLimited | Self::Server)
    }

    /// Classifies a raw error of `LlmClient::generate`.
    pub fn classify(raw: &str) -> Self {
        if raw.contains(NO_KEY_MARK) {
            return Self::NoKey;
        }
        let lower = raw.to_lowercase();
        if lower.contains(NETWORK_MARK) {
            return Self::Network;
        }
        if let Some(status) = http_status(raw) {
            return match status {
                code if RATE_LIMIT_CODES.contains(&code) => Self::RateLimited,
                code if code / STATUS_CLASS_DIVISOR == SERVER_ERROR_CLASS => Self::Server,
                _ => Self::Rejected,
            };
        }
        if PARSE_MARKS.iter().any(|mark| lower.contains(mark)) {
            Self::Empty
        } else {
            Self::Rejected
        }
    }
}

/// The HTTP status of an "API error (429 Too Many Requests): …" message.
fn http_status(raw: &str) -> Option<u16> {
    raw.split(STATUS_OPEN).skip(1).find_map(|part| {
        let digits: String = part.chars().take_while(char::is_ascii_digit).collect();
        (digits.len() == 3).then(|| digits.parse().ok()).flatten()
    })
}

/// How long one attempt may take and how often it is retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestPolicy {
    pub timeout: Duration,
    pub max_retries: u32,
}

/// Wait before the first retry, and the longest wait.
const BASE_BACKOFF: Duration = Duration::from_millis(800);
const MAX_BACKOFF: Duration = Duration::from_secs(20);

impl RequestPolicy {
    pub fn from_prefs(ai: &AiPrefs) -> Self {
        Self { timeout: Duration::from_secs(u64::from(ai.request_timeout_seconds)), max_retries: ai.max_retries }
    }

    /// The wait before retry number `attempt` (0-based), doubling up to a cap.
    pub fn backoff(&self, attempt: u32) -> Duration {
        BASE_BACKOFF.saturating_mul(2_u32.saturating_pow(attempt.min(16))).min(MAX_BACKOFF)
    }
}
