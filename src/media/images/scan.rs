//! Scanning an image file for the feed: its pixel size (from the header) and, when it is
//! larger than the feed ever draws it, a cached downscaled copy. Animated GIFs keep their
//! original so they still play.

use super::decode;
use crate::domain::media::catalog::ImageScan;
use crate::domain::media::{ImageFacts, ImageSize};
use crate::import::parse::sha256_hex;
use image::ImageFormat;
use std::path::Path;
use std::time::UNIX_EPOCH;

/// Longest edge of a feed thumbnail (wide enough for a full-width card on a 2x display).
pub const THUMB_MAX_EDGE: u32 = 1280;
const THUMBS_DIR: &str = "thumbs";
const KEY_HASH_CHARS: usize = 24;
/// Largest image file scanned.
const MAX_SCAN_BYTES: u64 = 64 * 1024 * 1024;

/// What the feed should know about the image at `source`.
pub fn scan_image(source: &str, media_dir: &Path) -> ImageScan {
    match scan(source, media_dir) {
        Ok(facts) => ImageScan::Known(facts),
        Err(_) => ImageScan::Unreadable,
    }
}

fn scan(source: &str, media_dir: &Path) -> Result<ImageFacts, String> {
    let meta = std::fs::metadata(source).map_err(|e| e.to_string())?;
    if !meta.is_file() || meta.len() > MAX_SCAN_BYTES {
        return Err("not a scannable file".into());
    }
    let bytes = std::fs::read(source).map_err(|e| e.to_string())?;
    let format = decode::accepted_format(&bytes)?;
    let (width, height) = decode::dimensions(&bytes)?;
    let size = ImageSize::new(width, height);
    if format == ImageFormat::Gif || width.max(height) <= THUMB_MAX_EDGE {
        return Ok(ImageFacts { size, thumb_path: None });
    }
    let key = thumb_key(source, meta.len(), meta.modified().ok());
    let existing = ["jpg", "png"].map(|ext| media_dir.join(THUMBS_DIR).join(format!("{key}.{ext}"))).into_iter().find(|path| path.exists());
    let thumb = match existing {
        Some(path) => path,
        None => {
            let image = decode::fit(decode::decode(&bytes)?, THUMB_MAX_EDGE);
            let path = media_dir.join(THUMBS_DIR).join(format!("{key}.{}", decode::encoded_extension(&image)));
            decode::write(&image, &path)?;
            path
        }
    };
    Ok(ImageFacts { size, thumb_path: Some(thumb.display().to_string()) })
}

/// A thumbnail name that changes whenever the source file is replaced.
fn thumb_key(source: &str, length: u64, modified: Option<std::time::SystemTime>) -> String {
    let stamp = modified.and_then(|time| time.duration_since(UNIX_EPOCH).ok()).map_or(0, |since| since.as_nanos());
    sha256_hex(format!("{source}|{length}|{stamp}").as_bytes()).chars().take(KEY_HASH_CHARS).collect()
}
