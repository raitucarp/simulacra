//! The Search page (Q4): the query field, its filters as chips, then one ranked list of
//! posts, each with the passage that matched and its words marked. ↑ ↓ move through the
//! results and Enter opens the selected one.
//!
//! - [`snippet`]: the passage a result shows and its marks.
//! - [`chips`]: the filter chips and quick filters, and moving the selection.
//! - [`render`]: the page itself.

pub mod chips;
mod render;
mod rows;
pub mod snippet;
#[cfg(test)]
mod tests;

pub use chips::{active_chips, stepped, FilterChip, QUICK_FILTERS};
pub use render::render_search_page;
pub use snippet::{snippet, Snippet};

use crate::domain::search_query::{PostQuery, QueryFilter};
use crate::domain::PostId;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use std::rc::Rc;

/// Everything the Search page asks the shell to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchEvent {
    /// Removes a filter chip (`true`: a negated filter).
    RemoveFilter(QueryFilter, bool),
    /// Adds a quick filter, or removes it when it is set.
    ToggleFilter(QueryFilter),
    /// Moves the selection down (`true`) or up.
    Step(bool),
    /// Opens the selected result.
    Confirm,
    Open(PostId),
}

pub type SearchAction = Rc<dyn Fn(SearchEvent, &mut Window, &mut App) + 'static>;

/// One result as the page shows it.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchRow {
    pub post_id: PostId,
    pub author: String,
    pub handle: String,
    /// The day it was written (`YYYY-MM-DD`, local).
    pub day: String,
    pub snippet: Snippet,
    /// Found by its meaning.
    pub by_meaning: bool,
}

/// What the page shows.
pub struct SearchView<'a> {
    pub input: &'a Entity<InputState>,
    pub query: &'a PostQuery,
    pub rows: &'a [SearchRow],
    pub selected: Option<usize>,
    /// Why meaning did not help this search, when it could not.
    pub note: Option<&'a str>,
    pub on_event: SearchAction,
}
