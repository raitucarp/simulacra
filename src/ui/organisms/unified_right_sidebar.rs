//! Unified Right Sidebar organism combining Mind & Cadence and Vault & Telemetry
//! into a modular, segmented panel to maximize center feed breathing room.

use std::rc::Rc;

use crate::domain::{GroundingDocument, Post};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::molecules::{render_knowledge_card, render_network_agent_roster, render_topic_item};
use crate::ui::organisms::secondary_sidebar::{
    render_clickable_heatmap, render_obsidian_dot_graph, render_random_spark_feed_with_lookup, ClickDayCallback,
};
use gpui_kit::base::StyledExt;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RightSidebarTab {
    MindCadence,
    VaultTelemetry,
}

#[allow(clippy::type_complexity)]
pub struct UnifiedRightSidebarParams<'a> {
    pub active_tab: RightSidebarTab,
    pub on_select_tab: Box<dyn Fn(RightSidebarTab, &mut Window, &mut App) + 'static>,
    pub state: &'a AppState,
    pub posts: &'a [Post],
    pub documents: &'a [GroundingDocument],
    pub selected_day: Option<usize>,
    pub on_click_day: ClickDayCallback,
    pub search_input: &'a Entity<InputState>,
    pub on_click_topic: Box<dyn Fn(&str, &mut Window, &mut App) + 'static>,
    pub theme: &'a ThemeTokens,
}

pub fn render_unified_right_sidebar(params: UnifiedRightSidebarParams<'_>) -> impl IntoElement {
    let theme = params.theme;
    let active_tab = params.active_tab;
    let on_select_tab = Rc::new(params.on_select_tab);

    let tab_mind = on_select_tab.clone();
    let tab_vault = on_select_tab.clone();

    let on_topic = Rc::new(params.on_click_topic);
    let mut trending_elements = Vec::new();
    for (idx, topic) in params.state.trending.iter().enumerate() {
        let cb = on_topic.clone();
        trending_elements.push(
            render_topic_item(topic, idx + 1, theme, move |tag, w, cx| {
                cb(tag, w, cx);
            })
            .into_any_element(),
        );
    }

    div()
        .id("unified-right-sidebar-scroll-container")
        .w_full()
        .h_full()
        .overflow_hidden()
        .bg(theme.bg_sidebar)
        .border_l_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        // Top Segmented Tab Switcher: Mind & Cadence vs Telemetry & Vault
        .child(
            div()
                .w_full()
                .px(theme.spacing_md)
                .pt(theme.spacing_md)
                .child(
                    div()
                        .w_full()
                        .p_1()
                        .rounded_xl()
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .items_center()
                        .gap_1()
                        // Tab 1: Mind & Cadence
                        .child({
                            let is_active = active_tab == RightSidebarTab::MindCadence;
                            div()
                                .id("tab-switch-mind-cadence")
                                .flex_1()
                                .py_1p5()
                                .rounded_lg()
                                .cursor_pointer()
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap_1p5()
                                .when(is_active, |s| {
                                    s.bg(theme.hover_bg)
                                        .text_color(theme.text_primary)
                                        .border_1()
                                        .border_color(theme.border)
                                })
                                .when(!is_active, |s| s.text_color(theme.text_muted))
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, window, cx| {
                                    tab_mind(RightSidebarTab::MindCadence, window, cx);
                                })
                                .child(
                                    div()
                                        .w_3p5()
                                        .h_3p5()
                                        .child(Icon::new(LucideIcon::Share2)),
                                )
                                .child(
                                    div()
                                        .font_family(theme.font_family_ui.clone())
                                        .text_xs()
                                        .font_medium()
                                        .child(t!("sidebar.mind_cadence").to_string()),
                                )
                        })
                        // Tab 2: Vault & Telemetry
                        .child({
                            let is_active = active_tab == RightSidebarTab::VaultTelemetry;
                            div()
                                .id("tab-switch-vault-telemetry")
                                .flex_1()
                                .py_1p5()
                                .rounded_lg()
                                .cursor_pointer()
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap_1p5()
                                .when(is_active, |s| {
                                    s.bg(theme.hover_bg)
                                        .text_color(theme.text_primary)
                                        .border_1()
                                        .border_color(theme.border)
                                })
                                .when(!is_active, |s| s.text_color(theme.text_muted))
                                .hover(move |s| s.text_color(theme.text_primary))
                                .on_click(move |_, window, cx| {
                                    tab_vault(RightSidebarTab::VaultTelemetry, window, cx);
                                })
                                .child(
                                    div()
                                        .w_3p5()
                                        .h_3p5()
                                        .child(Icon::new(LucideIcon::Activity)),
                                )
                                .child(
                                    div()
                                        .font_family(theme.font_family_ui.clone())
                                        .text_xs()
                                        .font_medium()
                                        .child(t!("sidebar.vault_telemetry").to_string()),
                                )
                        }),
                ),
        )
        // Content Area depending on active tab
        .when(active_tab == RightSidebarTab::MindCadence, |this| {
            this.child(
                div()
                    .w_full()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_md)
                    .child(
                        div()
                            .w_full()
                            .px(theme.spacing_md)
                            .child(render_obsidian_dot_graph(params.posts, params.documents, theme)),
                    )
                    .child(
                        div()
                            .w_full()
                            .px(theme.spacing_md)
                            .child(render_clickable_heatmap(params.selected_day, params.on_click_day, theme)),
                    )
                    .child(render_random_spark_feed_with_lookup(params.posts, params.state, theme)),
            )
        })
        .when(active_tab == RightSidebarTab::VaultTelemetry, |this| {
            this.child(
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
                    .child(
                        div()
                            .w_full()
                            .rounded_lg()
                            .border_1()
                            .border_color(theme.border)
                            .bg(theme.bg_card)
                            .p_1()
                            .child(Input::new(params.search_input).cleanable(true)),
                    )
                    .child(
                        div()
                            .w_full()
                            .pb_3p5()
                            .border_b_1()
                            .border_color(theme.border)
                            .child(render_knowledge_card(theme)),
                    )
                    .child(
                        div()
                            .w_full()
                            .pb_3p5()
                            .border_b_1()
                            .border_color(theme.border)
                            .flex()
                            .flex_col()
                            .gap_2p5()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child(t!("sidebar.popular_tags").to_string()),
                            )
                            .child(div().flex().flex_col().gap_1().children(trending_elements)),
                    )
                    .child(render_network_agent_roster(&params.state.current_network_agents(), theme)),
            )
        })
}
