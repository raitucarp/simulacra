//! Local files attached to a post (epic L). A file is linked, not copied: the post keeps
//! where it was and what it was when attached ([`FileRef`], in `extras.files`), so cards can
//! open it, show it in its folder and notice when it has moved or changed.
//!
//! - `kind`: a file's type from its bytes, then its extension.
//! - [`duration`]: how long a video plays, from its container headers.
//! - [`scan`] and `catalog`: what the background scan of a file found, cached by path.

mod catalog;
pub mod duration;
mod file_ref;
mod kind;
pub mod scan;

pub use catalog::FileShelf;
pub use duration::{media_duration, ByteSource, Bytes};
pub use file_ref::{FileRef, MAX_FILES_PER_POST};
pub use kind::{detect_mime, file_family, SNIFF_BYTES, UNKNOWN_MIME};
pub use scan::{FileFacts, FilePreview, FileScan};

#[cfg(test)]
mod tests;
