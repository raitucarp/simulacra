//! Scanning an attached file for its card, off the UI thread: whether it is still there, and
//! the preview its type gets (L7: a video's poster frame from the OS and its length, read from
//! the container headers).

use super::source::FileBytes;
use crate::domain::media::files::{file_family, media_duration, FileFacts, FilePreview, FileScan};
use crate::domain::media::FileRef;
use crate::import::FormatFamily;
use std::path::Path;

/// What the card of `file` can show now.
pub fn scan_file(file: &FileRef, media_dir: &Path) -> FileScan {
    let path = Path::new(&file.path);
    let Some(mut bytes) = FileBytes::open(path) else { return FileScan::Missing };
    let preview = match file_family(&file.mime) {
        FormatFamily::Video => FilePreview::Video { poster: super::poster::poster_for(file, media_dir), seconds: media_duration(&file.mime, &mut bytes) },
        _ => FilePreview::None,
    };
    FileScan::Present(FileFacts { preview })
}
