//! A link preview read from a page, and its cache entry: when it was fetched, when it goes
//! stale and the local copy of its image. Failures are cached too (for a shorter time) so a
//! dead link is not fetched on every render.

use serde::{Deserialize, Serialize};

/// Seconds a fetched preview stays fresh (a week).
pub const PREVIEW_TTL_SECS: i64 = 7 * 24 * 60 * 60;
/// Seconds a failed fetch is remembered before it is tried again (six hours).
pub const FAILURE_TTL_SECS: i64 = 6 * 60 * 60;

/// What a page says about itself.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LinkPreview {
    pub title: String,
    pub description: String,
    pub site_name: String,
    /// Absolute `http(s)` URL of the preview image.
    pub image_url: Option<String>,
    /// Absolute `http(s)` URL of the site icon.
    pub icon_url: Option<String>,
    /// Whether the page is a video (its card gets a play badge).
    pub is_video: bool,
    pub duration_secs: Option<u32>,
}

impl LinkPreview {
    /// Whether the page said anything worth a card (a title, description or image).
    pub fn has_content(&self) -> bool {
        !self.title.is_empty() || !self.description.is_empty() || self.image_url.is_some()
    }
}

/// One cached fetch of a URL.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreviewEntry {
    pub url: String,
    /// `None` when the fetch failed or the page had nothing to show.
    pub preview: Option<LinkPreview>,
    /// The preview image saved in the media folder.
    pub image_path: Option<String>,
    /// The site icon saved in the media folder.
    pub icon_path: Option<String>,
    /// Unix seconds.
    pub fetched_at: i64,
    pub expires_at: i64,
}

impl PreviewEntry {
    /// A successful fetch at `now`.
    pub fn ready(url: impl Into<String>, preview: LinkPreview, image_path: Option<String>, now: i64) -> Self {
        let preview = Some(preview).filter(LinkPreview::has_content);
        let ttl = if preview.is_some() { PREVIEW_TTL_SECS } else { FAILURE_TTL_SECS };
        Self { url: url.into(), preview, image_path, icon_path: None, fetched_at: now, expires_at: now.saturating_add(ttl) }
    }

    /// The entry with its saved site icon.
    #[must_use]
    pub fn with_icon(self, icon_path: Option<String>) -> Self {
        Self { icon_path, ..self }
    }

    /// A failed fetch at `now`.
    pub fn failed(url: impl Into<String>, now: i64) -> Self {
        Self { url: url.into(), preview: None, image_path: None, icon_path: None, fetched_at: now, expires_at: now.saturating_add(FAILURE_TTL_SECS) }
    }

    /// Whether the entry is still within its lifetime at `now`.
    pub fn is_fresh(&self, now: i64) -> bool {
        now < self.expires_at
    }

    /// Whether there is a card to show (stale entries still show until refreshed).
    pub fn is_ready(&self) -> bool {
        self.preview.is_some()
    }
}
