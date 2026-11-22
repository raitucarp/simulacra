//! Post media at the edges (sb C3): fetching link previews over HTTP and decoding, scaling and
//! storing images in the media folder. Everything here blocks and runs on the background
//! executor; the decisions (what a page says, what to fetch, how to lay out) are pure and live
//! in [`crate::domain::media`].
//!
//! - [`fetch`]: one link preview (page, then its image), capped in size, time and redirects,
//!   `http(s)` only, without cookies, referrers or anything about the user.
//! - [`images`]: attachments stored by content hash, image scans and cached thumbnails.
//! - [`files`]: local files linked to posts (epic L): their references.

pub mod fetch;
pub mod files;
pub mod images;

#[cfg(test)]
mod tests;

pub use fetch::{fetch_preview_blocking, PreviewJob};
pub use images::{scan_image, store_attachment_bytes, store_attachment_file};
