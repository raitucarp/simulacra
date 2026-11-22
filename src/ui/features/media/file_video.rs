//! L7: a video file attached to a post. With a poster frame from the OS (L6) it is a video
//! card: the frame, the play badge and the duration over the file's row; a click on the
//! frame plays it in the system player. Without one the fallback is the file card, its
//! facts line carrying the duration.

use super::consts::{FILE_CARD_ID, FILE_PLAY_ID};
use super::file_card::{file_card_with, file_text};
use super::video_card::frame;
use crate::domain::media::files::FilePreview;
use crate::domain::media::{format_duration, FileRef, MediaCatalog};
use crate::import::FormatFamily;
use crate::theme::ThemeTokens;
use gpui_kit::{div, AnyElement, InteractiveElement, IntoElement, MouseButton, ParentElement, StatefulInteractiveElement, Styled};
use std::path::PathBuf;

/// The card of the `index`-th file of a post, a video.
pub(super) fn video_file(index: usize, file: &FileRef, catalog: Option<&MediaCatalog>, theme: &ThemeTokens) -> AnyElement {
    let (poster, seconds) = match catalog.and_then(|catalog| catalog.file(&file.path)).and_then(|scan| scan.preview()) {
        Some(FilePreview::Video { poster, seconds }) => (poster.clone(), *seconds),
        _ => (None, None),
    };
    let duration = seconds.map(format_duration);
    let Some(poster) = poster else {
        return file_card_with(index, file, FormatFamily::Video, duration, theme).into_any_element();
    };
    let path = PathBuf::from(&file.path);
    let play = div()
        .id((FILE_PLAY_ID, index))
        .w_full()
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(move |_, window, cx| super::file_card::report(crate::media::files::open_file(&path), window, cx))
        .child(frame(Some(poster), seconds, theme));
    div()
        .id((FILE_CARD_ID, index))
        .w_full()
        .min_w_0()
        .rounded(theme.radius_lg)
        .overflow_hidden()
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .child(play)
        .child(div().p(theme.card_padding).child(file_text(index, file, FormatFamily::Video, duration, theme)))
        .into_any_element()
}
