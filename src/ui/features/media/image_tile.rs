//! One image filling its tile: the cached thumbnail when the scan made one (the original
//! otherwise), cropped to the tile, with an "ALT" badge when it has a description and the
//! missing-image art when the file is gone.

use crate::domain::media::catalog::ImageScan;
use crate::domain::media::{Corners, ImageAttachment, ImageSize, MediaCatalog};
use crate::theme::ThemeTokens;
use crate::ui::features::post::missing_image;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, img, AbsoluteLength, Div, FontWeight, IntoElement, ObjectFit, ParentElement, Pixels, Styled, StyledImage};
use rust_i18n::t;
use std::path::PathBuf;

/// The file to draw for `image` and its pixel size, from the attachment or the scan.
pub(super) fn resolve(image: &ImageAttachment, catalog: Option<&MediaCatalog>) -> (PathBuf, Option<ImageSize>) {
    match catalog.and_then(|catalog| catalog.image(&image.path)) {
        Some(ImageScan::Known(facts)) => (PathBuf::from(facts.display_path(&image.path)), image.size.or(Some(facts.size))),
        _ => (PathBuf::from(&image.path), image.size),
    }
}

/// `element` with the frame's radius on the grid `corners` it sits in and square elsewhere
/// (content masks are rectangular, so images round their own corners).
pub(super) fn round_corners<E: Styled>(mut element: E, corners: Corners, radius: Pixels) -> E {
    let radius_of = |rounded: bool| Some(AbsoluteLength::Pixels(if rounded { radius } else { Pixels::ZERO }));
    let [top_left, top_right, bottom_left, bottom_right] = corners;
    let radii = &mut element.style().corner_radii;
    (radii.top_left, radii.top_right) = (radius_of(top_left), radius_of(top_right));
    (radii.bottom_left, radii.bottom_right) = (radius_of(bottom_left), radius_of(bottom_right));
    element
}

/// `image` cropped to fill its parent, rounded on the frame `corners` it touches. The image is
/// pinned to the tile's edges, so its height never falls back to its own aspect ratio.
pub(super) fn image_tile(image: &ImageAttachment, catalog: Option<&MediaCatalog>, corners: Corners, theme: &ThemeTokens) -> Div {
    let (path, _) = resolve(image, catalog);
    let loading_bg = theme.hover_bg;
    let radius = theme.radius_lg;
    round_corners(div(), corners, radius)
        .relative()
        .size_full()
        .overflow_hidden()
        .bg(theme.bg_elevated)
        .child(
            round_corners(img(path), corners, radius)
                .absolute()
                .inset_0()
                .size_full()
                .object_fit(ObjectFit::Cover)
                .with_loading(move || div().size_full().bg(loading_bg).into_any_element())
                .with_fallback(missing_image(theme)),
        )
        .when(!image.alt.trim().is_empty(), |tile| tile.child(alt_badge(theme)))
}

/// The "ALT" badge in a tile's corner.
fn alt_badge(theme: &ThemeTokens) -> Div {
    div()
        .absolute()
        .left(theme.spacing_xs)
        .bottom(theme.spacing_xs)
        .px(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .bg(theme.bg_scrim)
        .text_color(theme.on_overlay)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::BOLD)
        .child(t!("media.alt_badge").to_string())
}
