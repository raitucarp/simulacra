//! A post's images: one at its own aspect ratio (portraits sized by height), or a two-column
//! grid of 2–4 tiles laid out by [`gallery_layout`]. Clicking an image opens the lightbox.

use super::consts::GALLERY_TILE_ID;
use super::image_tile::{image_tile, resolve};
use super::{GalleryOpen, OpenMediaAction};
use crate::domain::media::{gallery_layout, is_portrait, media_aspect, ImageAttachment, MediaCatalog};
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, InteractiveElement, MouseButton, ParentElement, StatefulInteractiveElement, Styled};
use std::rc::Rc;

/// The images of a post as one framed block.
pub(super) fn render_gallery(images: Vec<ImageAttachment>, catalog: Option<&MediaCatalog>, theme: &ThemeTokens, on_open: Option<OpenMediaAction>) -> Div {
    let images: Rc<[ImageAttachment]> = images.into();
    let layout = gallery_layout(images.len());
    let single_size = images.first().and_then(|image| resolve(image, catalog).1);
    let aspect = media_aspect(images.len(), single_size);
    let tiles = layout.tiles.iter().enumerate().map(|(index, tile)| {
        let open = on_open.clone().map(|open| (open, GalleryOpen { images: images.clone(), index }));
        div()
            .id((GALLERY_TILE_ID, index))
            .min_w_0()
            .min_h_0()
            .col_span(tile.column_span)
            .row_span(tile.row_span)
            .child(image_tile(&images[index], catalog, tile.corners(layout.columns, layout.rows), theme))
            .when_some(open, |tile, (open, target)| {
                tile.cursor_pointer()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_click(move |_, window, cx| open(target.clone(), window, cx))
            })
    });
    let frame = div()
        .aspect_ratio(aspect)
        .max_w_full()
        .rounded(theme.radius_lg)
        .overflow_hidden()
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.border_subtle)
        .grid()
        .grid_cols(layout.columns)
        .grid_rows(layout.rows)
        .gap(theme.media_gallery_gap)
        .children(tiles);
    if images.len() == 1 && is_portrait(aspect) {
        frame.h(theme.media_single_max_height)
    } else {
        frame.w_full()
    }
}
