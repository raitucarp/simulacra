//! Main feed entry point: sub-view router.

use super::inputs::{split_params, ToolViewInputs};
use super::params::FeedParams;
use super::timeline::render_timeline;
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::features::vault::render_knowledge_vault_view;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, IntoElement, ParentElement, Styled};
use rust_i18n::t;

pub fn render_main_feed<C: IntoElement>(params: FeedParams<'_, C>) -> impl IntoElement {
    let state = params.state;
    let theme = params.theme;

    let _network_name = state
        .current_network()
        .map(|n| n.name.clone())
        .unwrap_or_else(|| t!("feed.general_timeline").to_string());

    let (callbacks, timeline_inputs, tools) = split_params(params);
    let timeline = render_timeline(state, theme, &callbacks, timeline_inputs);

    let feed = div()
        .w_full()
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_app)
        // Sub-view Router
        .when(state.active_tab == NavigationTab::Timeline, |this| this.child(timeline));
    route_tool_views(feed, state, theme, tools)
}

/// Appends the knowledge vault view for the active tab (the Mind Graph is routed by the shell).
fn route_tool_views(feed: Div, state: &AppState, theme: &ThemeTokens, tools: ToolViewInputs<'_>) -> Div {
    feed.when(state.active_tab == NavigationTab::Knowledge, |this| {
        this.child(render_knowledge_vault_view(
            &state.documents,
            theme,
            tools.doc_title_input,
            tools.doc_content_input,
            tools.on_add_document,
            tools.on_delete_document,
        ))
    })
}
