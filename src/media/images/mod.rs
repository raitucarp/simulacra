//! Images in the media folder: attachments stored once by content hash, downloaded preview
//! images scaled down, and scans that learn an image's size and cache a feed thumbnail.
//!
//! - `attachments/<hash>.<ext>`: images attached in the composer (bytes as given).
//! - `previews/<hash>.<jpg|png>`: link preview images, at most [`PREVIEW_MAX_EDGE`] px.
//! - `thumbs/<key>.<jpg|png>`: feed thumbnails of large images, at most [`THUMB_MAX_EDGE`] px.

mod decode;
mod scan;

pub use scan::{scan_image, THUMB_MAX_EDGE};

use crate::domain::media::{ImageAttachment, ImageSize};
use crate::import::parse::sha256_hex;
use std::path::Path;

const ATTACHMENTS_DIR: &str = "attachments";
const PREVIEWS_DIR: &str = "previews";
/// Longest edge of a saved preview image and of a saved site icon.
pub const PREVIEW_MAX_EDGE: u32 = 720;
const ICON_MAX_EDGE: u32 = 64;
/// Hex characters of the content hash used as a file name.
const NAME_HASH_CHARS: usize = 24;
/// Largest image file attached (bigger files are refused rather than copied).
pub const MAX_ATTACHMENT_BYTES: u64 = 40 * 1024 * 1024;

/// The first hex characters of the SHA-256 of `bytes`.
fn content_name(bytes: &[u8]) -> String {
    sha256_hex(bytes).chars().take(NAME_HASH_CHARS).collect()
}

/// Copies the image at `source` into the media folder (once per content) and returns it as
/// an attachment with its pixel size.
pub fn store_attachment_file(source: &Path, media_dir: &Path) -> Result<ImageAttachment, String> {
    let size = std::fs::metadata(source).map_err(|e| e.to_string())?.len();
    if size > MAX_ATTACHMENT_BYTES {
        return Err(format!("image is larger than {MAX_ATTACHMENT_BYTES} bytes"));
    }
    let bytes = std::fs::read(source).map_err(|e| e.to_string())?;
    store_attachment_bytes(&bytes, media_dir)
}

/// Stores pasted or dropped image `bytes` in the media folder (once per content).
pub fn store_attachment_bytes(bytes: &[u8], media_dir: &Path) -> Result<ImageAttachment, String> {
    let format = decode::accepted_format(bytes)?;
    let (width, height) = decode::dimensions(bytes)?;
    let extension = format.extensions_str().first().copied().unwrap_or_default();
    let path = media_dir.join(ATTACHMENTS_DIR).join(format!("{}.{extension}", content_name(bytes)));
    if !path.exists() {
        std::fs::create_dir_all(media_dir.join(ATTACHMENTS_DIR)).map_err(|e| e.to_string())?;
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
    }
    let size = Some(ImageSize::new(width, height));
    Ok(ImageAttachment { path: path.display().to_string(), alt: String::new(), size })
}

/// Decodes downloaded preview `bytes`, scales them down and saves them; returns the path.
pub fn save_preview_image(bytes: &[u8], media_dir: &Path) -> Result<String, String> {
    decode::accepted_format(bytes)?;
    save_scaled(bytes, media_dir, PREVIEW_MAX_EDGE)
}

/// Decodes a downloaded site icon (icon files included), scales it down and saves it.
pub fn save_icon_image(bytes: &[u8], media_dir: &Path) -> Result<String, String> {
    decode::icon_format(bytes)?;
    save_scaled(bytes, media_dir, ICON_MAX_EDGE)
}

/// Saves `bytes` decoded and fitted to `max_edge` under `previews/`, once per content.
fn save_scaled(bytes: &[u8], media_dir: &Path, max_edge: u32) -> Result<String, String> {
    let image = decode::fit(decode::decode(bytes)?, max_edge);
    let name = format!("{}.{}", content_name(bytes), decode::encoded_extension(&image));
    let path = media_dir.join(PREVIEWS_DIR).join(name);
    if !path.exists() {
        decode::write(&image, &path)?;
    }
    Ok(path.display().to_string())
}
