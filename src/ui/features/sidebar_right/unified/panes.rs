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
use super::super::knowledge_card::render_knowledge_card;
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
    pub selected_day: Option<chrono::NaiveDate>,
    pub on_click_day: ClickDayCallback,
    pub graph: Rc<GraphScene>,
    pub on_open_graph: OpenGraphCallback,
}

fn padded_row(theme: &ThemeTokens) -> Div {
    div().w_full().px(theme.spacing_md)
}

/// Dot graph, cadence heatmap and spark feed.
pub(super) fn render_mind_cadence_pane(params: MindPaneParams<'_>, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
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
    div().w_full().pb_3p5().border_b_1().border_color(theme.border)
}

fn render_search_box(search_input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .rounded_lg()
        .border_1()
        .border_color(theme.border)
        .bg(theme.bg_card)
        .p_1()
        .child(Input::new(search_input).cleanable(true))
}

/// Vector search, knowledge card, popular tags and network telemetry.
pub(super) fn render_vault_telemetry_pane(
    state: &AppState,
    search_input: &Entity<InputState>,
    trending: Vec<AnyElement>,
    theme: &ThemeTokens,
) -> impl IntoElement {
    div()
        .w_full()
        .flex_1()
        .min_h_0()
        .overflow_y_scrollbar()
        .px(theme.spacing_md)
        .pb(theme.spacing_md)
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(render_search_box(search_input, theme))
        .child(divided_block(theme).child(render_knowledge_card(theme)))
        .child(
            divided_block(theme)
                .flex()
                .flex_col()
                .gap_2p5()
                .child(render_popular_tags_title(theme))
                .child(render_trending_list(trending)),
        )
        .child(render_network_agent_roster(&state.current_network_agents(), theme))
}
