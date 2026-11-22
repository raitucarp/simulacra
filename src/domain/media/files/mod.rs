//! Local files attached to a post (epic L). A file is linked, not copied: the post keeps
//! where it was and what it was when attached ([`FileRef`], in `extras.files`), so cards can
//! open it, show it in its folder and notice when it has moved or changed.
//!
//! - `kind`: a file's type from its bytes, then its extension.
//! - `agent`: what an agent reads about attached files (names and facts, never contents).
//! - [`duration`]: how long a video or audio file plays, from its container headers.
//! - `text`: the first lines of a text or code file and the language that colours them.
//! - [`scan`] and `catalog`: what the background scan of a file found, cached by path.

mod agent;
mod catalog;
pub mod duration;
mod file_ref;
mod kind;
pub mod scan;
mod text;

pub use agent::{files_note, with_files_note};
pub use catalog::FileShelf;
pub use duration::{media_duration, ByteSource, Bytes};
pub use file_ref::{FileRef, MAX_FILES_PER_POST};
pub use kind::{detect_mime, file_family, SNIFF_BYTES, UNKNOWN_MIME};
pub use scan::{FileFacts, FilePreview, FileScan};
pub use text::{text_preview, TEXT_PREVIEW_BYTES, TEXT_PREVIEW_LINES};

#[cfg(test)]
mod tests;
