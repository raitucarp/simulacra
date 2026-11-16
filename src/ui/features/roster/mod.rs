//! "Personas & Agents": the roster of the active network (segmented Network Agents / My
//! Personas switch, contact list, "New agent / persona" button) beside an overview card of the
//! selected profile with "Edit in Forge". Creating and editing happen in the Agent Forge.

mod contact_row;
mod empty;
mod mode_switch;
mod overview;
mod pane;
mod params;

pub use params::{LabMode, RosterCallbacks, RosterParams};

use crate::theme::ThemeTokens;
use empty::render_empty_roster;
use gpui_kit::*;
use overview::render_overview;
use pane::render_roster_pane;
use params::RosterData;

const ROOT_ID: &str = "roster-workspace-root";

/// The roster pane and the overview of the selected profile.
pub fn render_roster_view(params: RosterParams<'_>, callbacks: RosterCallbacks) -> impl IntoElement {
    let data = RosterData::collect(&params);
    let theme: &ThemeTokens = params.theme;
    let body = match data.selected {
        Some(profile) => render_overview(profile, &params, &callbacks).into_any_element(),
        None => render_empty_roster(params.mode, theme, callbacks.create.clone()).into_any_element(),
    };
    div()
        .id(ROOT_ID)
        .size_full()
        .bg(theme.bg_app)
        .flex()
        .overflow_hidden()
        .child(render_roster_pane(&params, &data, &callbacks))
        .child(body)
}
