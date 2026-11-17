//! [`PortraitView`]: a circular avatar for any [`AvatarMode`] at any size.

use super::form::form_image;
use super::photo::{is_croppable, photo_image};
use super::portrait::{emblem_image, portrait_image};
use super::template::raster_width;
use crate::domain::portrait::{AvatarMode, PhotoCrop};
use crate::storage::avatars::{resolve_photo_path, vault_dir};
use crate::theme::ThemeTokens;
use crate::ui::atoms::avatar_cache::resolve_avatar;
use gpui_kit::*;

/// The image source of `mode` rasterized for a `size`-wide circle, or `None` when a photo
/// file is missing (callers fall back to the initial badge).
pub fn avatar_source(mode: &AvatarMode, size: Pixels) -> Option<ImageSource> {
    let width = Some(raster_width(size.as_f32()));
    match mode {
        AvatarMode::Pcg(spec) => Some(portrait_image(spec, width).into()),
        AvatarMode::Form(spec) => Some(form_image(spec, width).into()),
        AvatarMode::Emblem(spec) => Some(emblem_image(spec, width).into()),
        AvatarMode::Photo { path, crop } => photo_source(path, *crop, width),
    }
}

/// A stored photo path resolved inside the vault; croppable formats are framed by `crop`,
/// others are shown whole (cover-fit).
fn photo_source(stored: &str, crop: PhotoCrop, width: Option<u16>) -> Option<ImageSource> {
    let file = resolve_photo_path(vault_dir().as_deref(), stored);
    let found = resolve_avatar(&file.to_string_lossy())?;
    if is_croppable(&found) {
        Some(photo_image(&found, crop, width).into())
    } else {
        Some(found.into())
    }
}

/// Renders `mode` as a `size` circle framed like the initial badge; `None` when it has no
/// image (missing photo).
pub fn render_avatar_mode(mode: &AvatarMode, size: Pixels, theme: &ThemeTokens) -> Option<Div> {
    let source = avatar_source(mode, size)?;
    Some(
        div()
            .size(size)
            .flex_shrink_0()
            .rounded(theme.radius_full)
            .bg(theme.bg_card)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .overflow_hidden()
            .child(img(source).size(size).rounded(theme.radius_full).object_fit(ObjectFit::Cover)),
    )
}

/// Element form of [`render_avatar_mode`], falling back to an empty themed circle.
#[derive(IntoElement)]
pub struct PortraitView {
    mode: AvatarMode,
    size: Pixels,
    theme: ThemeTokens,
}

impl PortraitView {
    /// A `size` avatar of `mode` styled with `theme`.
    pub fn new(mode: AvatarMode, size: Pixels, theme: &ThemeTokens) -> Self {
        Self { mode, size, theme: theme.clone() }
    }
}

impl RenderOnce for PortraitView {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        render_avatar_mode(&self.mode, self.size, &self.theme).unwrap_or_else(|| {
            div().size(self.size).flex_shrink_0().rounded(self.theme.radius_full).bg(self.theme.bg_card)
        })
    }
}
