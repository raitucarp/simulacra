//! Left sidebar root composition: top navigation stack and bottom identity anchor.

use super::grounding::render_grounding_section;
use super::members::render_members_section;
use super::nav_list::render_nav_list;
use super::new_thread::render_new_thread_button;
use super::user_anchor::render_user_anchor;
use super::{LeftSidebarParams, NavHandle, NewThreadCallback, SelectSidebarAgentCallback};
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::features::popovers::WorkspaceTrigger;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, IntoElement, ParentElement, Styled};
use std::rc::Rc;

pub fn render_left_sidebar(params: LeftSidebarParams<'_>) -> impl IntoElement {
    let state = params.state;
    let theme = params.theme;
    let is_collapsed = params.is_collapsed;
    let on_select_nav: NavHandle = Rc::new(params.on_select_nav);
    let on_switch_persona = params.on_switch_persona.map(Rc::new);
    let workspace_trigger = WorkspaceTrigger::new(
        state.current_network().cloned(),
        params.is_workspace_open,
        is_collapsed,
        theme.clone(),
        params.on_toggle_workspace,
    );
    let top = TopSection {
        state,
        theme,
        is_collapsed,
        on_select_nav: on_select_nav.clone(),
        on_select_agent: params.on_select_agent.map(Rc::new),
        on_new_thread: params.on_new_thread,
    };

    div()
        .w_full()
        .h_full()
        .bg(theme.bg_sidebar)
        .flex()
        .flex_col()
        .justify_between()
        .p(theme.panel_padding_compact)
        // Top section
        .child(top.render(workspace_trigger))
        // Bottom User Profile Anchor (Click to open Persona mode in unified Lab, click Switch to cycle active persona)
        .child(render_user_anchor(state, is_collapsed, &on_select_nav, on_switch_persona, theme))
}

/// Inputs of the scrollable top stack (switcher, nav, new thread, sources, members).
struct TopSection<'a> {
    state: &'a AppState,
    theme: &'a ThemeTokens,
    is_collapsed: bool,
    on_select_nav: NavHandle,
    on_select_agent: Option<Rc<SelectSidebarAgentCallback>>,
    on_new_thread: NewThreadCallback,
}

impl TopSection<'_> {
    fn render(self, workspace_trigger: WorkspaceTrigger) -> Div {
        let (state, theme, is_collapsed) = (self.state, self.theme, self.is_collapsed);
        let docs_in_network: Vec<_> = state.current_documents().collect();
        let doc_count = docs_in_network.len();
        let is_lab_active = matches!(
            state.active_tab,
            NavigationTab::AgentLab | NavigationTab::PersonaLab
        );
        let nav = &self.on_select_nav;
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            // 1. Workspace Switcher Header
            .child(workspace_trigger)
            // 2. Primary Navigation Views with Lucide Icons (Network Agents & Persona Lab unified!)
            .child(render_nav_list(state, is_lab_active, is_collapsed, doc_count, nav, theme))
            // 2.5 Prominent "+ New Thread" Action Button
            .child(render_new_thread_button(self.on_new_thread, is_collapsed, theme))
            // 3. Grounding Sources Channels (Only in Expanded mode & when docs exist or not in Lab view)
            .when(!is_collapsed && (!is_lab_active || doc_count > 0), |this| {
                this.child(render_grounding_section(docs_in_network, nav, theme))
            })
            // 4. Network Member Agents Quick Roster
            // Progressive Disclosure: Hidden when `is_lab_active` because the adjacent Contact List Pane
            // already displays the full interactive member roster!
            .when(!is_collapsed && !is_lab_active, |this| {
                let agents = state.current_network_agents();
                this.child(render_members_section(agents, nav, &self.on_select_agent, theme))
            })
    }
}
