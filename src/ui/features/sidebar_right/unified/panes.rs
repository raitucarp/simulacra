//! Tab content panes of the unified right sidebar.

use super::super::dot_graph::{render_mini_graph, OpenGraphCallback};
use super::super::heatmap::{render_clickable_heatmap, ClickDayCallback, HeatmapParams};
use super::super::spark_feed::render_random_spark_feed_with_lookup;
use super::super::trending::{render_popular_tags_title, render_trending_list};
use crate::domain::activity::ActivityHeatmap;
use crate::domain::Post;
use crate::state::graph::GraphScene;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use super::super::knowledge_card::{render_knowledge_card, ImportDocumentCallback};
use super::super::roster::render_network_agent_roster;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;
use std::rc::Rc;

/// Inputs for the Mind & Cadence pane.
pub(super) struct MindPaneParams<'a> {
    pub state: &'a AppState,
    pub posts: &'a [Post],
    pub heatmap: &'a ActivityHeatmap,
    pub selected_day: Option<jiff::civil::Date>,
    pub on_click_day: ClickDayCallback,
    pub graph: Rc<GraphScene>,
    pub on_open_graph: OpenGraphCallback,
}

fn padded_row(theme: &ThemeTokens) -> Div {
    div().w_full().px(theme.panel_padding_compact)
}

/// Dot graph, cadence heatmap and spark feed.
pub(super) fn render_mind_cadence_pane(params: MindPaneParams<'_>, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(padded_row(theme).child(render_mini_graph(params.graph, params.on_open_graph, theme)))
        .child(padded_row(theme).child(render_clickable_heatmap(
            HeatmapParams {
                heatmap: params.heatmap,
                selected: params.selected_day,
                on_click_day: params.on_click_day,
            },
            theme,
        )))
        .child(render_random_spark_feed_with_lookup(params.posts, params.state, theme))
}

fn divided_block(theme: &ThemeTokens) -> Div {
    div().w_full().pb(theme.stack_gap).border_b(theme.border_width_hairline).border_color(theme.border)
}

fn render_search_box(search_input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_card)
        .p(theme.spacing_xs)
        .child(Input::new(search_input).cleanable(true))
}

/// Vector search, knowledge card, popular tags and network telemetry.
pub(super) fn render_vault_telemetry_pane(
    state: &AppState,
    search_input: &Entity<InputState>,
    (trending, on_import): (Vec<AnyElement>, ImportDocumentCallback),
    theme: &ThemeTokens,
) -> impl IntoElement {
    let doc_count = state.current_documents().count();
    div()
        .w_full()
        .flex_1()
        .min_h_0()
        .overflow_y_scrollbar()
        .px(theme.panel_padding_compact)
        .pb(theme.panel_padding_compact)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(render_search_box(search_input, theme))
        .child(divided_block(theme).child(render_knowledge_card(doc_count, on_import, theme)))
        .child(
            divided_block(theme)
                .flex()
                .flex_col()
                .gap(theme.stack_gap)
                .child(render_popular_tags_title(theme))
                .child(render_trending_list(trending, theme)),
        )
        .child(render_network_agent_roster(&state.current_network_agents(), theme))
}
