//! Scanning an attached file for its card, off the UI thread: whether it is still there, and
//! the preview its type gets (L7: a video's poster frame from the OS and its length; L8: an audio
//! file's length, both read from the container headers; L9: a text file's first lines).

use super::source::FileBytes;
use crate::domain::media::files::{file_family, media_duration, text_preview, ByteSource, FileFacts, FilePreview, FileScan, TEXT_PREVIEW_BYTES};
use crate::domain::media::FileRef;
use crate::import::FormatFamily;
use std::path::Path;

/// What the card of `file` can show now.
pub fn scan_file(file: &FileRef, media_dir: &Path) -> FileScan {
    let path = Path::new(&file.path);
    let Some(mut bytes) = FileBytes::open(path) else { return FileScan::Missing };
    let preview = match file_family(&file.mime) {
        FormatFamily::Video => FilePreview::Video { poster: super::poster::poster_for(file, media_dir), seconds: media_duration(&file.mime, &mut bytes) },
        FormatFamily::Audio => FilePreview::Audio { seconds: media_duration(&file.mime, &mut bytes) },
        FormatFamily::Text | FormatFamily::Data | FormatFamily::Web => text(file, &mut bytes),
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
