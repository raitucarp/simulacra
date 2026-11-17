//! The primary places: Timeline, Knowledge Vault, Personas & Agents and Mind Graph, each with
//! its count. Preferences (Appearance) live in the footer instead.

use super::nav_button::{render_nav_button, NavButtonSpec};
use super::SidebarCallbacks;
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::CommandId;
use crate::ui::icons::LucideIcon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// One place: its tab, label key, icon and the command that opens it (for the shortcut).
struct Place {
    tab: NavigationTab,
    label_key: &'static str,
    icon: LucideIcon,
    command: CommandId,
}

const PLACES: [Place; 4] = [
    Place { tab: NavigationTab::Timeline, label_key: "nav.home", icon: LucideIcon::Activity, command: CommandId::GoTimeline },
    Place { tab: NavigationTab::Knowledge, label_key: "nav.knowledge", icon: LucideIcon::BookOpen, command: CommandId::GoKnowledge },
    Place { tab: NavigationTab::AgentLab, label_key: "nav.agent_lab", icon: LucideIcon::Users, command: CommandId::GoAgents },
    Place { tab: NavigationTab::MindGraph, label_key: "nav.mind_graph", icon: LucideIcon::Share2, command: CommandId::GoMindGraph },
];

/// Whether `place` is the active screen (the Persona Lab counts as Personas & Agents).
fn is_active(place: &Place, active: NavigationTab) -> bool {
    place.tab == active || (place.tab == NavigationTab::AgentLab && active == NavigationTab::PersonaLab)
}

fn count_of(tab: NavigationTab, state: &AppState) -> Option<usize> {
    match tab {
        NavigationTab::Timeline => Some(state.current_network_post_count()),
        NavigationTab::Knowledge => Some(state.current_documents().count()),
        NavigationTab::AgentLab => Some(state.current_network_agents().len() + state.current_network_personas().len()),
        _ => None,
    }
}

pub(super) fn render_nav_list(state: &AppState, is_rail: bool, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Div {
    let buttons = PLACES.iter().map(|place| {
        let spec = NavButtonSpec {
            label: t!(place.label_key).to_string(),
            icon: place.icon,
            is_active: is_active(place, state.active_tab),
            count: count_of(place.tab, state),
            command: Some(place.command),
        };
        let (nav, tab) = (callbacks.on_select_nav.clone(), place.tab);
        render_nav_button(spec, is_rail, theme, Rc::new(move |window, cx| nav(tab, window, cx)))
    });
    div().flex().flex_col().gap(theme.spacing_2xs).when(is_rail, |list| list.items_center()).children(buttons)
}
