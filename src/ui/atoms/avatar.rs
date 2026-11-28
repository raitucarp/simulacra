//! Atomic avatar circle component.
//! Supports generated portraits, emblems and cropped photos (`config.avatar`), uploaded image
//! files (`avatar_path`) and initial-based role badges.
//! Fully tokenized via ThemeTokens.

use crate::domain::{Profile, ProfileRole};
use crate::theme::ThemeTokens;
use gpui_kit::*;
use super::avatar_cache::resolve_avatar;
use crate::ui::illustration::render_avatar_mode;

/// Past the medium cap an initial fills this share of the circle, so a large profile avatar
/// reads as a letter and not as an empty ring.
const LARGE_INITIAL_SCALE: f32 = 0.42;

/// The initial's size: it steps up as the avatar crosses the small and medium caps, then grows
/// with the circle.
fn initial_size(size: Pixels, theme: &ThemeTokens) -> Pixels {
    if size <= theme.avatar_initial_small_max {
        theme.font_size_2xs
    } else if size <= theme.avatar_initial_medium_max {
        theme.font_size_xs
    } else if size * LARGE_INITIAL_SCALE > theme.font_size_base {
        size * LARGE_INITIAL_SCALE
    } else {
        theme.font_size_base
    }
}

/// Renders a circular profile avatar with initial and role-based coloring.
pub fn render_avatar(
    initial: &str,
    role: &ProfileRole,
    size: Pixels,
    theme: &ThemeTokens,
) -> Div {
    let accent = match role {
        ProfileRole::UserPrimary => theme.accent_user,
        ProfileRole::UserAlt => theme.accent_alt,
        ProfileRole::Agent { .. } => theme.accent_agent,
    };

    let font_size = initial_size(size, theme);

    div()
        .w(size)
        .h(size)
        .flex_shrink_0()
        .rounded(theme.radius_full)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .text_size(font_size)
        .font_weight(gpui_kit::FontWeight::BOLD)
        .text_color(accent)
        .flex()
        .items_center()
        .justify_center()
        .overflow_hidden()
        .child(initial.to_string())
}

/// Renders a profile avatar: the chosen [`AvatarMode`](crate::domain::portrait::AvatarMode)
/// (`profile.config.avatar`) if it can be drawn, else the uploaded image at
/// `profile.avatar_path`, else the role-colored initial badge.
pub fn render_profile_avatar(
    profile: &Profile,
    size: Pixels,
    theme: &ThemeTokens,
) -> Div {
    if let Some(avatar) = profile.config.avatar.as_ref().and_then(|mode| render_avatar_mode(mode, size, theme)) {
        return avatar;
    }
    if let Some(path) = profile.avatar_path.as_deref().and_then(resolve_avatar) {
        return div()
            .w(size)
            .h(size)
            .flex_shrink_0()
            .rounded(theme.radius_full)
            .bg(theme.bg_card)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_center()
            .child(
                img(path)
                    .w(size)
                    .h(size)
                    .rounded(theme.radius_full)
                    .object_fit(ObjectFit::Cover),
            );
    }

    render_avatar(&profile.avatar_initial, &profile.role, size, theme)
}
