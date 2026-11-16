//! Atomic avatar circle component.
//! Supports both custom uploaded image files (`avatar_path`) and initial-based role badges.
//! Fully tokenized via ThemeTokens.

use crate::domain::{Profile, ProfileRole};
use crate::theme::ThemeTokens;
use gpui_kit::*;
use std::path::PathBuf;

/// Avatars at or below this diameter use the 2xs initial font size.
const SMALL_AVATAR_MAX_PX: f32 = 24.0;
/// Avatars at or below this diameter (and above the small cap) use the xs font size.
const MEDIUM_AVATAR_MAX_PX: f32 = 36.0;

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

    let size_f32 = size.as_f32();
    let font_size = if size_f32 <= SMALL_AVATAR_MAX_PX {
        theme.font_size_2xs
    } else if size_f32 <= MEDIUM_AVATAR_MAX_PX {
        theme.font_size_xs
    } else {
        theme.font_size_base
    };

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

/// Renders a profile avatar, using the uploaded image at `profile.avatar_path` if present,
/// or falling back to the role-colored initial badge.
pub fn render_profile_avatar(
    profile: &Profile,
    size: Pixels,
    theme: &ThemeTokens,
) -> Div {
    if let Some(ref path_str) = profile.avatar_path {
        let trimmed = path_str.trim();
        if !trimmed.is_empty() {
            let path = PathBuf::from(trimmed);
            if path.exists() {
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
        }
    }

    render_avatar(&profile.avatar_initial, &profile.role, size, theme)
}
