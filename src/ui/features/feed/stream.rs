//! Main dialectic stream: windowed post selection and the scrollable stream body.

use super::callbacks::{CardMode, PostCallbacks};
use super::consts::{DEFAULT_FEED_WINDOW_SIZE, STREAM_SCROLL_ID};
use super::load_more::render_load_more;
use super::params::LoadMoreCallback;
use super::threadline::compute_stream_threadline_style;
use crate::domain::Post;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, AnyElement, InteractiveElement, IntoElement, ParentElement, Styled};
use rust_i18n::t;
use std::rc::Rc;

/// Windowed slice of the stream to render progressively.
pub(crate) struct StreamWindow<'a> {
    /// Total number of thoughts matching the stream (before windowing).
    pub total: usize,
    /// Candidate posts (window plus one lookahead post when unfiltered).
    pub posts: Vec<&'a Post>,
    /// Number of posts actually rendered.
    pub limit: usize,
}

/// Selects the visible stream for the current network or search query.
pub(crate) fn compute_stream_window<'a>(
    state: &'a AppState,
    search_query: &'a str,
    feed_visible_count: Option<usize>,
) -> StreamWindow<'a> {
    let query_trimmed = search_query.trim();
    let requested_window = feed_visible_count.unwrap_or(DEFAULT_FEED_WINDOW_SIZE);
    let (total, posts): (usize, Vec<&Post>) = if query_trimmed.is_empty() {
        let total = state.current_network_post_count();
        let limit = requested_window.min(total);
        (total, state.current_network_posts().take(limit + 1).collect())
    } else {
        let all_filtered: Vec<_> = state.filtered_posts(query_trimmed).collect();
        (all_filtered.len(), all_filtered)
    };
    StreamWindow { total, posts, limit: requested_window.min(total) }
}

/// Renders windowed stream cards followed by the load-more trigger when more remain.
pub(crate) fn render_stream_elements(
    window: &StreamWindow<'_>,
    state: &AppState,
    theme: &ThemeTokens,
    callbacks: &PostCallbacks,
    on_load_more: Option<Rc<LoadMoreCallback>>,
) -> Vec<AnyElement> {
    let mut elements = Vec::with_capacity(window.limit + 1);
    for (idx, post) in window.posts.iter().take(window.limit).enumerate() {
        // Stream posts render with seamless flat OLED black background without card boxes (R2)
        let style = compute_stream_threadline_style(&window.posts, idx);
        let mode = CardMode { is_focused: false, style, selectable: true };
        elements.push(callbacks.render_post(post, state, theme, mode));
    }
    if window.limit < window.total {
        elements.push(render_load_more(window.limit, window.total, theme, on_load_more));
    }
    elements
}

/// Scrollable stream body with an empty-state fallback.
pub(crate) fn render_stream_scroll(elements: Vec<AnyElement>, theme: &ThemeTokens) -> impl IntoElement {
    let is_empty = elements.is_empty();
    div()
        .id(STREAM_SCROLL_ID)
        .flex_1()
        .overflow_y_scrollbar()
        .flex()
        .flex_col()
        .when(is_empty, |this| {
            this.p(theme.spacing_2xl).items_center().justify_center().child(
                div()
                    .text_size(theme.font_size_sm)
                    .text_color(theme.text_muted)
                    .child(t!("feed.no_posts").to_string()),
            )
        })
        .when(!is_empty, |this| this.children(elements))
}
