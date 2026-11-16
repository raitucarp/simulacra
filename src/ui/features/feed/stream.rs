//! Main dialectic stream: windowed post selection and the scrollable stream body.

use super::callbacks::{CardMode, PostCallbacks};
use super::consts::{DEFAULT_FEED_WINDOW_SIZE, STREAM_SCROLL_ID};
use super::load_more::render_load_more;
use super::params::LoadMoreCallback;
use super::threadline::compute_stream_threadline_style;
use crate::domain::Post;
use crate::state::{feed_entries, AppState, FeedTab};
use crate::theme::ThemeTokens;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, AnyElement, InteractiveElement, IntoElement, ParentElement, ScrollHandle,
    StatefulInteractiveElement, Styled,
};
use rust_i18n::t;
use std::rc::Rc;

/// Windowed slice of the stream to render progressively.
pub(crate) struct StreamWindow<'a> {
    /// Total number of thoughts matching the stream (before windowing).
    pub total: usize,
    /// Every matching post in display order (conversations grouped when not searching).
    pub posts: Vec<&'a Post>,
    /// Number of posts actually rendered.
    pub limit: usize,
}

/// Selects the visible stream: the tab's grouped conversations, or flat search matches.
pub(crate) fn compute_stream_window<'a>(
    state: &'a AppState,
    tab: FeedTab,
    search_query: &'a str,
    feed_visible_count: Option<usize>,
) -> StreamWindow<'a> {
    let query_trimmed = search_query.trim();
    let requested_window = feed_visible_count.unwrap_or(DEFAULT_FEED_WINDOW_SIZE);
    let posts: Vec<&Post> = if query_trimmed.is_empty() {
        feed_entries(state, tab)
    } else {
        state.filtered_posts(query_trimmed).collect()
    };
    let total = posts.len();
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
        // Rigid rows keep each card's full height inside the scrolling column.
        let card = callbacks.render_post(post, state, theme, mode);
        elements.push(div().w_full().flex_shrink_0().child(card).into_any_element());
    }
    if window.limit < window.total {
        elements.push(render_load_more(window.limit, window.total, theme, on_load_more));
    }
    elements
}

/// Scrollable stream body with an empty-state fallback. The cards are the direct children of
/// the area tracked by `scroll`, so `scroll.scroll_to_item(i)` targets the i-th stream entry.
pub(crate) fn render_stream_scroll(
    elements: Vec<AnyElement>,
    scroll: &ScrollHandle,
    theme: &ThemeTokens,
) -> impl IntoElement {
    let is_empty = elements.is_empty();
    let area = div()
        .id(STREAM_SCROLL_ID)
        .absolute()
        .inset_0()
        .overflow_y_scroll()
        .track_scroll(scroll)
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
        .when(!is_empty, |this| this.children(elements));
    div()
        .relative()
        .flex_1()
        .w_full()
        .overflow_hidden()
        .child(area)
        .vertical_scrollbar(scroll)
}
