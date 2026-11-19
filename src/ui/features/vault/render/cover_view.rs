//! A document's generated cover at a given width (height follows the 2:3 canvas).

use crate::domain::GroundingDocument;
use crate::theme::ThemeTokens;
use crate::ui::atoms::elevation::ElevationExt;
use crate::ui::features::vault::cover::{cover_image, COVER_HEIGHT, COVER_WIDTH};
use gpui_kit::*;

/// The cover of `doc`, `width` wide, with rounded corners and a soft shadow.
pub fn cover(doc: &GroundingDocument, width: Pixels, theme: &ThemeTokens) -> Div {
    let height = width * (COVER_HEIGHT / COVER_WIDTH);
    div()
        .w(width)
        .h(height)
        .flex_shrink_0()
        .rounded(theme.radius_sm)
        .overflow_hidden()
        .elevation_shadow(theme)
        .child(img(cover_image(&doc.title, doc.record.kind, width)).w(width).h(height))
}
