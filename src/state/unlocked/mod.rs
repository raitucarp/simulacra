//! The session's unlocked posts (X4): the opened text of each post the user unlocked, kept
//! in memory only (never stored, logged or handed to agents) and zeroized when it is
//! dropped. Everything locks again after [`RELOCK_AFTER`] without user activity, when the
//! user locks a post again, and when the app quits.

use crate::domain::PostId;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

/// Idle time after which every unlocked post locks again.
pub const RELOCK_AFTER: Duration = Duration::from_secs(5 * 60);

/// The opened texts and when the user was last active.
pub struct UnlockedPosts {
    texts: HashMap<PostId, Rc<Zeroizing<String>>>,
    last_activity: Instant,
    idle: Duration,
}

impl Default for UnlockedPosts {
    fn default() -> Self {
        Self::new(RELOCK_AFTER, Instant::now())
    }
}

impl UnlockedPosts {
    /// Nothing unlocked; everything locks again after `idle` without activity.
    pub fn new(idle: Duration, now: Instant) -> Self {
        Self { texts: HashMap::new(), last_activity: now, idle }
    }

    /// Keeps `text` as the opened text of `post` (the unlock is itself activity).
    pub fn insert(&mut self, post: PostId, text: Zeroizing<String>, now: Instant) {
        self.texts.insert(post, Rc::new(text));
        self.last_activity = now;
    }

    /// The opened text of `post`, while it is unlocked.
    pub fn text(&self, post: &PostId) -> Option<&str> {
        self.texts.get(post).map(|text| text.as_str())
    }

    /// Every opened text, shared (not copied) with the cards that show them.
    pub fn texts(&self) -> HashMap<PostId, Rc<Zeroizing<String>>> {
        self.texts.clone()
    }

    pub fn is_unlocked(&self, post: &PostId) -> bool {
        self.texts.contains_key(post)
    }

    /// How many posts are unlocked.
    pub fn len(&self) -> usize {
        self.texts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.texts.is_empty()
    }

    /// The user did something at `now`.
    pub fn touch(&mut self, now: Instant) {
        self.last_activity = self.last_activity.max(now);
    }

    /// Locks `post` again; `true` when it was unlocked.
    pub fn relock(&mut self, post: &PostId) -> bool {
        self.texts.remove(post).is_some()
    }

    /// Locks everything again.
    pub fn relock_all(&mut self) {
        self.texts.clear();
    }

    /// Locks everything again when the user has been idle for the idle time at `now`;
    /// `true` when anything was locked.
    pub fn relock_if_idle(&mut self, now: Instant) -> bool {
        if self.texts.is_empty() || now.saturating_duration_since(self.last_activity) < self.idle {
            return false;
        }
        self.relock_all();
        true
    }
}

#[cfg(test)]
mod tests;
