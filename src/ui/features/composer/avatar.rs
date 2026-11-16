//! Author avatar shared by both composers: the profile picture, or a role-colored initial.

use crate::domain::{Profile, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_profile_avatar};
use gpui_kit::{Div, Pixels};

/// Renders the active profile's avatar, or a primary-user initial badge when none is set.
pub(super) fn composer_avatar(
    profile: Option<&Profile>,
    fallback_initial: &str,
    size: Pixels,
    theme: &ThemeTokens,
) -> Div {
    match profile {
        Some(prof) => render_profile_avatar(prof, size, theme),
        None => render_avatar(fallback_initial, &ProfileRole::UserPrimary, size, theme),
    }
}
