//! "Personas & Agents": the roster of the active network (segmented Network Agents / My
//! Personas switch, contact list, "New agent / persona" button) beside an overview card of the
//! selected profile with "Edit in Forge". Creating and editing happen in the Agent Forge.

mod contact_row;
mod empty;
mod mode_switch;
mod overview;
mod pane;
mod params;
mod post_cards; // W7
mod posts_state; // W7
mod posts_tab; // W7
mod profile_tabs;
mod recommended;
#[cfg(test)]
mod tests;

pub use params::{LabMode, ProfileTab, RosterCallbacks, RosterParams};
pub use posts_state::{ProfilePosts, ProfilePostsEvent};
pub use recommended::OpenPostCallback;

use crate::theme::ThemeTokens;
use crate::ui::components::{persisted_h_resizable, SplitPane};
use empty::render_empty_roster;
use gpui_kit::*;
use overview::render_overview;
use pane::render_roster_pane;
use params::RosterData;

const ROOT_ID: &str = "roster-workspace-root";
/// Layout id of the contact list | overview split (sizes persist).
const SPLIT_ID: &str = "roster-split";

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
        .child(
            persisted_h_resizable(SPLIT_ID, theme)
                .pane(
                    SplitPane::sized(theme.contact_list_pane_width, theme.contact_list_pane_min..theme.contact_list_pane_max)
                        .child(render_roster_pane(&params, &data, &callbacks)),
                )
                .pane(SplitPane::flex(theme.split_flex_min).child(body)),
        )
}
