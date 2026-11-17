//! Resizable timeline split: main dialectic stream + focus thread inspector.

use super::callbacks::PostCallbacks;
use super::consts::TIMELINE_RESIZABLE_ID;
use super::inputs::TimelineInputs;
use super::inspector::{render_inspector_pane, InspectorContext};
use super::stream::render_stream_list;
use super::stream_header::render_stream_header;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::components::{persisted_h_resizable, SplitPane};
use gpui_kit::{div, Div, IntoElement, ParentElement, Styled};
use std::rc::Rc;

/// Resizable dual feed layout: left stream ("For you" / "Dialectic") + right focus thread inspector.
pub(crate) fn render_timeline<C: IntoElement>(
    state: &AppState,
    theme: &ThemeTokens,
    callbacks: &PostCallbacks,
    inputs: TimelineInputs<C>,
) -> impl IntoElement {
    // Focused post (or the top of the feed), resolved to the original of an instant repost.
    let focused_post = inputs
        .selected_post_id
        .as_ref()
        .and_then(|id| state.get_post(id))
        .or_else(|| inputs.stream.rows.post(state, 0))
        .or_else(|| state.posts.first())
        .and_then(|post| state.get_post(&state.action_target(&post.id)));
    let on_load_more = inputs.on_load_more.map(Rc::new);
    let ctx = InspectorContext {
        state,
        theme,
        callbacks,
        expanded: inputs.inspector_expanded,
        on_toggle_replies: Rc::new(inputs.on_toggle_replies),
    };
    let inspector = render_inspector_pane(&ctx, focused_post);
    let header = render_stream_header(theme, inputs.feed_tab, Rc::new(inputs.on_select_feed_tab));
    let stream = render_stream_list(inputs.stream, callbacks.clone(), on_load_more, theme);

    persisted_h_resizable(TIMELINE_RESIZABLE_ID, theme)
        // PANE 1: Main Dialectic Stream
        .pane(
            SplitPane::sized(theme.stream_pane_width, theme.stream_pane_min_width..theme.stream_pane_max_width)
                .child(render_stream_pane(header, inputs.composer, stream, theme)),
        )
        // PANE 2: Focus Thread & Epistemic Inspector
        .pane(SplitPane::flex(theme.split_flex_min).child(inspector))
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
