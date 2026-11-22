//! Post media (sb C3), all pure:
//!
//! - [`links`]: the web links of a post (its attached link, then the URLs in its text).
//! - [`video`]: video links (the major video host's watch, short and embed forms, video files).
//! - [`preview`]: a link preview, its cache entry and expiry.
//! - [`opengraph`]: a tolerant OpenGraph / Twitter-card / `<title>` reader over fetched HTML.
//! - [`image`]: image attachments of a post and what is known about an image file.
//! - [`gallery`]: tile layout of a 2–4 image gallery, aspect clamping and lightbox stepping.
//! - [`catalog`]: the in-memory cache of previews and image facts, and what to fetch next.
//! - [`plan`]: what the posts on screen need fetched or scanned.
//!
//! Fetching, decoding and storage happen at the edges (`crate::media`, `crate::storage`).

pub mod catalog;
pub mod gallery;
pub mod image;
pub mod links;
pub mod opengraph;
pub mod plan;
pub mod preview;
pub mod video;

#[cfg(test)]
mod tests;

pub use catalog::{ImageScan, MediaCatalog, PreviewLookup};
pub use gallery::{clamp_aspect, fit_within, Corners, gallery_layout, is_portrait, media_aspect, step_index, GalleryLayout, Step, Tile, MAX_GALLERY_IMAGES};
pub use image::{post_images, ImageAttachment, ImageFacts, ImageSize};
pub use links::{detect_urls, display_domain, link_file_name, post_links, written_url};
pub use opengraph::{decode_html, parse_link_preview};
pub use plan::{media_wants, MediaWants, WantedLink};
pub use preview::{LinkPreview, PreviewEntry};
pub use video::{format_duration, recognise_video, VideoLink, VideoSource};
