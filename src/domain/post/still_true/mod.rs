//! N13: "Still true?". The user's own claims come back for a check once they have stood a
//! while: confirm (it comes back later, the wait doubling each time), revise (an edit, a new
//! version, starts the wait again) or retract (N14). The rule is pure: [`due_at`] says when a
//! post is due from when it was written, last edited and last confirmed ([`StillTrue`],
//! `extras.check`); [`checkable`] says which posts are checked at all.

use super::Post;
use jiff::{SignedDuration, Timestamp};
use serde::{Deserialize, Serialize};

/// The first wait, in days, before a post is asked about.
pub const FIRST_WAIT_DAYS: i64 = 30;
/// The longest wait, in days, however often it was confirmed.
pub const LONGEST_WAIT_DAYS: i64 = 365;
const SECONDS_PER_DAY: i64 = 86_400;
/// The doubling stops growing after this many confirmations (2^4 × 30 days is past a year).
const MAX_DOUBLINGS: u8 = 4;

/// The last check of a post (`extras.check`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StillTrue {
    /// When it was last confirmed.
    pub confirmed_at: Timestamp,
    /// How many times in a row it was confirmed.
    pub streak: u8,
}

/// The wait after `streak` confirmations: 30, 60, 120, 240 days, then a year.
pub fn wait_days(streak: u8) -> i64 {
    (FIRST_WAIT_DAYS << streak.min(MAX_DOUBLINGS)).min(LONGEST_WAIT_DAYS)
}

/// Whether `post` is a claim to check at all: written with words, not locked, not a repost,
/// a feeling or a question asked of the room, not already retracted, and not archived.
pub fn checkable(post: &Post) -> bool {
    !post.is_locked()
        && !post.is_repost()
        && !post.content.trim().is_empty()
        && post.kind.is_standard()
        && post.extras.question.is_none()
        && post.extras.correction.as_ref().is_none_or(|c| c.kind != super::correction::CorrectionKind::Retracted)
        && !post.lifecycle.is_archived()
}

/// When `post` was last made to stand as it is: its last edit, or when it was written.
fn stood_since(post: &Post) -> Timestamp {
    post.extras.head.as_ref().and_then(|head| head.edited_at).unwrap_or(post.created_at).max(post.created_at)
}

/// When `post` is due for its next check (`None`: it is not checked).
pub fn due_at(post: &Post) -> Option<Timestamp> {
    if !checkable(post) {
        return None;
    }
    let stood = stood_since(post);
    let (since, streak) = match post.extras.check.as_deref() {
        Some(check) if check.confirmed_at >= stood => (check.confirmed_at, check.streak),
        _ => (stood, 0),
    };
    since.checked_add(SignedDuration::from_secs(wait_days(streak) * SECONDS_PER_DAY)).ok()
}

/// Whether `post` is due for a check at `now`.
pub fn is_due(post: &Post, now: Timestamp) -> bool {
    due_at(post).is_some_and(|due| due <= now)
}

/// The check after confirming `post` at `now` (the streak goes on unless an edit came between).
pub fn confirmed(post: &Post, now: Timestamp) -> StillTrue {
    let stood = stood_since(post);
    let streak = post.extras.check.as_deref().filter(|check| check.confirmed_at >= stood).map_or(0, |check| check.streak.saturating_add(1));
    StillTrue { confirmed_at: now, streak: streak.max(1) }
}

#[cfg(test)]
mod tests;
