//! Overview card of the selected profile, as wide as its other tabs: portrait beside the
//! identity, the biography and the actions (Edit in Forge, use persona / run a turn, remove).

use super::params::{ProfileCallback, ProfileTab, RosterCallbacks, RosterParams};
use super::profile_tabs::{beliefs_column, profile_tabs};
use super::recommended::recommended_column;
use crate::domain::{Profile, ProfileId};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::components::{meta_badge, mono_tag, BadgeColors};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants};
use gpui_kit::component::WindowExt;
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
    let text = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(div().text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).child(profile.name.clone()))
        .child(div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(profile.display_handle()))
        .child(div().flex().gap(theme.spacing_xs).child(mono_tag(profile.config.mbti.code(), accent, theme)).children(active))
        .children((!meta.is_empty()).then(|| div().text_size(theme.font_size_sm).text_color(theme.text_secondary).child(meta.join(META_SEPARATOR))));
    div().flex().items_center().gap(theme.section_gap).child(render_profile_avatar(profile, theme.forge_portrait_size * PORTRAIT_SCALE, theme)).child(text)
}

fn actions(profile: &Profile, callbacks: &RosterCallbacks) -> Div {
    let id = profile.id.clone();
    let (edit, delete) = (callbacks.edit.clone(), callbacks.delete.clone());
    let (edit_id, delete_id, action_id) = (id.clone(), id.clone(), id);
    let name = profile.name.clone();
    let primary = if profile.is_agent() {
        let run = callbacks.trigger_agent.clone();
        Button::new(PRIMARY_ACTION_ID).outline().icon(LucideIcon::Zap).label(t!("trigger.open").to_string()).on_click(move |_, w, cx| run(action_id.clone(), w, cx))
    } else {
        let activate = callbacks.activate_persona.clone();
        Button::new(PRIMARY_ACTION_ID).outline().icon(LucideIcon::UserCheck).label(t!("lab.use_persona").to_string()).on_click(move |_, w, cx| activate(action_id.clone(), w, cx))
    };
    div()
        .flex()
        .flex_wrap()
        .child(Button::new(EDIT_ID).primary().icon(LucideIcon::Edit).label(t!("lab.edit_in_forge").to_string()).on_click(move |_, w, cx| edit(edit_id.clone(), w, cx)))
        .child(primary)
        .child(Button::new(DELETE_ID).ghost().icon(LucideIcon::Trash).tooltip(t!("lab.delete_tooltip").to_string()).on_click(move |_, w, cx| confirm_remove(name.clone(), delete_id.clone(), delete.clone(), w, cx)))
}

/// Asks before removing a profile: it leaves the roster for good (its posts stay).
pub(super) fn confirm_remove(name: String, id: ProfileId, delete: ProfileCallback, window: &mut Window, cx: &mut App) {
    window.open_alert_dialog(cx, move |alert, _, _| {
        let (id, delete) = (id.clone(), delete.clone());
        alert
            .confirm()
            .title(t!("lab.delete_title", name = name).to_string())
            .description(t!("lab.delete_desc").to_string())
            .ok_text(t!("lab.delete_ok").to_string())
            .ok_variant(ButtonVariant::Danger)
            .cancel_text(t!("composer.cancel").to_string())
            .on_ok(move |_, window, cx| {
                delete(id.clone(), window, cx);
                true
            })
    });
}

pub(super) fn render_overview(profile: &Profile, params: &RosterParams<'_>, callbacks: &RosterCallbacks) -> impl IntoElement {
    let theme = params.theme;
    let active_persona = !profile.is_agent() && params.state.active_profile_id == profile.id;
    let bio = (!profile.config.bio.trim().is_empty()).then(|| {
        div().text_size(theme.font_size_sm).text_color(theme.text_muted).line_height(relative(theme.line_height_body)).child(profile.config.bio.clone())
    });
    let card = div()
        .w_full()
        .max_w(theme.reader_measure_width)
        .p(theme.panel_padding)
        .flex()
        .flex_col()
        .gap(theme.section_gap)
        .rounded(theme.radius_xl)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(identity(profile, active_persona, theme))
        .children(bio)
        .child(actions(profile, callbacks).gap(theme.control_gap))
        .children(params.run_status.as_ref().filter(|_| profile.is_agent()).map(|status| super::run_status::render_run_status(profile, status, &callbacks.run, &callbacks.open_post, theme))); // A5
    // sb F1, A2: the overview switches between the profile card, its posts, its beliefs (a
    // persona's stated credences), and the posts recommended to it.
    let tabs = profile_tabs(params.profile_tab, profile.is_agent(), theme, callbacks.switch_profile_tab.clone());
    let body = match (&params.beliefs, params.profile_tab, profile.is_agent()) {
        (Some(view), ProfileTab::Beliefs, true) => beliefs_column(view.clone(), theme),
        (_, ProfileTab::Beliefs, false) => super::persona_beliefs::persona_beliefs_column(profile, &params.persona_credences, &callbacks.open_post, theme), // A2
        (_, ProfileTab::Recommended, _) => recommended_column(profile, &params.recommended, &callbacks.open_post, theme),
        (_, ProfileTab::Posts, _) => super::posts_tab::posts_column(profile, params, callbacks),
        _ => card,
    };
    div().id(OVERVIEW_ID).flex_1().h_full().overflow_y_scrollbar().p(theme.panel_padding).flex().flex_col().items_center().gap(theme.section_gap).child(tabs).child(body)
}
