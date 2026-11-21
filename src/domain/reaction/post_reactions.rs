//! Every reaction to one post: adding and removing emoji, clapping any number of times, and
//! the totals the reaction bar and the clap button show.
//!
//! Each change returns the row as it now stands (count zero when it was removed), ready to be
//! saved as is.

use super::{Reaction, CLAP};
use crate::domain::{PostId, ProfileId};
use jiff::Timestamp;

/// The reactions to one post, in the order they were first given.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PostReactions {
    rows: Vec<Reaction>,
}

impl PostReactions {
    /// Builds the set from stored rows (rows with a zero count are dropped).
    pub fn from_rows(rows: impl IntoIterator<Item = Reaction>) -> Self {
        Self { rows: rows.into_iter().filter(|row| row.count > 0).collect() }
    }

    /// Every row, in the order they were first given.
    pub fn rows(&self) -> &[Reaction] {
        &self.rows
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn position(&self, profile: &ProfileId, emoji: &str) -> Option<usize> {
        self.rows.iter().position(|row| row.profile_id == *profile && row.emoji == emoji)
    }

    /// Whether `profile` reacted with `emoji` (clapped, for [`CLAP`]).
    pub fn has(&self, profile: &ProfileId, emoji: &str) -> bool {
        self.position(profile, emoji).is_some()
    }

    /// `profile` reacts with `emoji`; an emoji counts once per profile, so adding it again
    /// changes nothing, while adding [`CLAP`] claps once more.
    pub fn add(&mut self, post: &PostId, profile: &ProfileId, emoji: &str, at: Timestamp) -> Option<Reaction> {
        if emoji == CLAP {
            return self.clap(post, profile, 1, at);
        }
        if self.has(profile, emoji) {
            return None;
        }
        let row = Reaction { post_id: post.clone(), profile_id: profile.clone(), emoji: emoji.to_string(), count: 1, updated_at: at };
        self.rows.push(row.clone());
        Some(row)
    }

    /// Takes back `profile`'s reaction with `emoji` (every clap, for [`CLAP`]).
    pub fn remove(&mut self, profile: &ProfileId, emoji: &str, at: Timestamp) -> Option<Reaction> {
        let row = self.rows.remove(self.position(profile, emoji)?);
        Some(Reaction { count: 0, updated_at: at, ..row })
    }

    /// Adds `emoji` when `profile` has not reacted with it, else takes it back.
    pub fn toggle(&mut self, post: &PostId, profile: &ProfileId, emoji: &str, at: Timestamp) -> Option<Reaction> {
        match self.has(profile, emoji) && emoji != CLAP {
            true => self.remove(profile, emoji, at),
            false => self.add(post, profile, emoji, at),
        }
    }

    /// `profile` claps `times` more times (there is no upper limit).
    pub fn clap(&mut self, post: &PostId, profile: &ProfileId, times: u32, at: Timestamp) -> Option<Reaction> {
        if times == 0 {
            return None;
        }
        let Some(index) = self.position(profile, CLAP) else {
            let row = Reaction { post_id: post.clone(), profile_id: profile.clone(), emoji: CLAP.to_string(), count: times, updated_at: at };
            self.rows.push(row.clone());
            return Some(row);
        };
        let row = &mut self.rows[index];
        row.count = row.count.saturating_add(times);
        row.updated_at = at;
        Some(row.clone())
    }

    /// Takes back `times` of `profile`'s claps; the row goes when none are left.
    pub fn unclap(&mut self, profile: &ProfileId, times: u32, at: Timestamp) -> Option<Reaction> {
        let index = self.position(profile, CLAP).filter(|_| times > 0)?;
        let row = &mut self.rows[index];
        row.count = row.count.saturating_sub(times);
        row.updated_at = at;
        let changed = row.clone();
        if changed.count == 0 {
            self.rows.remove(index);
        }
        Some(changed)
    }
}

impl PostReactions {
    /// Replaces the row of the same profile and emoji with `row` (a zero count removes it).
    pub(super) fn put(&mut self, row: Reaction) -> Option<Reaction> {
        match self.position(&row.profile_id, &row.emoji) {
            Some(index) if row.count == 0 => drop(self.rows.remove(index)),
            Some(index) => self.rows[index] = row.clone(),
            None if row.count > 0 => self.rows.push(row.clone()),
            None => return None,
        }
        Some(row)
    }
}
