//! Vault operations of the media cache (sb C3): one startup read, queued upserts.

use super::Vault;
use crate::domain::media::catalog::ImageScan;
use crate::domain::media::{MediaCatalog, PreviewEntry};
use crate::error::Result;

impl Vault {
    /// Reads every cached preview and image scan into a catalog.
    pub fn load_media_catalog(&self) -> Result<MediaCatalog> {
        self.read(|db| Ok(MediaCatalog::from_rows(db.load_link_previews()?, db.load_media_images()?)))
    }

    /// Queues storing a finished preview fetch.
    pub fn save_link_preview(&self, entry: PreviewEntry) -> Result<()> {
        self.write(move |db| db.save_link_preview(&entry))
    }

    /// Queues storing a finished image scan.
    pub fn save_media_image(&self, source: String, scan: ImageScan, scanned_at: i64) -> Result<()> {
        self.write(move |db| db.save_media_image(&source, &scan, scanned_at))
    }

    /// Queues forgetting every cached preview and scan.
    pub fn clear_media_cache(&self) -> Result<()> {
        self.write(|db| db.clear_media_cache())
    }
}
