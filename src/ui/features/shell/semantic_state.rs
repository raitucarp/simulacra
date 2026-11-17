//! What the shell keeps for semantic search (sb B3): the in-memory vector index, the
//! background indexing queue, today's spend, the open "find similar" sheet and the debounce
//! generation of palette queries.

use crate::domain::embedding::{DailyUsage, FailureKind, IndexQueue, VectorIndex};
use crate::state::semantic::IndexKey;
use crate::ui::features::semantic::SimilarView;
use gpui_kit::{App, FocusHandle};

/// Semantic search state of the window.
pub struct SemanticState {
    /// Every stored vector (loaded from the vault on the first render).
    pub index: VectorIndex,
    pub queue: IndexQueue,
    /// Today's embedding requests and tokens (the daily AI budget).
    pub usage: DailyUsage,
    pub(super) loaded: bool,
    /// The collections and configuration the last plan was made for.
    pub(super) watched: Option<IndexKey>,
    /// The next plan re-embeds everything (Settings › Rebuild the index).
    pub(super) force_next_plan: bool,
    /// The failure the user was last told about (not repeated until it changes).
    pub(super) reported: Option<FailureKind>,
    /// The paused badge was closed.
    pub(super) badge_dismissed: bool,
    /// The open "find similar" sheet.
    pub similar: Option<SimilarView>,
    pub sheet_focus: FocusHandle,
    /// Bumped on every palette keystroke in semantic mode; only the latest query runs.
    pub(super) query_generation: u64,
}

impl SemanticState {
    pub(super) fn new(cx: &mut App) -> Self {
        Self {
            index: VectorIndex::default(),
            queue: IndexQueue::default(),
            usage: DailyUsage::default(),
            loaded: false,
            watched: None,
            force_next_plan: false,
            reported: None,
            badge_dismissed: false,
            similar: None,
            sheet_focus: cx.focus_handle(),
            query_generation: 0,
        }
    }
}
