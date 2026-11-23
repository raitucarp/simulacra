//! The virtualized main stream's state: cached listing, `ListState` and the row identities
//! used to splice only what changed (so scroll position survives new posts and load-more).

use super::consts::STREAM_LIST_OVERDRAW;
use super::feed_budget::FeedBudget;
use super::view::AppView;
use crate::domain::PostId;
use crate::state::{AppState, FeedRanking, FeedTab, StreamCache};
use crate::theme::ThemeTokens;
use crate::ui::features::feed::{FeedReader, StreamInputs, StreamRows};
use gpui_kit::{px, App, Entity, ListAlignment, ListOffset, ListState, Pixels};
use std::rc::Rc;

/// Identity of one stream row: a post, or the trailing load-more row (`None`).
type RowKey = Option<PostId>;

/// Cached rows of the main stream and the list state that virtualizes them.
pub struct StreamList {
    cache: StreamCache,
    list: ListState,
    rows: StreamRows,
    keys: Vec<RowKey>,
}

impl Default for StreamList {
    fn default() -> Self {
        Self {
            cache: StreamCache::default(),
            list: ListState::new(0, ListAlignment::Top, px(STREAM_LIST_OVERDRAW)),
            rows: StreamRows::default(),
            keys: Vec::new(),
        }
    }
}

/// `(start, old_end, new_len)` of the smallest range replacing `old` by `new`.
fn changed_range(old: &[RowKey], new: &[RowKey]) -> (usize, usize, usize) {
    let prefix = old.iter().zip(new).take_while(|(a, b)| a == b).count();
    let max_suffix = old.len().min(new.len()) - prefix;
    let suffix = old.iter().rev().zip(new.iter().rev()).take(max_suffix).take_while(|(a, b)| a == b).count();
    (prefix, old.len() - suffix, new.len() - prefix - suffix)
}

impl StreamList {
    /// Brings the rows up to date with the store (ordered by `ranking` when given), splicing
    /// the list where rows changed.
    /// Under a daily `budget` (N4, never while searching) only what it allows is listed; returns
    /// the shown posts that count against it.
    pub fn sync(&mut self, state: &AppState, tab: FeedTab, query: &str, limit: usize, ranking: Option<&FeedRanking>, budget: FeedBudget<'_>) -> Vec<PostId> {
        let (positions, rebuilt) = self.cache.ranked_positions(state, tab, query, ranking);
        let (rows, counted) = budget.rows(state, positions, limit, !query.trim().is_empty());
        if !rebuilt && rows == self.rows {
            return counted;
        }
        let keys: Vec<RowKey> = (0..rows.row_count()).map(|row| rows.post(state, row).map(|post| post.id.clone()).or_else(|| rows.ends_with_card().then(super::feed_budget::end_row_key))).collect();
        let (start, old_end, count) = changed_range(&self.keys, &keys);
        if start != old_end || count > 0 {
            self.list.splice(start..old_end, count);
        }
        self.rows = rows;
        self.keys = keys;
        counted
    }

    /// The rows as of the last [`Self::sync`].
    pub fn rows(&self) -> &StreamRows {
        &self.rows
    }

    /// Scrolls so that `row` sits at the top of the stream.
    pub fn scroll_to_row(&self, row: usize) {
        self.list.scroll_to(ListOffset { item_ix: row, offset_in_item: Pixels::ZERO });
    }

    /// Forgets measured row heights (after a theme change resizes the cards).
    pub fn remeasure(&self) {
        self.list.remeasure();
    }

    /// How many times the listing was recomputed.
    pub fn builds(&self) -> usize {
        self.cache.builds()
    }

    /// Inputs of the feed's virtualized stream, reading the store through `view`.
    pub fn inputs(&self, view: &Entity<AppView>) -> StreamInputs {
        StreamInputs { list: self.list.clone(), rows: self.rows.clone(), reader: reader(view.clone()) }
    }
}

fn read_view<'a>(view: &Entity<AppView>, cx: &'a App) -> (&'a AppState, &'a ThemeTokens) {
    let this = view.read(cx);
    (&this.state, &this.theme)
}

fn reader(view: Entity<AppView>) -> FeedReader {
    Rc::new(move |cx| read_view(&view, cx))
}

#[cfg(test)]
mod tests {
    use super::{changed_range, RowKey};
    use crate::domain::PostId;

    fn keys(ids: &[&str]) -> Vec<RowKey> {
        ids.iter().map(|id| (!id.is_empty()).then(|| PostId::new(*id))).collect()
    }

    #[test]
    fn head_insert_and_load_more_splice_only_the_new_rows() {
        assert_eq!(changed_range(&keys(&["a", "b", ""]), &keys(&["n", "a", "b", ""])), (0, 0, 1));
        assert_eq!(changed_range(&keys(&["a", "b", ""]), &keys(&["a", "b", "c", "d", ""])), (2, 2, 2));
        assert_eq!(changed_range(&keys(&["a", "b"]), &keys(&["a", "b"])), (2, 2, 0));
        assert_eq!(changed_range(&keys(&["a", "x", "b"]), &keys(&["a", "b"])), (1, 2, 0));
    }
}
