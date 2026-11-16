//! User identity card with Persona Lab opener and quick persona switcher trigger molecule.

use crate::domain::{Profile, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_profile_avatar};
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// Avatar initial shown when no profile is active.
const UNKNOWN_INITIAL: &str = "?";
/// Handle shown when no profile is active.
const UNKNOWN_HANDLE: &str = "@user";
const CARD_ID: &str = "sidebar-user-card-container";
const OPEN_PERSONA_LAB_ID: &str = "sidebar-user-card-open-persona-lab";
const SWITCH_BUTTON_ID: &str = "sidebar-user-card-switch-btn";

/// Renders the interactive user identity card where clicking the user opens Persona Lab
/// and clicking the Switch button cycles the active controllable human persona in the network.
pub fn render_interactive_user_card(
    profile: Option<&Profile>,
    theme: &ThemeTokens,
    on_open_persona_lab: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_switch_persona: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(CARD_ID)
        .w_full()
        .p(theme.spacing_sm)
        .rounded(theme.radius_lg)
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_xs)
        .child(render_identity(profile, theme, on_open_persona_lab))
        .child(render_switch_button(theme, on_switch_persona))
}

/// Avatar + name/handle block; clicking opens the Persona Lab.
fn render_identity(
    profile: Option<&Profile>,
    theme: &ThemeTokens,
    on_open_persona_lab: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(OPEN_PERSONA_LAB_ID)
        .flex_1()
        .min_w_0()
        .p(theme.spacing_2xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(on_open_persona_lab)
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(render_identity_avatar(profile, theme))
        .child(render_identity_text(profile, theme))
}

fn render_identity_avatar(profile: Option<&Profile>, theme: &ThemeTokens) -> Div {
    if let Some(prof) = profile {
        render_profile_avatar(prof, theme.avatar_size_md, theme)
    } else {
        render_avatar(UNKNOWN_INITIAL, &ProfileRole::UserPrimary, theme.avatar_size_md, theme)
    }
}

fn render_identity_text(profile: Option<&Profile>, theme: &ThemeTokens) -> Div {
    let name = profile
        .map(|p| p.name.clone())
        .unwrap_or_else(|| t!("user.unknown").to_string());
    let handle = profile
        .map(|p| p.handle.clone())
        .unwrap_or_else(|| UNKNOWN_HANDLE.to_string());
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_bold()
                .text_color(theme.text_primary)
                .truncate()
                .child(name),
        )
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .truncate()
                .child(handle),
        )
}

/// "Switch" pill that cycles the active controllable persona.
fn render_switch_button(
    theme: &ThemeTokens,
    on_switch_persona: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let border_focus = theme.border_focus;
    let text_primary = theme.text_primary;
    div()
        .id(SWITCH_BUTTON_ID)
        .flex_shrink_0()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.hover_bg)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .text_size(theme.font_size_2xs)
        .font_medium()
        .text_color(theme.text_secondary)
        .cursor_pointer()
        .hover(move |s| s.text_color(text_primary).border_color(border_focus))
        .on_click(on_switch_persona)
        .child(
            div()
                .w(theme.font_size_xs)
                .h(theme.font_size_xs)
                .child(Icon::new(LucideIcon::UserCheck)),
        )
        .child(t!("user.switch").to_string())
}
