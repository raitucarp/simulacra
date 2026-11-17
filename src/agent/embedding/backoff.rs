//! Retry timing: exponential backoff, or the provider's own hint when it gives one.

use std::time::Duration;

/// Wait before the first retry.
const BASE_DELAY: Duration = Duration::from_millis(1_000);
/// Longest wait between attempts.
const MAX_DELAY: Duration = Duration::from_secs(60);

/// The wait before retry number `attempt` (0-based): the provider's hint, else
/// `BASE_DELAY · 2^attempt`, never above [`MAX_DELAY`].
pub fn retry_delay(attempt: u32, hint: Option<Duration>) -> Duration {
    let doubled = BASE_DELAY.saturating_mul(2_u32.saturating_pow(attempt.min(16)));
    hint.unwrap_or(doubled).min(MAX_DELAY)
}
