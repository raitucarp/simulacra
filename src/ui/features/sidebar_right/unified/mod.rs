//! Unified Right Sidebar combining Mind & Cadence and Vault & Telemetry
//! into a modular, segmented panel to maximize center feed breathing room.

mod panes;
mod tabs;

use std::rc::Rc;

use super::heatmap::ClickDayCallback;
use super::trending::trending_elements;
use crate::domain::{GroundingDocument, Post};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::InputState;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use panes::{render_mind_cadence_pane, render_vault_telemetry_pane, MindPaneParams};

const UNIFIED_SIDEBAR_ID: &str = "unified-right-sidebar-scroll-container";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RightSidebarTab {
    MindCadence,
    VaultTelemetry,
}

/// Callback fired when a segmented tab is selected.
pub(crate) type SelectTabCallback = Box<dyn Fn(RightSidebarTab, &mut Window, &mut App) + 'static>;

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
    let trending = trending_elements(params.state, Rc::new(params.on_click_topic), theme);
    let mind = MindPaneParams {
        state: params.state,
        posts: params.posts,
        documents: params.documents,
        selected_day: params.selected_day,
        on_click_day: params.on_click_day,
    };

    div()
        .id(UNIFIED_SIDEBAR_ID)
        .w_full()
        .h_full()
        .overflow_hidden()
        .bg(theme.bg_sidebar)
        .border_l_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(tabs::render_tab_switcher(active_tab, Rc::new(params.on_select_tab), theme))
        .when(active_tab == RightSidebarTab::MindCadence, |this| {
            this.child(render_mind_cadence_pane(mind, theme))
        })
        .when(active_tab == RightSidebarTab::VaultTelemetry, |this| {
            this.child(render_vault_telemetry_pane(params.state, params.search_input, trending, theme))
        })
}
