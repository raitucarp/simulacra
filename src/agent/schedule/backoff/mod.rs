//! Deduplication and backoff of LLM failures, so a broken setup (no API key, a revoked key,
//! an unreachable endpoint) raises one notice per distinct cause instead of one per heartbeat,
//! and autonomous generation waits longer after each consecutive failure.

use crate::agent::{LlmFailure, LlmProvider};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashSet;

/// Pause after the first failure of a streak.
pub const BACKOFF_BASE_MINUTES: i64 = 2;
/// Longest pause, however long the streak.
pub const BACKOFF_MAX_MINUTES: i64 = 60;
/// Separates a provider error's stable label (`"API error (401 Unauthorized)"`) from its
/// variable details (response body, request ids).
const DETAIL_SEPARATOR: char = ':';

/// What went wrong, coarse enough that repeats of the same problem compare equal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailureCause {
    /// Stable provider id (`LlmProvider::id_str`).
    provider: &'static str,
    kind: FailureKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FailureKind {
    /// No API key is configured.
    MissingKey,
    /// The request failed; the error's label without its details.
    Request(String),
}

impl FailureCause {
    /// The cause of `failure`.
    pub fn of(failure: &LlmFailure) -> Self {
        let kind = if failure.missing_key {
            FailureKind::MissingKey
        } else {
            let label = failure.message.split(DETAIL_SEPARATOR).next().unwrap_or_default();
            FailureKind::Request(label.trim().to_string())
        };
        Self { provider: failure.provider.id_str(), kind }
    }

    /// No API key is configured for `provider`.
    pub fn missing_key(provider: LlmProvider) -> Self {
        Self { provider: provider.id_str(), kind: FailureKind::MissingKey }
    }
}

/// The pause before autonomous generation retries after `streak` consecutive failures:
/// [`BACKOFF_BASE_MINUTES`] doubling per failure, capped at [`BACKOFF_MAX_MINUTES`].
pub fn backoff_delay(streak: u32) -> Duration {
    let doublings = streak.saturating_sub(1).min(i64::BITS - 2);
    let minutes = BACKOFF_BASE_MINUTES.saturating_mul(1_i64 << doublings);
    Duration::minutes(minutes.min(BACKOFF_MAX_MINUTES))
}

/// Failure bookkeeping between two resets (a settings change or a successful call).
#[derive(Debug, Clone, Default)]
pub struct FailureGate {
    /// Causes whose notice was already shown.
    shown: HashSet<FailureCause>,
    /// Consecutive failed generations.
    streak: u32,
    /// Autonomous generation waits until this instant.
    retry_at: Option<DateTime<Utc>>,
}

impl FailureGate {
    /// Remembers `cause`; `true` the first time it is seen since the last reset, i.e. when
    /// its notice should be shown.
    pub fn notice(&mut self, cause: FailureCause) -> bool {
        self.shown.insert(cause)
    }

    /// Records a failed generation at `now`: extends the backoff and returns whether the
    /// failure's notice should be shown (see [`Self::notice`]).
    pub fn record_failure(&mut self, cause: FailureCause, now: DateTime<Utc>) -> bool {
        self.streak = self.streak.saturating_add(1);
        self.retry_at = Some(now + backoff_delay(self.streak));
        self.notice(cause)
    }

    /// Whether autonomous generation may run at `now` (no backoff pending).
    pub fn allows_generation(&self, now: DateTime<Utc>) -> bool {
        self.retry_at.is_none_or(|at| now >= at)
    }

    /// Consecutive failed generations since the last reset.
    pub fn streak(&self) -> u32 {
        self.streak
    }

    /// Forgets every shown notice and the backoff (settings changed or a call succeeded).
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests;
