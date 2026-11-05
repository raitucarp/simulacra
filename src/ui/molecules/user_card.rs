//! User identity card with Persona Lab opener and quick persona switcher trigger molecule.

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_profile_avatar};
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// Renders the static user identity card.
pub fn render_user_card(profile: Option<&Profile>, theme: &ThemeTokens) -> impl IntoElement {
    render_interactive_user_card(
        profile,
        theme,
        |_, _, _| {},
        |_, _, _| {},
    )
}

/// Renders the interactive user identity card where clicking the user opens Persona Lab
/// and clicking the Switch button cycles the active controllable human persona in the network.
pub fn render_interactive_user_card(
    profile: Option<&Profile>,
    theme: &ThemeTokens,
    on_open_persona_lab: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_switch_persona: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let initial = profile.map(|p| p.avatar_initial.as_str()).unwrap_or("?");
    let role = profile
        .map(|p| &p.role)
        .unwrap_or(&crate::domain::ProfileRole::UserPrimary);
    let name = profile
        .map(|p| p.name.clone())
        .unwrap_or_else(|| t!("user.unknown").to_string());
    let handle = profile
        .map(|p| p.handle.clone())
        .unwrap_or_else(|| "@user".to_string());

    let hover_bg = theme.hover_bg;
    let border_focus = theme.border_focus;

    div()
        .id("sidebar-user-card-container")
        .w_full()
        .p(theme.spacing_sm)
        .rounded(theme.radius_lg)
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_xs)
        .child(
            div()
                .id("sidebar-user-card-open-persona-lab")
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
                .child(if let Some(prof) = profile {
                    render_profile_avatar(prof, theme.avatar_size_md, theme)
                } else {
                    render_avatar(initial, role, theme.avatar_size_md, theme)
                })
                .child(
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
                        ),
                ),
        )
        .child(
            div()
                .id("sidebar-user-card-switch-btn")
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
                .hover(move |s| s.text_color(theme.text_primary).border_color(border_focus))
                .on_click(on_switch_persona)
                .child(
                    div()
                        .w(theme.font_size_xs)
                        .h(theme.font_size_xs)
                        .child(Icon::new(LucideIcon::UserCheck)),
                )
                .child(t!("user.switch").to_string()),
        )
}
