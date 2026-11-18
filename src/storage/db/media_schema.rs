//! The media cache tables (sb C3), created idempotently on every open:
//!
//! - `link_previews`: one fetched (or failed) link preview per URL, with its expiry. The
//!   preview itself is JSON (`NULL` for a failure); its image lives in the media folder.
//! - `media_images`: what a scan learned about an image file (size, feed thumbnail).

pub(super) const CREATE_MEDIA_TABLES: &str = r#"
                CREATE TABLE IF NOT EXISTS link_previews (
                    url TEXT PRIMARY KEY,
                    preview_json TEXT,
                    image_path TEXT,
                    icon_path TEXT,
                    fetched_at INTEGER NOT NULL,
                    expires_at INTEGER NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_link_previews_expires ON link_previews(expires_at);

                CREATE TABLE IF NOT EXISTS media_images (
                    source TEXT PRIMARY KEY,
                    readable INTEGER NOT NULL,
                    width INTEGER NOT NULL DEFAULT 0,
                    height INTEGER NOT NULL DEFAULT 0,
                    thumb_path TEXT,
                    scanned_at INTEGER NOT NULL
                );
                "#;
