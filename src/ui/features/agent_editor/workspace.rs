//! Three-tier workspace root: contact roster (left) + configuration pane (right).

use super::config_pane::render_config_pane;
use super::config_tab::LabMode;
use super::params::{AgentWorkspaceParams, WorkspaceScope};
use super::roster::render_roster_pane;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;

const ROOT_ID: &str = "agent-workspace-two-pane-root";
const EMPTY_STATE_TEXT: &str = "Select or create a profile in the left roster.";

/// Values derived once from [`crate::state::AppState`] per render.
pub(crate) struct WorkspaceData<'a> {
    pub network_name: String,
    pub grounding_doc_titles: Vec<String>,
    pub agents_count: usize,
    pub personas_count: usize,
    /// Profiles listed in the roster for the current [`LabMode`].
    pub contact_list: Vec<&'a Profile>,
    /// Explicitly selected profile, falling back to the first roster entry.
    pub selected: Option<&'a Profile>,
}

impl<'a> WorkspaceData<'a> {
    fn collect(scope: &WorkspaceScope<'a>) -> Self {
        let state = scope.state;
        let network_name = state
            .current_network()
            .map(|n| n.name.clone())
            .unwrap_or_else(|| t!("workspace.active").to_string());
        let contact_list: Vec<&'a Profile> = match scope.mode {
            LabMode::NetworkAgents => state.current_network_agents(),
            LabMode::PersonaLab => state.current_network_personas(),
        };
        let selected = scope
            .selected_profile_id
            .and_then(|id| contact_list.iter().find(|p| &p.id == id).copied())
            .or_else(|| contact_list.first().copied());
        Self {
            network_name,
            grounding_doc_titles: state.current_documents().map(|d| d.title.clone()).collect(),
            agents_count: state.current_network_agents().len(),
            personas_count: state.current_network_personas().len(),
            contact_list,
            selected,
        }
    }
}

/// Renders the complete Three-Tier Workspace:
/// - Left Pane: Contact Roster (`Network Agents` | `My Personas`)
/// - Right Pane: Compact Hero Header + (Vertical Category Rail | Horizontal Sub-Tabs Content Area)
pub fn render_agent_workspace_view(params: AgentWorkspaceParams<'_>) -> impl IntoElement {
    let (scope, callbacks) = params.split();
    let data = WorkspaceData::collect(&scope);
    let theme = scope.theme;

    div()
        .id(ROOT_ID)
        .w_full()
        .h_full()
        .bg(theme.bg_app)
        .flex()
        .flex_row()
        .overflow_hidden()
        .child(render_roster_pane(&scope, &data, &callbacks))
        .child(match data.selected {
            Some(profile) => {
                render_config_pane(profile, &scope, &data, &callbacks).into_any_element()
            }
            None => render_empty_state(theme).into_any_element(),
        })
}

fn render_empty_state(theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .text_size(theme.font_size_sm)
        .text_color(theme.text_muted)
        .child(EMPTY_STATE_TEXT)
}
