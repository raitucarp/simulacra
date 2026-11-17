//! Warming up illustrations before the main window is shown: every profile's avatar is
//! themed and handed to gpui's (app-wide) image cache, which rasterizes it in the background.

use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::illustration::avatar_source;
use gpui_kit::gpui::Image;
use gpui_kit::{App, ImageSource, Pixels, Window};
use std::sync::Arc;

/// Avatar sizes the shell's first screen draws (post rows, sidebars, the user card).
fn avatar_sizes(theme: &ThemeTokens) -> [Pixels; 4] {
    [theme.avatar_size_sm, theme.avatar_size_md, theme.avatar_size_lg, theme.contact_avatar_size]
}

/// Themed avatar images of every profile at every first-screen size (photos stored as plain
/// files load on demand and are left out).
pub(super) fn avatar_images(state: &AppState, theme: &ThemeTokens) -> Vec<Arc<Image>> {
    let modes = state.user_profiles.iter().filter_map(|profile| profile.config.avatar.as_ref());
    modes
        .flat_map(|mode| avatar_sizes(theme).into_iter().filter_map(move |size| avatar_source(mode, size)))
        .filter_map(|source| match source {
            ImageSource::Image(image) => Some(image),
            _ => None,
        })
        .collect()
}

/// Starts rasterizing `images` and keeps only those still loading.
pub(super) fn still_loading(images: Vec<Arc<Image>>, window: &mut Window, cx: &mut App) -> Vec<Arc<Image>> {
    images.into_iter().filter(|image| image.clone().get_render_image(window, cx).is_none()).collect()
}
