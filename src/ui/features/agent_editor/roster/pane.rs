//! Roster pane container: segmented mode switch, scrollable contacts, pinned create button.

use super::contact_row::{render_contact_row, ContactRowState};
use super::mode_switch::render_mode_switch;
use crate::theme::ThemeTokens;
use crate::ui::features::agent_editor::callbacks::{CreateProfileCallback, SharedCallbacks};
use crate::ui::features::agent_editor::config_tab::LabMode;
use crate::ui::features::agent_editor::params::WorkspaceScope;
use crate::ui::features::agent_editor::workspace::WorkspaceData;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const PANE_ID: &str = "agent-workspace-left-contact-pane";
const LIST_ID: &str = "agent-contact-list-scroll";
const CREATE_BUTTON_ID: &str = "create-new-lab-profile-btn";

pub(crate) fn render_roster_pane(
    scope: &WorkspaceScope<'_>,
    data: &WorkspaceData<'_>,
    callbacks: &SharedCallbacks,
) -> Stateful<Div> {
    let theme = scope.theme;
    div()
        .id(PANE_ID)
        .w(theme.contact_list_pane_width)
        .flex_shrink_0()
        .h_full()
        .bg(theme.bg_sidebar)
        .border_r(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .child(render_mode_switch(scope, data, callbacks.switch_mode.clone()))
        .child(contact_list(scope, data, callbacks))
        .child(create_footer(scope.mode, theme, callbacks.create.clone()))
}

fn contact_list(
    scope: &WorkspaceScope<'_>,
    data: &WorkspaceData<'_>,
    callbacks: &SharedCallbacks,
) -> impl IntoElement {
    let theme = scope.theme;
    let selected_id = data.selected.map(|p| &p.id);
    div()
        .id(LIST_ID)
        .flex_1()
        .overflow_y_scrollbar()
        .p(theme.spacing_xs)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .children(data.contact_list.iter().enumerate().map(|(idx, prof)| {
            let state = ContactRowState {
                selected: selected_id == Some(&prof.id),
                active_user_persona: scope.state.active_profile_id == prof.id,
            };
            render_contact_row(idx, prof, state, theme, callbacks.select_profile.clone())
        }))
}

fn create_footer(mode: LabMode, theme: &ThemeTokens, on_create: Rc<CreateProfileCallback>) -> Div {
    let label = match mode {
        LabMode::NetworkAgents => t!("lab.new_agent").to_string(),
        LabMode::PersonaLab => t!("lab.new_persona").to_string(),
    };
    div()
        .w_full()
        .p(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(
            Button::new(CREATE_BUTTON_ID)
                .primary()
                .w_full()
                .icon(Icon::new(LucideIcon::Plus))
                .label(label)
                .on_click(move |_, w, cx| on_create(w, cx)),
        )
}
