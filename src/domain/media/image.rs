//! Image attachments of a post (with alt text and the pixel size when known), and what the
//! background scan learned about an image file: its size and its cached thumbnail.

use super::gallery::MAX_GALLERY_IMAGES;
use crate::domain::Post;
use serde::{Deserialize, Serialize};

/// Pixel size of an image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl ImageSize {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Width over height; `None` for a degenerate size.
    pub fn aspect(self) -> Option<f32> {
        (self.width > 0 && self.height > 0).then(|| self.width as f32 / self.height as f32)
    }
}

/// One image attached to a post.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ImageAttachment {
    /// The file in the media folder (or wherever an older post pointed).
    pub path: String,
    /// Alternative text read out in place of the image; empty when not written.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,
}

impl ImageAttachment {
    /// An attachment of the file at `path` with no alt text or known size.
    pub fn at(path: impl Into<String>) -> Self {
        Self { path: path.into(), ..Self::default() }
    }
}

/// What the background scan learned about an image file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageFacts {
    pub size: ImageSize,
    /// A smaller copy for the feed; `None` when the original is small enough (or animated).
    pub thumb_path: Option<String>,
}

impl ImageFacts {
    /// The file the feed should draw for `source`.
    pub fn display_path<'a>(&'a self, source: &'a str) -> &'a str {
        self.thumb_path.as_deref().unwrap_or(source)
    }
}

/// The images of `post`, at most a gallery's worth: its attachments, or else the single
/// image an agent generated or a capture saved (`image_path`).
pub fn post_images(post: &Post) -> Vec<ImageAttachment> {
    let attached: Vec<ImageAttachment> = post.extras.images.iter().filter(|image| !image.path.trim().is_empty()).take(MAX_GALLERY_IMAGES).cloned().collect();
    if !attached.is_empty() {
        return attached;
    }
    let legacy = post.image_path.as_deref().map(str::trim).filter(|path| !path.is_empty());
    legacy.map(ImageAttachment::at).into_iter().collect()
}
