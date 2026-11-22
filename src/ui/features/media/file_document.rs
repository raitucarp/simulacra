//! L10: a PDF or an EPUB attached to a post: the file card led by the title and authors the
//! import parsers read from it, its facts line ending in its pages (PDF) or chapters (EPUB).

use super::consts::TITLE_LINES;
use super::file_card::{card_frame, file_text};
use crate::domain::media::files::FilePreview;
use crate::domain::media::{FileRef, MediaCatalog};
use crate::i18n::t_count;
use crate::import::FormatFamily;
use crate::theme::ThemeTokens;
use crate::ui::features::import::family_art;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::{div, AnyElement, FontWeight, IntoElement, ParentElement, Styled};

/// Separator between a document's authors.
const CREATORS_SEPARATOR: &str = ", ";

/// The card of the `index`-th file of a post, a PDF or EPUB of `family`.
pub(super) fn document_file(index: usize, file: &FileRef, family: FormatFamily, catalog: Option<&MediaCatalog>, theme: &ThemeTokens) -> AnyElement {
    let outline = catalog.and_then(|catalog| catalog.file(&file.path)).and_then(|scan| scan.preview());
    let (title, creators, size_line) = match outline {
        Some(FilePreview::Document { title, creators, pages, chapters }) => (title.clone(), creators.join(CREATORS_SEPARATOR), extent(*pages, *chapters)),
        _ => (None, String::new(), None),
    };
    let lead = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .children(title.map(|title| div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).line_clamp(TITLE_LINES).child(title)))
        .children((!creators.is_empty()).then(|| div().text_size(theme.font_size_xs).text_color(theme.text_secondary).truncate().child(creators)))
        .child(file_text(index, file, family, size_line, theme));
    let art = Illustration::asset(family_art(family), IllustrationKind::TraitCard).width(theme.file_card_art_width);
    card_frame(index, theme).child(div().flex_none().child(art)).child(lead).into_any_element()
}

/// "12 pages" or "8 chapters".
fn extent(pages: Option<usize>, chapters: Option<usize>) -> Option<String> {
    pages.map(|count| t_count("media.file.pages", count)).or_else(|| chapters.map(|count| t_count("media.file.chapters", count)))
}
