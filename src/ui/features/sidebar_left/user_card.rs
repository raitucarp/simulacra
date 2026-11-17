//! The compact identity card: avatar, name and handle (opens the Persona Lab) and an icon
//! button that cycles the active persona.

use crate::domain::{Profile, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_profile_avatar};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// Avatar initial shown when no profile is active.
const UNKNOWN_INITIAL: &str = "?";
const CARD_ID: &str = "sidebar-user-card-container";
const OPEN_PERSONA_LAB_ID: &str = "sidebar-user-card-open-persona-lab";
const SWITCH_BUTTON_ID: &str = "sidebar-user-card-switch-btn";

/// The card; clicking the identity opens the Persona Lab, the switch button cycles personas.
pub fn render_interactive_user_card(
    profile: Option<&Profile>,
    theme: &ThemeTokens,
    on_open_persona_lab: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_switch_persona: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(CARD_ID)
        .w_full()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .child(render_identity(profile, theme, on_open_persona_lab))
        .child(render_switch_button(theme, on_switch_persona))
}

fn render_identity(
    profile: Option<&Profile>,
    theme: &ThemeTokens,
    on_open_persona_lab: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let name = profile.map(|p| p.name.clone()).unwrap_or_else(|| t!("user.unknown").to_string());
    let handle = profile.map(Profile::display_handle).unwrap_or_default();
    let avatar = match profile {
        Some(profile) => render_profile_avatar(profile, theme.avatar_size_sm, theme),
        None => render_avatar(UNKNOWN_INITIAL, &ProfileRole::UserPrimary, theme.avatar_size_sm, theme),
    };
    let text = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .child(div().truncate().text_size(theme.font_size_label).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(name))
        .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(handle));
    div()
        .id(OPEN_PERSONA_LAB_ID)
        .flex_1()
        .min_w_0()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .p(theme.spacing_xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .hover(move |style| style.bg(hover_bg))
        .on_click(on_open_persona_lab)
        .child(avatar)
        .child(text)
}

/// Icon button that cycles the active controllable persona.
fn render_switch_button(theme: &ThemeTokens, on_switch_persona: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Stateful<Div> {
    let (hover_bg, hover_text) = (theme.hover_bg, theme.text_primary);
    div()
        .id(SWITCH_BUTTON_ID)
        .size(theme.nav_rail_button_size)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .rounded(theme.radius_md)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(move |style| style.bg(hover_bg).text_color(hover_text))
        .tooltip(|window, cx| Tooltip::new(t!("user.switch").to_string()).build(window, cx))
        .on_click(on_switch_persona)
        .child(Icon::new(LucideIcon::Repeat).size(theme.icon_size_sm))
}
