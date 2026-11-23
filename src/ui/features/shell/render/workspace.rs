//! Personas & Agents wiring: the roster and the selected profile's overview.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::state::semantic::SemanticHit;
use crate::ui::features::roster::{render_roster_view, LabMode, ProfileTab, RosterCallbacks, RosterParams};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// Roster plus the selected profile's overview for the active lab mode.
    pub(super) fn agent_workspace(&self, bind: &Binder) -> impl IntoElement {
        let params = RosterParams {
            state: &self.state,
            mode: self.current_lab_mode(),
            selected_profile_id: self.lab.selected_profile_id.as_ref(),
            theme: &self.theme,
            profile_tab: self.arena.profile_tab,
            beliefs: Some(self.arena.beliefs_panel.clone().into()),
            recommended: self.roster_recommendations(),
            posts: self.lab.posts, // W7
        };
        let callbacks = RosterCallbacks {
            select: Rc::new(bind.with(Self::lab_select_profile)),
            switch_mode: Rc::new(bind.with(Self::lab_switch_mode)),
            create: Rc::new(bind.act(Self::open_forge_new)),
            edit: Rc::new(bind.with(Self::open_forge_edit)),
            delete: Rc::new(bind.with(Self::lab_delete_profile)),
            activate_persona: Rc::new(bind.with(Self::lab_activate_persona)),
            trigger_agent: Rc::new(bind.with(Self::lab_trigger_agent)),
            switch_profile_tab: Rc::new(bind.with(Self::switch_profile_tab)),
            open_post: Rc::new(bind.with(Self::open_profile_post)),
            posts: Rc::new(bind.with(Self::profile_posts_event)), // W7
        };
        render_roster_view(params, callbacks)
    }

    /// The selected profile's recommended posts while its Recommended tab is open (the
    /// roster shows the first contact when none is selected).
    fn roster_recommendations(&self) -> Rc<[SemanticHit]> {
        if self.arena.profile_tab != ProfileTab::Recommended {
            return Rc::from([]);
        }
        let contacts = match self.current_lab_mode() {
            LabMode::NetworkAgents => self.state.current_network_agents(),
            LabMode::PersonaLab => self.state.current_network_personas(),
        };
        let selected = self.lab.selected_profile_id.as_ref().filter(|id| contacts.iter().any(|p| &p.id == *id)).or_else(|| contacts.first().map(|p| &p.id));
        self.profile_hits(selected)
    }
}
