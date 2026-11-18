//! Media cache rows (sb C3): link previews and image scans, loaded once at startup and
//! upserted as background work finishes.

use super::{collect_rows, db_err, sql_bool, Database};
use crate::domain::media::catalog::ImageScan;
use crate::domain::media::{ImageFacts, ImageSize, LinkPreview, PreviewEntry};
use crate::error::Result;
use rusqlite::{params, Row};

const UPSERT_PREVIEW: &str = r#"
    INSERT INTO link_previews (url, preview_json, image_path, icon_path, fetched_at, expires_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
    ON CONFLICT(url) DO UPDATE SET preview_json = excluded.preview_json, image_path = excluded.image_path,
        icon_path = excluded.icon_path, fetched_at = excluded.fetched_at, expires_at = excluded.expires_at;
"#;
const SELECT_PREVIEWS: &str = "SELECT url, preview_json, image_path, icon_path, fetched_at, expires_at FROM link_previews";
const UPSERT_IMAGE: &str = r#"
    INSERT INTO media_images (source, readable, width, height, thumb_path, scanned_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
    ON CONFLICT(source) DO UPDATE SET readable = excluded.readable, width = excluded.width, height = excluded.height,
        thumb_path = excluded.thumb_path, scanned_at = excluded.scanned_at;
"#;
const SELECT_IMAGES: &str = "SELECT source, readable, width, height, thumb_path FROM media_images";
const CLEAR_MEDIA: &str = "DELETE FROM link_previews; DELETE FROM media_images;";

impl Database {
    /// Every cached link preview (an unreadable preview row counts as a failure).
    pub fn load_link_previews(&self) -> Result<Vec<PreviewEntry>> {
        let mut stmt = self.conn.prepare(SELECT_PREVIEWS).map_err(db_err)?;
        let rows = stmt.query_map([], preview_from_row).map_err(db_err)?;
        collect_rows(rows)
    }

    pub fn save_link_preview(&self, entry: &PreviewEntry) -> Result<()> {
        let json = entry.preview.as_ref().and_then(|preview| serde_json::to_string(preview).ok());
        let values = params![entry.url, json, entry.image_path, entry.icon_path, entry.fetched_at, entry.expires_at];
        self.conn.execute(UPSERT_PREVIEW, values).map_err(db_err).map(|_| ())
    }

    /// Every stored image scan, by source path.
    pub fn load_media_images(&self) -> Result<Vec<(String, ImageScan)>> {
        let mut stmt = self.conn.prepare(SELECT_IMAGES).map_err(db_err)?;
        let rows = stmt.query_map([], image_from_row).map_err(db_err)?;
        collect_rows(rows)
    }

    pub fn save_media_image(&self, source: &str, scan: &ImageScan, scanned_at: i64) -> Result<()> {
        let (readable, width, height, thumb) = match scan {
            ImageScan::Known(facts) => (true, facts.size.width, facts.size.height, facts.thumb_path.clone()),
            ImageScan::Unreadable => (false, 0, 0, None),
        };
        let values = params![source, sql_bool(readable), width, height, thumb, scanned_at];
        self.conn.execute(UPSERT_IMAGE, values).map_err(db_err).map(|_| ())
    }

    /// Forgets every cached preview and scan (Settings › Clear caches).
    pub fn clear_media_cache(&self) -> Result<()> {
        self.conn.execute_batch(CLEAR_MEDIA).map_err(db_err)
    }
}

fn preview_from_row(row: &Row<'_>) -> rusqlite::Result<PreviewEntry> {
    let json: Option<String> = row.get(1)?;
    Ok(PreviewEntry {
        url: row.get(0)?,
        preview: json.and_then(|json| serde_json::from_str::<LinkPreview>(&json).ok()),
        image_path: row.get(2)?,
        icon_path: row.get(3)?,
        fetched_at: row.get(4)?,
        expires_at: row.get(5)?,
    })
}

fn image_from_row(row: &Row<'_>) -> rusqlite::Result<(String, ImageScan)> {
    let readable: i64 = row.get(1)?;
    let scan = match readable {
        0 => ImageScan::Unreadable,
        _ => ImageScan::Known(ImageFacts { size: ImageSize::new(row.get(2)?, row.get(3)?), thumb_path: row.get(4)? }),
    };
    Ok((row.get(0)?, scan))
}
