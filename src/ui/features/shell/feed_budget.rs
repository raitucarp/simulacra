//! N4: the finite feed in the shell. The day's tally lives with the feed state and is saved
//! to the settings table whenever it grows; syncing the stream applies the budget to its
//! rows (never while searching) and counts what it showed.

use super::view::AppView;
use crate::domain::finite_feed::{budget_window, FeedDay};
use crate::domain::PostId;
use crate::state::AppState;
use crate::storage::{SettingKey, Vault};
use crate::ui::features::feed::StreamRows;
use std::collections::HashSet;
use std::rc::Rc;

/// The row key of the closing card; no post has it.
const END_ROW_KEY: &str = "\u{0}feed-end";

/// The day's budget and what already counted against it.
pub struct FeedBudget<'a> {
    /// Posts a day (`0`: no budget).
    pub posts: usize,
    pub seen: &'a [PostId],
}

impl FeedBudget<'_> {
    /// The rows of `positions` (display order) under this budget, and the shown posts that
    /// count against it (the user's own posts never do).
    pub fn rows(&self, state: &AppState, positions: Rc<[usize]>, limit: usize, searching: bool) -> (StreamRows, Vec<PostId>) {
        if self.posts == 0 || searching {
            return (StreamRows::new(positions, limit), Vec::new());
        }
        let counts = |post: &crate::domain::Post| !state.author_of(post).is_human_persona();
        let seen: HashSet<PostId> = self.seen.iter().cloned().collect();
        let order = positions.iter().filter_map(|&at| state.posts.get(at).map(|post| (&post.id, at, counts(post))));
        let window = budget_window(order, &seen, self.posts);
        let rows = StreamRows { ended: window.ended, ..StreamRows::new(window.positions.into(), limit) };
        let counted = (0..rows.shown).filter_map(|row| rows.post(state, row)).filter(|post| counts(post) && !seen.contains(&post.id)).map(|post| post.id.clone()).collect();
        (rows, counted)
    }
}

/// The row key of the closing card.
pub fn end_row_key() -> PostId {
    PostId::new(END_ROW_KEY)
}

/// The saved tally (empty when missing or unreadable).
pub fn load_feed_day(vault: Option<&Vault>) -> FeedDay {
    let saved = vault.and_then(|vault| vault.get_setting(SettingKey::FeedDay).ok().flatten());
    saved.and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default()
}

impl AppView {
    /// Today's tally, started afresh on a new day.
    pub(in crate::ui::features::shell) fn feed_day_today(&mut self) {
        let today = crate::time::local_now().date();
        self.feed.day = std::mem::take(&mut self.feed.day).on(today);
    }

    /// Counts `shown` against today's budget, saving the tally when it grew.
    pub(in crate::ui::features::shell) fn count_feed_day(&mut self, shown: Vec<PostId>) {
        if self.feed.day.count(shown) {
            self.save_feed_day();
        }
    }

    pub(in crate::ui::features::shell) fn save_feed_day(&self) {
        let (Some(vault), Ok(json)) = (self.state.db.as_ref(), serde_json::to_string(&self.feed.day)) else { return };
        if let Err(err) = vault.set_setting(SettingKey::FeedDay, &json) {
            eprintln!("simulacra: feed tally not saved: {err}");
        }
    }
}
