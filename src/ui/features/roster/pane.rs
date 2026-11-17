//! Roster pane: segmented mode switch, scrollable contacts, pinned "New agent / persona".

use super::contact_row::{render_contact_row, ContactRowState};
use super::mode_switch::render_mode_switch;
use super::params::{LabMode, RosterCallbacks, RosterData, RosterParams};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const PANE_ID: &str = "roster-contact-pane";
const LIST_ID: &str = "roster-contact-list";
const CREATE_BUTTON_ID: &str = "roster-create-profile";

pub(super) fn render_roster_pane(params: &RosterParams<'_>, data: &RosterData<'_>, callbacks: &RosterCallbacks) -> Stateful<Div> {
    let theme = params.theme;
    div()
        .id(PANE_ID)
        .size_full()
        .bg(theme.bg_sidebar)
        .border_r(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .child(render_mode_switch(params, data, callbacks.switch_mode.clone()))
        .child(contact_list(params, data, callbacks))
        .child(create_footer(params.mode, theme, callbacks.create.clone()))
}

fn contact_list(params: &RosterParams<'_>, data: &RosterData<'_>, callbacks: &RosterCallbacks) -> impl IntoElement {
    let theme = params.theme;
    let selected_id = data.selected.map(|p| &p.id);
    div()
        .id(LIST_ID)
        .flex_1()
        .overflow_y_scrollbar()
        .p(theme.panel_padding_compact)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .children(data.contacts.iter().enumerate().map(|(idx, prof)| {
            let state = ContactRowState {
                selected: selected_id == Some(&prof.id),
                active_user_persona: params.state.active_profile_id == prof.id,
            };
            render_contact_row(idx, prof, state, theme, callbacks.select.clone())
        }))
}

/// The "New agent / persona" label of `mode` (the button adds its own plus icon).
pub(super) fn create_label(mode: LabMode) -> String {
    match mode {
        LabMode::NetworkAgents => t!("lab.new_agent").to_string(),
        LabMode::PersonaLab => t!("lab.new_persona").to_string(),
    }
}

fn create_footer(mode: LabMode, theme: &ThemeTokens, on_create: Rc<dyn Fn(&mut Window, &mut App)>) -> Div {
    div()
        .w_full()
        .p(theme.panel_padding_compact)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(
            Button::new(CREATE_BUTTON_ID)
                .primary()
                .w_full()
                .icon(Icon::new(LucideIcon::Plus))
                .label(create_label(mode))
                .on_click(move |_, w, cx| on_create(w, cx)),
        )
}
