//! L9: a text or code file attached to a post: its first lines in the code block card (the
//! highlighter colours them by the file's extension, B5 / B6), then the file card.

use super::consts::FILE_CARD_ID;
use super::file_card::file_card_with;
use crate::domain::media::files::FilePreview;
use crate::domain::media::{FileRef, MediaCatalog};
use crate::import::FormatFamily;
use crate::theme::ThemeTokens;
use crate::ui::features::post::render_code_block;
use gpui_kit::{div, AnyElement, InteractiveElement, IntoElement, ParentElement, Styled};

/// The line that says the file goes on.
const MORE_LINE: &str = "\n…";

/// The card of the `index`-th file of a post, a text file of `family`.
pub(super) fn text_file(index: usize, file: &FileRef, family: FormatFamily, catalog: Option<&MediaCatalog>, theme: &ThemeTokens) -> AnyElement {
    let card = file_card_with(index, file, family, None, theme);
    let Some(FilePreview::Text { lines, lang, more }) = catalog.and_then(|catalog| catalog.file(&file.path)).and_then(|scan| scan.preview()) else {
        return card.into_any_element();
    };
    let shown = if *more { format!("{lines}{MORE_LINE}") } else { lines.clone() };
    div()
        .id((FILE_CARD_ID, index))
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .child(render_code_block(index, lang.as_deref(), &shown, theme))
        .child(card)
        .into_any_element()
}
