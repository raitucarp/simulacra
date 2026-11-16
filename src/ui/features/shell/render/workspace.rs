//! Personas & Agents wiring: the roster and the selected profile's overview.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::roster::{render_roster_view, RosterCallbacks, RosterParams};
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
        };
        let callbacks = RosterCallbacks {
            select: Rc::new(bind.with(Self::lab_select_profile)),
            switch_mode: Rc::new(bind.with(Self::lab_switch_mode)),
            create: Rc::new(bind.act(Self::open_forge_new)),
            edit: Rc::new(bind.with(Self::open_forge_edit)),
            delete: Rc::new(bind.with(Self::lab_delete_profile)),
            activate_persona: Rc::new(bind.with(Self::lab_activate_persona)),
            trigger_agent: Rc::new(bind.with(Self::lab_trigger_agent)),
        };
        render_roster_view(params, callbacks)
    }
}
