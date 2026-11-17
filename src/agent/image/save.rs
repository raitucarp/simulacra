//! Turning an engine's answer (bytes, base64 or a URL) into an image file in the vault's
//! media folder.

use crate::agent::tools::http::read_capped;
use base64::Engine as _;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Largest image accepted (decoded or downloaded).
pub const MAX_IMAGE_BYTES: usize = 20 * 1024 * 1024;
/// Prefix of a data URL's payload separator.
const DATA_URL_SEPARATOR: &str = ";base64,";
/// File name prefix of generated images.
const IMAGE_FILE_PREFIX: &str = "img_";
/// Magic numbers → file extension.
const JPEG_MAGIC: &[u8] = &[0xFF, 0xD8, 0xFF];
const RIFF_MAGIC: &[u8] = b"RIFF";
const WEBP_MARK: &[u8] = b"WEBP";
const WEBP_MARK_OFFSET: usize = 8;

/// What an engine returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageData {
    Bytes(Vec<u8>),
    Base64(String),
    Url(String),
}

impl ImageData {
    /// A string answer: a URL when it starts with `http(s)://`, base64 (or a data URL) otherwise.
    pub fn from_text(text: &str) -> Self {
        let text = text.trim();
        if text.starts_with("http://") || text.starts_with("https://") {
            Self::Url(text.to_string())
        } else {
            Self::Base64(text.to_string())
        }
    }
}

/// Decodes base64, accepting a `data:image/...;base64,` prefix.
pub fn decode_base64(text: &str) -> Result<Vec<u8>, String> {
    let payload = text.split_once(DATA_URL_SEPARATOR).map_or(text, |(_, data)| data);
    let clean: String = payload.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(clean)
        .map_err(|e| format!("Image data is not valid base64: {e}"))?;
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err("Image is larger than the size limit".to_string());
    }
    Ok(bytes)
}

/// The image bytes, downloading URLs (size-capped).
pub async fn resolve(http: &reqwest::Client, data: ImageData) -> Result<Vec<u8>, String> {
    match data {
        ImageData::Bytes(bytes) => Ok(bytes),
        ImageData::Base64(text) => decode_base64(&text),
        ImageData::Url(url) => {
            let resp = http.get(&url).send().await.map_err(|e| format!("Image download failed: {e}"))?;
            if !resp.status().is_success() {
                return Err(format!("Image download returned {}", resp.status()));
            }
            let (bytes, cut) = read_capped(resp, MAX_IMAGE_BYTES).await?;
            if cut {
                return Err("Image is larger than the size limit".to_string());
            }
            Ok(bytes)
        }
    }
}

/// File extension for `bytes` (by magic number; PNG when unknown).
pub fn extension_for(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(JPEG_MAGIC) {
        "jpg"
    } else if bytes.starts_with(RIFF_MAGIC) && bytes.get(WEBP_MARK_OFFSET..WEBP_MARK_OFFSET + WEBP_MARK.len()) == Some(WEBP_MARK) {
        "webp"
    } else {
        "png"
    }
}

/// Writes `bytes` to a fresh file in `media_dir` (created when missing).
pub fn save_image(media_dir: &Path, bytes: &[u8]) -> Result<PathBuf, String> {
    if bytes.is_empty() {
        return Err("The image engine returned no image data".to_string());
    }
    std::fs::create_dir_all(media_dir).map_err(|e| format!("Cannot create {}: {e}", media_dir.display()))?;
    let name = format!("{IMAGE_FILE_PREFIX}{}.{}", Uuid::new_v4().simple(), extension_for(bytes));
    let path = media_dir.join(name);
    std::fs::write(&path, bytes).map_err(|e| format!("Cannot write {}: {e}", path.display()))?;
    Ok(path)
}
