//! L8: an audio file attached to a post: the file card with its length on the facts line,
//! and "Play", which hands it to the system player.

use super::file_card::file_card_with;
use crate::domain::media::files::FilePreview;
use crate::domain::media::{format_duration, FileRef, MediaCatalog};
use crate::import::FormatFamily;
use crate::theme::ThemeTokens;
use gpui_kit::{AnyElement, IntoElement};

/// The card of the `index`-th file of a post, an audio file.
pub(super) fn audio_file(index: usize, file: &FileRef, catalog: Option<&MediaCatalog>, theme: &ThemeTokens) -> AnyElement {
    let seconds = match catalog.and_then(|catalog| catalog.file(&file.path)).and_then(|scan| scan.preview()) {
        Some(FilePreview::Audio { seconds }) => *seconds,
        _ => None,
    };
    file_card_with(index, file, FormatFamily::Audio, seconds.map(format_duration), theme).into_any_element()
}
