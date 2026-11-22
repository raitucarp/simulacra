//! L5: an image file attached to a post, previewed through the image pipeline: the background
//! scan learns its size and caches a feed thumbnail, the gallery draws it at its aspect ratio
//! and a click opens the lightbox. The file's row (name, facts, Open, Show in folder) follows.

use super::consts::FILE_CARD_ID;
use super::file_card::{card_frame, file_text};
use super::gallery::render_gallery;
use super::OpenMediaAction;
use crate::domain::media::{FileRef, ImageAttachment, MediaCatalog};
use crate::import::FormatFamily;
use crate::theme::ThemeTokens;
use gpui_kit::{div, AnyElement, InteractiveElement, IntoElement, ParentElement, Styled};

/// The preview and row of the `index`-th file of a post, an image.
pub(super) fn image_file(index: usize, file: &FileRef, catalog: Option<&MediaCatalog>, on_media: Option<&OpenMediaAction>, theme: &ThemeTokens) -> AnyElement {
    let preview = render_gallery(vec![ImageAttachment::at(file.path.clone())], catalog, theme, on_media.cloned());
    // The block's own id scopes the gallery's tile ids apart from the post's gallery.
    div()
        .id((FILE_CARD_ID, index))
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .items_start()
        .gap(theme.stack_gap)
        .child(preview)
        .child(card_frame(index, theme).child(file_text(index, file, FormatFamily::Image, None, theme)))
        .into_any_element()
}
