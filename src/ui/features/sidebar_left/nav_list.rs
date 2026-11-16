//! Primary navigation button stack (Home, Knowledge, Agents, Mind Graph, Appearance).

use super::nav_button::render_nav_button;
use super::NavHandle;
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::{div, Div, ParentElement, Styled};
use rust_i18n::t;

/// One row of the navigation table.
struct NavEntry {
    label: String,
    icon: LucideIcon,
    target: NavigationTab,
    is_active: bool,
    badge: Option<usize>,
}

fn nav_entries(state: &AppState, is_lab_active: bool, doc_count: usize) -> [NavEntry; 5] {
    let active = |tab: NavigationTab| state.active_tab == tab;
    let entry = |label: String, icon, target, is_active, badge| NavEntry {
        label,
        icon,
        target,
        is_active,
        badge,
    };
    let posts = Some(state.current_network_post_count());
    let agents = Some(state.current_network_agents().len());
    [
        entry(t!("nav.home").to_string(), LucideIcon::Activity, NavigationTab::Timeline, active(NavigationTab::Timeline), posts),
        entry(t!("nav.knowledge").to_string(), LucideIcon::BookOpen, NavigationTab::Knowledge, active(NavigationTab::Knowledge), Some(doc_count)),
        entry(t!("nav.agent_lab").to_string(), LucideIcon::Users, NavigationTab::AgentLab, is_lab_active, agents),
        entry(t!("nav.mind_graph").to_string(), LucideIcon::Share2, NavigationTab::MindGraph, active(NavigationTab::MindGraph), None),
        entry(t!("nav.appearance").to_string(), LucideIcon::Palette, NavigationTab::Appearance, active(NavigationTab::Appearance), None),
    ]
}

/// Renders the navigation stack; each button routes to its tab through `on_select_nav`.
pub(super) fn render_nav_list(
    state: &AppState,
    is_lab_active: bool,
    is_collapsed: bool,
    doc_count: usize,
    on_select_nav: &NavHandle,
    theme: &ThemeTokens,
) -> Div {
    let buttons = nav_entries(state, is_lab_active, doc_count).into_iter().map(|entry| {
        let nav = on_select_nav.clone();
        let target = entry.target;
        render_nav_button(
            &entry.label,
            entry.icon,
            entry.is_active,
            is_collapsed,
            entry.badge,
            theme,
            move |_, window, cx| {
                nav(target, window, cx);
            },
        )
    });
    div().flex().flex_col().gap(theme.spacing_xs).children(buttons)
}
