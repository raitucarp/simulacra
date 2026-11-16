//! Resizable timeline split: main dialectic stream + focus thread inspector.

use super::callbacks::PostCallbacks;
use super::consts::{
    STREAM_PANE_DEFAULT_WIDTH, STREAM_PANE_MAX_WIDTH, STREAM_PANE_MIN_WIDTH, TIMELINE_RESIZABLE_ID,
};
use super::inputs::TimelineInputs;
use super::inspector::{render_inspector_pane, InspectorContext};
use super::stream::{compute_stream_window, render_stream_elements, render_stream_scroll};
use super::stream_header::render_stream_header;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use gpui_kit::component::resizable::{h_resizable, resizable_panel};
use gpui_kit::{div, px, Div, IntoElement, ParentElement, Styled};
use std::rc::Rc;

/// Resizable dual feed layout: left stream ("For you" / "Dialectic") + right focus thread inspector.
pub(crate) fn render_timeline<C: IntoElement>(
    state: &AppState,
    theme: &ThemeTokens,
    callbacks: &PostCallbacks,
    inputs: TimelineInputs<'_, C>,
) -> impl IntoElement {
    let window = compute_stream_window(state, inputs.feed_tab, inputs.search_query, inputs.feed_visible_count);
    // Determine currently focused post (or fallback to top/first post in feed) via O(1) lookup
    let focused_post = inputs
        .selected_post_id
        .as_ref()
        .and_then(|id| state.get_post(id))
        .or_else(|| window.posts.first().copied())
        .or_else(|| state.posts.first());
    let on_load_more = inputs.on_load_more.map(Rc::new);
    let stream_elements = render_stream_elements(&window, state, theme, callbacks, on_load_more);
    let ctx = InspectorContext {
        state,
        theme,
        callbacks,
        expanded: inputs.inspector_expanded,
        on_toggle_replies: Rc::new(inputs.on_toggle_replies),
    };
    let inspector = render_inspector_pane(&ctx, focused_post);
    let header = render_stream_header(theme, inputs.feed_tab, Rc::new(inputs.on_select_feed_tab));
    let stream = render_stream_scroll(stream_elements, inputs.stream_scroll, theme);

    h_resizable(TIMELINE_RESIZABLE_ID)
        // PANE 1: Main Dialectic Stream
        .child(
            resizable_panel()
                .size(px(STREAM_PANE_DEFAULT_WIDTH))
                .size_range(px(STREAM_PANE_MIN_WIDTH)..px(STREAM_PANE_MAX_WIDTH))
                .child(render_stream_pane(header, inputs.composer, stream, theme)),
        )
        // PANE 2: Focus Thread & Epistemic Inspector
        .child(resizable_panel().child(inspector))
}

/// Stream pane: header, composer, and scrollable stream.
fn render_stream_pane<C: IntoElement>(
    header: Div,
    composer: C,
    stream: impl IntoElement,
    theme: &ThemeTokens,
) -> Div {
    div()
        .w_full()
        .h_full()
        .flex()
        .flex_col()
        .border_r(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(header)
        .child(composer)
        .child(stream)
}
