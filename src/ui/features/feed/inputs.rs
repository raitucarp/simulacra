//! Splits [`FeedParams`] into the inputs of each feed sub-view.

use super::callbacks::PostCallbacks;
use super::params::{AddDocCallback, DeleteDocCallback, FeedParams, LoadMoreCallback};
use crate::domain::PostId;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::Entity;
use std::rc::Rc;

/// Timeline-only feed inputs.
pub(crate) struct TimelineInputs<'a, C> {
    pub composer: C,
    pub selected_post_id: Option<PostId>,
    pub search_query: &'a str,
    pub feed_visible_count: Option<usize>,
    pub on_load_more: Option<LoadMoreCallback>,
}

/// Inputs of the non-timeline tool views (knowledge vault).
pub(crate) struct ToolViewInputs<'a> {
    pub doc_title_input: &'a Entity<InputState>,
    pub doc_content_input: &'a Entity<TextareaState>,
    pub on_add_document: AddDocCallback,
    pub on_delete_document: DeleteDocCallback,
}

/// Distributes feed parameters to the post callbacks, timeline, and tool views.
pub(crate) fn split_params<C>(params: FeedParams<'_, C>) -> (PostCallbacks, TimelineInputs<'_, C>, ToolViewInputs<'_>) {
    let callbacks = PostCallbacks {
        on_like: Rc::new(params.on_like_post),
        on_reply: Rc::new(params.on_reply_post),
        on_select: Rc::new(params.on_select_post),
        on_repost: Rc::new(params.on_repost_post),
        on_quote: Rc::new(params.on_quote_post),
    };
    let timeline = TimelineInputs {
        composer: params.composer,
        selected_post_id: params.selected_post_id,
        search_query: params.search_query,
        feed_visible_count: params.feed_visible_count,
        on_load_more: params.on_load_more,
    };
    let tools = ToolViewInputs {
        doc_title_input: params.doc_title_input,
        doc_content_input: params.doc_content_input,
        on_add_document: params.on_add_document,
        on_delete_document: params.on_delete_document,
    };
    (callbacks, timeline, tools)
}
