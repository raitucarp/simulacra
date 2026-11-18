//! Render parameters, callbacks and per-render data of the roster.

use crate::domain::{Profile, ProfileId};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const ACTIVE_NETWORK_KEY: &str = "workspace.active";

/// Which roster shows: the network's autonomous agents or the user's personas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabMode {
    #[default]
    NetworkAgents,
    PersonaLab,
}

/// sb F1: what the selected agent's overview shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfileTab {
    #[default]
    Profile,
    /// Its beliefs and their revision log.
    Beliefs,
}

/// A shared callback taking a profile id.
pub type ProfileCallback = Rc<dyn Fn(ProfileId, &mut Window, &mut App)>;

/// What the roster can ask the shell to do.
#[derive(Clone)]
pub struct RosterCallbacks {
    pub select: ProfileCallback,
    pub switch_mode: Rc<dyn Fn(LabMode, &mut Window, &mut App)>,
    /// "New agent / persona": opens the Forge for the current mode.
    pub create: Rc<dyn Fn(&mut Window, &mut App)>,
    /// "Edit in Forge".
    pub edit: ProfileCallback,
    pub delete: ProfileCallback,
    pub activate_persona: ProfileCallback,
    pub trigger_agent: ProfileCallback,
    /// sb F1: shows the profile or its beliefs.
    pub switch_profile_tab: Rc<dyn Fn(ProfileTab, &mut Window, &mut App)>,
}

/// Plain data the roster renders from.
pub struct RosterParams<'a> {
    pub state: &'a AppState,
    pub mode: LabMode,
    pub selected_profile_id: Option<&'a ProfileId>,
    pub theme: &'a ThemeTokens,
    /// sb F1: the overview's tab, and the selected agent's Beliefs tab.
    pub profile_tab: ProfileTab,
    pub beliefs: Option<AnyView>,
}

/// Values derived once per render.
pub(super) struct RosterData<'a> {
    pub network_name: String,
    pub agents_count: usize,
    pub personas_count: usize,
    /// Profiles listed for the current mode.
    pub contacts: Vec<&'a Profile>,
    /// The selected profile, falling back to the first contact.
    pub selected: Option<&'a Profile>,
}

impl<'a> RosterData<'a> {
    pub(super) fn collect(params: &RosterParams<'a>) -> Self {
        let state = params.state;
        let network_name = state.current_network().map(|n| n.name.clone()).unwrap_or_else(|| t!(ACTIVE_NETWORK_KEY).to_string());
        let contacts: Vec<&'a Profile> = match params.mode {
            LabMode::NetworkAgents => state.current_network_agents(),
            LabMode::PersonaLab => state.current_network_personas(),
        };
        let selected = params
            .selected_profile_id
            .and_then(|id| contacts.iter().find(|p| &p.id == id).copied())
            .or_else(|| contacts.first().copied());
        Self {
            network_name,
            agents_count: state.current_network_agents().len(),
            personas_count: state.current_network_personas().len(),
            contacts,
            selected,
        }
    }
}
