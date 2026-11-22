//! Scanning an attached file for its card, off the UI thread: whether it is still there, and
//! the preview its type gets (L7: a video's poster frame from the OS and its length; L8: an audio
//! file's length, both read from the container headers; L9: a text file's first lines;
//! L10: a PDF's or EPUB's outline, read by the import parsers).

use super::source::FileBytes;
use crate::domain::media::files::{file_family, media_duration, text_preview, ByteSource, FileFacts, FilePreview, FileScan, TEXT_PREVIEW_BYTES};
use crate::domain::media::FileRef;
use crate::import::detect::format_of_mime;
use crate::import::parse::{document_outline, DocumentOutline};
use crate::import::FormatFamily;
use std::path::Path;

/// Largest PDF or EPUB read for its outline.
const MAX_DOCUMENT_BYTES: u64 = 64 * 1024 * 1024;

/// What the card of `file` can show now.
pub fn scan_file(file: &FileRef, media_dir: &Path) -> FileScan {
    let path = Path::new(&file.path);
    let Some(mut bytes) = FileBytes::open(path) else { return FileScan::Missing };
    let preview = match file_family(&file.mime) {
        FormatFamily::Video => FilePreview::Video { poster: super::poster::poster_for(file, media_dir), seconds: media_duration(&file.mime, &mut bytes) },
        FormatFamily::Audio => FilePreview::Audio { seconds: media_duration(&file.mime, &mut bytes) },
        FormatFamily::Text | FormatFamily::Data | FormatFamily::Web => text(file, &mut bytes),
        FormatFamily::Pdf | FormatFamily::Book => document(file, &bytes),
        _ => FilePreview::None,
    };
    FileScan::Present(FileFacts { preview })
}

/// The first lines of a text file (L9: only [`TEXT_PREVIEW_BYTES`] are read).
fn text(file: &FileRef, bytes: &mut FileBytes) -> FilePreview {
    match bytes.read_at(0, TEXT_PREVIEW_BYTES) {
        Some(head) => text_preview(&head, bytes.len(), file.extension().as_deref()),
        None => FilePreview::None,
    }
}

/// The outline the import parsers read from a PDF or EPUB (L10), up to [`MAX_DOCUMENT_BYTES`].
fn document(file: &FileRef, bytes: &FileBytes) -> FilePreview {
    let format = format_of_mime(&file.mime).filter(|_| bytes.len() <= MAX_DOCUMENT_BYTES);
    let outline = format.and_then(|format| std::fs::read(&file.path).ok().and_then(|all| document_outline(format, &all)));
    match outline {
        Some(DocumentOutline { metadata, pages, chapters }) => FilePreview::Document { title: metadata.title, creators: metadata.creators, pages, chapters },
        None => FilePreview::None,
    }
}
