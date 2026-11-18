//! One link preview job: read the page (capped), parse what it says about itself, then fetch
//! its image (capped), scale it and save it in the media folder. Hosted videos fall back to
//! their still frame when the page cannot be read.

mod client;
mod read;

pub use client::{preview_client, FETCH_TIMEOUT, MAX_REDIRECTS};
pub use read::{MAX_ICON_BYTES, MAX_IMAGE_BYTES, MAX_PAGE_BYTES};

use super::images::{save_icon_image, save_preview_image};
use crate::domain::media::{recognise_video, LinkPreview, PreviewEntry};
use read::{fetch_capped, fetch_page, Page};
use std::path::PathBuf;

/// What to fetch, and where its image goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewJob {
    pub url: String,
    /// Whether the link may point at a loopback or private address (links the user wrote).
    pub allow_private: bool,
    pub media_dir: PathBuf,
}

/// Runs `job` to completion on a fresh single-thread runtime (call from a background thread)
/// and returns the cache entry to store, stamped `now` (Unix seconds).
pub fn fetch_preview_blocking(job: &PreviewJob, now: i64) -> PreviewEntry {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
    let outcome = match runtime {
        Ok(runtime) => runtime.block_on(fetch_preview(job)),
        Err(err) => Err(err.to_string()),
    };
    match outcome {
        Ok(Fetched { preview, image_path, icon_path }) => PreviewEntry::ready(job.url.as_str(), preview, image_path, now).with_icon(icon_path),
        Err(_) => PreviewEntry::failed(job.url.as_str(), now),
    }
}

/// A page's preview with the saved copies of its image and site icon.
struct Fetched {
    preview: LinkPreview,
    image_path: Option<String>,
    icon_path: Option<String>,
}

/// The page's preview, then its image, then its icon.
async fn fetch_preview(job: &PreviewJob) -> Result<Fetched, String> {
    let client = preview_client(job.allow_private);
    let video = recognise_video(&job.url);
    let page = fetch_page(&client, &job.url, job.allow_private).await;
    let mut preview = match (page, video.as_ref()) {
        (Ok(Page::Html(preview)), _) => preview,
        (Ok(Page::Image), _) => LinkPreview { image_url: Some(job.url.clone()), ..LinkPreview::default() },
        (Err(_), Some(video)) if video.has_page() => LinkPreview::default(),
        (Err(err), _) => return Err(err),
    };
    if let Some(video) = &video {
        preview.is_video = true;
        preview.image_url = preview.image_url.or_else(|| video.thumbnail_url());
    }
    let image_path = fetch_saved(&client, preview.image_url.as_deref(), job, MAX_IMAGE_BYTES, save_preview_image).await;
    let icon_path = fetch_saved(&client, preview.icon_url.as_deref(), job, MAX_ICON_BYTES, save_icon_image).await;
    Ok(Fetched { preview, image_path, icon_path })
}

/// The image at `url` saved in the media folder by `save`, if it downloads and decodes.
async fn fetch_saved(
    client: &reqwest::Client,
    url: Option<&str>,
    job: &PreviewJob,
    max_bytes: usize,
    save: fn(&[u8], &std::path::Path) -> Result<String, String>,
) -> Option<String> {
    let (bytes, _) = fetch_capped(client, url?, job.allow_private, max_bytes).await.ok()?;
    save(&bytes, &job.media_dir).ok()
}
