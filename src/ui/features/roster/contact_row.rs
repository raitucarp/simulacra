//! One roster entry: avatar, name, handle/description and MBTI + status markers.

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::components::{mono_tag, selectable_row, status_dot, status_glyph};
use super::params::RosterCallbacks;
use crate::ui::menus::render_menu;
use gpui_kit::component::menu::ContextMenuExt;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const ROW_ID_PREFIX: &str = "contact-row";

/// Selection flags of a contact row.
#[derive(Debug, Clone, Copy)]
pub(super) struct ContactRowState {
    pub selected: bool,
    /// The row is the human persona currently used as the active user identity.
    pub active_user_persona: bool,
}

pub(super) fn render_contact_row(
    idx: usize,
    prof: &Profile,
    state: ContactRowState,
    theme: &ThemeTokens,
    callbacks: &RosterCallbacks,
) -> impl IntoElement {
    let (prof_id, on_select) = (prof.id.clone(), callbacks.select.clone());
    let row_id = format!("{}-{}-{}", ROW_ID_PREFIX, idx, prof.id);
    let menu = super::profile_menu::profile_menu_spec(prof, callbacks);
    selectable_row(row_id, state.selected, theme.accent_brand, theme)
        .on_click(move |_, w, cx| on_select(prof_id.clone(), w, cx))
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .child(identity(prof, state.selected, theme))
        .child(markers(prof, state.active_user_persona, theme))
        .context_menu(move |popup, window, cx| render_menu(popup, &menu, window, cx)) // M1
}

fn identity(prof: &Profile, selected: bool, theme: &ThemeTokens) -> Div {
    let subtitle = if !prof.config.description.is_empty() {
        format!("{} · {}", prof.display_handle(), prof.config.description)
    } else {
        prof.display_handle()
    };
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .min_w_0()
        .flex_1()
        .child(render_profile_avatar(prof, theme.contact_avatar_size, theme))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(theme.spacing_3xs)
                .child(
                    div()
                        .text_size(theme.font_size_sm)
                        .font_weight(if selected { FontWeight::BOLD } else { FontWeight::SEMIBOLD })
                        .text_color(theme.text_primary)
                        .truncate()
                        .child(prof.name.clone()),
                )
                .child(
                    div()
                        .text_size(theme.font_size_2xs)
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(subtitle),
                ),
        )
}

/// MBTI code tag plus an "active" marker (dot for running agents, glyph for the in-use persona).
fn markers(prof: &Profile, active_user_persona: bool, theme: &ThemeTokens) -> Div {
    let is_agent = prof.is_agent();
    let code_color = if is_agent { theme.accent_agent } else { theme.accent_user };
    div()
        .flex_shrink_0()
        .flex()
        .flex_col()
        .items_end()
        .gap(theme.spacing_2xs)
        .child(mono_tag(prof.config.mbti.code(), code_color, theme).font_weight(FontWeight::SEMIBOLD))
        .when(is_agent && prof.config.is_active, |s| {
            s.child(status_dot(theme.accent_success, theme))
        })
        .when(!is_agent && active_user_persona, |s| {
            s.child(status_glyph(theme.accent_success, theme))
        })
}
