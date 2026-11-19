//! "Find similar" (sb B3): the items nearest in meaning to an indexed post or document,
//! computed from the stored vectors without a network call, in a sheet. Items that are not
//! indexed yet (or when meaning is unavailable) fall back to word matches on their text.

use super::super::view::AppView;
use super::semantic_search::{Fallback, RESULT_LIMIT};
use crate::domain::embedding::{similar_to, EmbeddingOwner, OwnerKind};
use crate::domain::PostId;
use crate::state::semantic::hits::shortened;
use crate::state::semantic::{hits_for, lexical_hits, HitTarget};
use crate::ui::features::semantic::SimilarView;
use crate::ui::features::sidebar_left::NavRef;
use gpui_kit::*;

/// Characters of the source excerpt in the sheet header.
const SOURCE_CHARS: usize = 60;
/// Words of the source text used as the word-match query.
const QUERY_WORDS: usize = 48;

impl AppView {
    /// Post "…" menu › Find similar.
    pub(crate) fn find_similar_post(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        self.find_similar(EmbeddingOwner::post(&post_id), window, cx);
    }

    /// Opens the sheet of items similar to `owner` (a post or a vault document). This is the
    /// seam a document page calls for its "Find similar" action.
    pub(crate) fn find_similar(&mut self, owner: EmbeddingOwner, window: &mut Window, cx: &mut Context<Self>) {
        let Some((title, text)) = self.owner_text(&owner) else { return };
        let model = self.embedding_config().model;
        let keep = |candidate: &EmbeddingOwner| candidate != &owner && self.state.is_in_active_network(candidate);
        let ranked = match self.semantic_fallback() {
            None => similar_to(&self.semantic.index, &owner, &model, RESULT_LIMIT, keep),
            Some(_) => Vec::new(),
        };
        let view = if ranked.is_empty() {
            let reason = self.semantic_fallback().unwrap_or(Fallback::NotIndexed);
            let query = text.split_whitespace().take(QUERY_WORDS).collect::<Vec<_>>().join(" ");
            let hits = lexical_hits(&self.state, &query, RESULT_LIMIT + 1, self.chunk_spec()).into_iter().filter(|hit| hit.target.owner() != owner).take(RESULT_LIMIT).collect();
            SimilarView { source: shortened(&title, SOURCE_CHARS), note: Some(reason.note()), hits }
        } else {
            SimilarView { source: shortened(&title, SOURCE_CHARS), note: None, hits: hits_for(&self.state, &ranked, self.chunk_spec()) }
        };
        self.semantic.similar = Some(view);
        window.focus(&self.semantic.sheet_focus, cx);
        cx.notify();
    }

    /// The display title (a post's text, a document's title) and the text of `owner`.
    fn owner_text(&self, owner: &EmbeddingOwner) -> Option<(String, String)> {
        match owner.kind {
            OwnerKind::Post => self.state.get_post_by_id(&owner.id).map(|post| (post.content.clone(), post.content.clone())),
            OwnerKind::Document => self.state.documents.iter().find(|doc| doc.id == owner.id).map(|doc| (doc.title.clone(), doc.content.clone())),
        }
    }

    /// Closes the sheet and gives the keyboard back to the shell.
    pub(crate) fn close_similar_sheet(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.semantic.similar = None;
        window.focus(&self.palette.shell_focus, cx);
        cx.notify();
    }

    /// Opens a hit of the sheet (the sheet closes).
    pub(crate) fn open_similar_hit(&mut self, target: HitTarget, window: &mut Window, cx: &mut Context<Self>) {
        self.close_similar_sheet(window, cx);
        match target {
            HitTarget::Post(id) => self.reveal_post(id, window, cx),
            HitTarget::Document(id) => self.open_nav_item(NavRef::Document(id), window, cx),
        }
    }

    /// The command palette's "Find similar": the thought open in the inspector, else the
    /// palette in semantic mode.
    pub(crate) fn find_similar_to_open_thought(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.feed.selected_thread_post_id.clone() {
            Some(post_id) => self.find_similar_post(post_id, window, cx),
            None => self.open_palette(crate::ui::features::command_palette::PaletteMode::Semantic, window, cx),
        }
    }
}
