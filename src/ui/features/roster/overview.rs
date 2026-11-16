//! Overview card of the selected profile: portrait, identity, a glimpse of the biography and
//! the actions (Edit in Forge, use persona / run a turn, delete).

use super::params::{RosterCallbacks, RosterParams};
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::components::{meta_badge, mono_tag, BadgeColors};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;
use rust_i18n::t;

const OVERVIEW_ID: &str = "roster-overview";
const EDIT_ID: &str = "roster-edit-in-forge";
const PRIMARY_ACTION_ID: &str = "roster-primary-action";
const DELETE_ID: &str = "roster-delete";
const META_SEPARATOR: &str = " · ";
/// The overview portrait is this fraction of the forge stage portrait.
const PORTRAIT_SCALE: f32 = 0.6;

fn identity(profile: &Profile, active_persona: bool, theme: &ThemeTokens) -> Div {
    let accent = if profile.is_agent() { theme.accent_agent } else { theme.accent_user };
    let meta: Vec<String> = [&profile.config.occupation, &profile.config.location_culture]
        .into_iter()
        .filter(|text| !text.trim().is_empty())
        .cloned()
        .collect();
    let active = active_persona.then(|| {
        let colors = BadgeColors::new(theme.accent_success, theme.accent_success_subtle, theme.accent_success_border);
        meta_badge(&t!("lab.active_persona_badge"), colors, theme)
    });
    div()
        .flex()
        .flex_col()
        .items_center()
        .gap(theme.spacing_xs)
        .child(render_profile_avatar(profile, theme.forge_portrait_size * PORTRAIT_SCALE, theme))
        .child(div().text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).child(profile.name.clone()))
        .child(div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(profile.display_handle()))
        .child(div().flex().gap(theme.spacing_xs).child(mono_tag(profile.config.mbti.code(), accent, theme)).children(active))
        .children((!meta.is_empty()).then(|| div().text_size(theme.font_size_sm).text_color(theme.text_secondary).child(meta.join(META_SEPARATOR))))
}

fn actions(profile: &Profile, callbacks: &RosterCallbacks) -> Div {
    let id = profile.id.clone();
    let (edit, delete) = (callbacks.edit.clone(), callbacks.delete.clone());
    let (edit_id, delete_id, action_id) = (id.clone(), id.clone(), id);
    let primary = if profile.is_agent() {
        let run = callbacks.trigger_agent.clone();
        Button::new(PRIMARY_ACTION_ID).outline().icon(LucideIcon::Zap).label(t!("lab.synthesize_post").to_string()).on_click(move |_, w, cx| run(action_id.clone(), w, cx))
    } else {
        let activate = callbacks.activate_persona.clone();
        Button::new(PRIMARY_ACTION_ID).outline().icon(LucideIcon::UserCheck).label(t!("lab.use_persona").to_string()).on_click(move |_, w, cx| activate(action_id.clone(), w, cx))
    };
    div()
        .flex()
        .flex_wrap()
        .justify_center()
        .child(Button::new(EDIT_ID).primary().icon(LucideIcon::Edit).label(t!("lab.edit_in_forge").to_string()).on_click(move |_, w, cx| edit(edit_id.clone(), w, cx)))
        .child(primary)
        .child(Button::new(DELETE_ID).ghost().icon(LucideIcon::Trash).tooltip(t!("lab.delete_tooltip").to_string()).on_click(move |_, w, cx| delete(delete_id.clone(), w, cx)))
}

pub(super) fn render_overview(profile: &Profile, params: &RosterParams<'_>, callbacks: &RosterCallbacks) -> impl IntoElement {
    let theme = params.theme;
    let active_persona = !profile.is_agent() && params.state.active_profile_id == profile.id;
    let bio = (!profile.config.bio.trim().is_empty()).then(|| {
        div().text_size(theme.font_size_sm).text_color(theme.text_muted).line_height(relative(theme.line_height_body)).child(profile.config.bio.clone())
    });
    let card = div()
        .w(theme.forge_trading_card_width)
        .p(theme.spacing_xl)
        .flex()
        .flex_col()
        .items_center()
        .gap(theme.spacing_lg)
        .rounded(theme.radius_xl)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(identity(profile, active_persona, theme))
        .children(bio)
        .child(actions(profile, callbacks).gap(theme.spacing_sm));
    div().id(OVERVIEW_ID).flex_1().h_full().overflow_y_scrollbar().p(theme.spacing_2xl).flex().items_start().justify_center().child(card)
}
