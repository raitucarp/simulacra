//! Resizable timeline split: main dialectic stream + focus thread inspector.

use super::callbacks::PostCallbacks;
use super::consts::{TIMELINE_RESIZABLE_ID, TIMELINE_STACKED_ID};
use super::inputs::TimelineInputs;
use super::inspector::{render_inspector_pane, InspectorContext};
use super::stream::render_stream_list;
use super::stream_header::render_stream_header;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::components::{persisted_h_resizable, persisted_v_resizable, SplitPane};
use gpui_kit::{div, AnyElement, Div, IntoElement, ParentElement, Styled};
use std::rc::Rc;

/// Resizable dual feed layout: the stream ("For you" / "Dialectic") beside the focus thread
/// inspector, or above it below the wide layout class.
pub(crate) fn render_timeline<C: IntoElement>(
    state: &AppState,
    theme: &ThemeTokens,
    callbacks: &PostCallbacks,
    inputs: TimelineInputs<C>,
) -> AnyElement {
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
        view: &inputs.inspector,
        on_inspector: inputs.on_inspector.clone(),
        related: &inputs.discovery.related,
    };
    let inspector = render_inspector_pane(&ctx, focused_post);
    let header = render_stream_header(theme, inputs.feed_tab, Rc::new(inputs.on_select_feed_tab), inputs.on_run_command, &inputs.discovery);
    let stream = render_stream_list(inputs.stream, callbacks.clone(), on_load_more, theme);
    let stacked = inputs.layout.stacks_inspector();
    let stream_pane = render_stream_pane(header, inputs.composer, stream, stacked, theme);
    split_panes(stream_pane, inspector, stacked, theme)
}

/// The stream and the inspector side by side, or stacked (each arrangement keeps its sizes).
fn split_panes(stream_pane: Div, inspector: Div, stacked: bool, theme: &ThemeTokens) -> AnyElement {
    if stacked {
        let inspector_range = theme.inspector_stacked_min..theme.inspector_stacked_max;
        return persisted_v_resizable(TIMELINE_STACKED_ID, theme)
            .pane(SplitPane::flex(theme.split_flex_min).child(stream_pane))
            .pane(SplitPane::sized(theme.inspector_stacked_height, inspector_range).child(inspector))
            .into_any_element();
    }
    persisted_h_resizable(TIMELINE_RESIZABLE_ID, theme)
        // PANE 1: Main Dialectic Stream
        .pane(
            SplitPane::sized(theme.stream_pane_width, theme.stream_pane_min_width..theme.stream_pane_max_width)
                .child(stream_pane),
        )
        // PANE 2: Focus Thread & Epistemic Inspector
        .pane(SplitPane::flex(theme.split_flex_min).child(inspector))
        .into_any_element()
}

/// Stream pane: header, composer, and scrollable stream, edged toward the inspector.
fn render_stream_pane<C: IntoElement>(
    header: Div,
    composer: C,
    stream: impl IntoElement,
    stacked: bool,
    theme: &ThemeTokens,
) -> Div {
    let pane = div().w_full().h_full().flex().flex_col().border_color(theme.border_subtle);
    let edged = if stacked { pane.border_b(theme.border_width_hairline) } else { pane.border_r(theme.border_width_hairline) };
    edged
        .child(header)
        .child(composer)
        .child(stream)
}
