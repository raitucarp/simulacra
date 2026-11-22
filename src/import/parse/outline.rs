//! L10: what a PDF or an EPUB says about itself without its text being read: the import
//! parsers' metadata (title, authors), and its pages (PDF) or chapters (EPUB).

use super::{epub, pdf};
use crate::import::detect::Format;
use crate::import::document::DocumentMetadata;

/// A document's metadata and its extent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DocumentOutline {
    pub metadata: DocumentMetadata,
    /// Pages of a PDF.
    pub pages: Option<usize>,
    /// Spine documents of an EPUB.
    pub chapters: Option<usize>,
}

/// The outline of `bytes` read as `format`; `None` for other formats and unreadable files.
pub fn document_outline(format: Format, bytes: &[u8]) -> Option<DocumentOutline> {
    match format {
        Format::Pdf => pdf::outline(bytes).ok().map(|(metadata, pages)| DocumentOutline { metadata, pages: Some(pages), chapters: None }),
        Format::Epub => epub::outline(bytes).ok().map(|(metadata, chapters)| DocumentOutline { metadata, pages: None, chapters: Some(chapters) }),
        _ => None,
    }
}
