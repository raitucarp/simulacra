//! Splits [`FeedParams`] into the inputs of each feed sub-view.

use super::callbacks::PostCallbacks;
use super::inspector_outline::{InspectorCallback, InspectorView};
use super::params::{FeedDiscovery, FeedParams, LoadMoreCallback, RunCommandCallback, SelectFeedTabCallback, ToggleRepliesCallback};
use crate::domain::PostId;
use crate::state::FeedTab;
use super::stream::StreamInputs;
use crate::ui::features::post::BodyLinks;
use crate::ui::responsive::LayoutClass;
use std::rc::Rc;

/// Timeline-only feed inputs.
pub(crate) struct TimelineInputs<C> {
    pub composer: C,
    pub selected_post_id: Option<PostId>,
    pub feed_tab: FeedTab,
    pub inspector_expanded: bool,
    pub layout: LayoutClass,
    pub stream: StreamInputs,
    pub on_load_more: Option<LoadMoreCallback>,
    pub on_select_feed_tab: SelectFeedTabCallback,
    pub on_toggle_replies: ToggleRepliesCallback,
    pub on_run_command: RunCommandCallback,
    pub inspector: InspectorView,
    pub on_inspector: InspectorCallback,
    pub discovery: FeedDiscovery,
}

/// Distributes feed parameters to the post callbacks and the timeline.
pub(crate) fn split_params<C>(params: FeedParams<'_, C>) -> (PostCallbacks, TimelineInputs<C>) {
    let callbacks = PostCallbacks {
        on_like: Rc::new(params.on_like_post),
        on_reply: Rc::new(params.on_reply_post),
        on_select: Rc::new(params.on_select_post),
        on_repost: Rc::new(params.on_repost_post),
        on_quote: Rc::new(params.on_quote_post),
        on_delete: Rc::new(params.on_delete_post),
        on_find_similar: Rc::new(params.on_find_similar_post),
        on_learn: Rc::new(params.on_learn_post),
        learning: params.learning_posts,
        on_post_action: Rc::new(params.on_post_action),
        credence: params.credence,
        data: params.data,
        reactions: params.reactions,
        links: BodyLinks {
            on_tag: Some(params.on_open_tag),
            on_wikilink: Some(params.on_open_wikilink),
            on_source: Some(params.on_open_source),
            on_media: Some(params.on_open_media),
            scope: String::new(),
        },
    };
    let timeline = TimelineInputs {
        composer: params.composer,
        selected_post_id: params.selected_post_id,
        feed_tab: params.feed_tab,
        inspector_expanded: params.inspector_expanded,
        layout: params.layout,
        stream: params.stream,
        on_load_more: params.on_load_more,
        on_select_feed_tab: params.on_select_feed_tab,
        on_toggle_replies: params.on_toggle_replies,
        on_run_command: params.on_run_command,
        inspector: params.inspector,
        on_inspector: params.on_inspector, discovery: params.discovery,
    };
    (callbacks, timeline)
}
