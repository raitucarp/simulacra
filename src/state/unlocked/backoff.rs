//! Wrong-password backoff (X7): a few wrong passwords for a post are free; after that each
//! further attempt has to wait, the wait doubling with every wrong password up to a cap. The
//! right password clears it. Kept in memory for the session, per post.

use crate::domain::PostId;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Wrong passwords in a row before attempts have to wait.
pub const FREE_ATTEMPTS: u32 = 3;
/// The first wait, doubled after every further wrong password.
pub const FIRST_WAIT: Duration = Duration::from_secs(5);
/// The longest wait.
pub const MAX_WAIT: Duration = Duration::from_secs(5 * 60);

/// One post's wrong passwords in a row and when the next attempt may be made.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Backoff {
    failures: u32,
    until: Option<Instant>,
}

/// The wait after `failures` wrong passwords in a row (`None` while still free).
pub fn wait_after(failures: u32) -> Option<Duration> {
    let doublings = failures.checked_sub(FREE_ATTEMPTS)?;
    let factor = 1u32.checked_shl(doublings).unwrap_or(u32::MAX);
    Some(FIRST_WAIT.saturating_mul(factor).min(MAX_WAIT))
}

/// Every post's backoff.
#[derive(Debug, Default)]
pub struct UnlockAttempts {
    posts: HashMap<PostId, Backoff>,
}

impl UnlockAttempts {
    /// How long an attempt on `post` still has to wait at `now`; `None` when it may go now.
    pub fn wait(&self, post: &PostId, now: Instant) -> Option<Duration> {
        let until = self.posts.get(post)?.until?;
        let left = until.saturating_duration_since(now);
        (!left.is_zero()).then_some(left)
    }

    /// A wrong password for `post` at `now`; returns the wait it now has, if any.
    pub fn failed(&mut self, post: &PostId, now: Instant) -> Option<Duration> {
        let backoff = self.posts.entry(post.clone()).or_default();
        backoff.failures = backoff.failures.saturating_add(1);
        let wait = wait_after(backoff.failures);
        backoff.until = wait.map(|wait| now + wait);
        wait
    }

    /// The right password for `post`: its backoff is cleared.
    pub fn succeeded(&mut self, post: &PostId) {
        self.posts.remove(post);
    }
}
