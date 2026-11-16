//! Circular, zoomed and panned photo avatars.
//!
//! gpui content masks are rectangular, so "rounded container + positioned img" crops a zoomed
//! photo to a square. Instead the crop is a tiny SVG that references the photo file and clips
//! it to a circle; gpui rasterizes it off the UI thread like any SVG `img`.

use super::active::{forget_illustrations, with_active_cache};
use super::template::TemplateExtras;
use crate::domain::portrait::{PhotoCrop, PER_MILLE};
use crate::storage::avatars::{resolve_photo_path, vault_dir};
use crate::ui::atoms::forget_avatar;
use gpui_kit::gpui::Image;
use std::path::Path;
use std::sync::Arc;
use std::time::UNIX_EPOCH;

/// Cache source prefix of photo avatars (`photo/<abs path>|zoom|x|y`).
pub const PHOTO_SOURCE_PREFIX: &str = "photo/";
/// Photo formats the SVG renderer decodes; anything else falls back to a plain `img`.
pub const CROPPABLE_EXTENSIONS: [&str; 5] = ["png", "jpg", "jpeg", "gif", "webp"];
/// Separator of crop fields in a cache source.
const SOURCE_SEPARATOR: char = '|';
/// Id of the circular clip path.
const CLIP_ID: &str = "avatar-photo-clip";

/// True when the file at `path` can be cropped through the SVG renderer.
pub fn is_croppable(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| CROPPABLE_EXTENSIONS.iter().any(|known| known.eq_ignore_ascii_case(ext)))
}

/// `text` escaped for an XML attribute value.
fn xml_attr(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Modification stamp of `path`, embedded so a replaced file yields new image bytes
/// (gpui caches images by content hash).
fn file_version(path: &Path) -> u128 {
    std::fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |age| age.as_nanos())
}

/// SVG drawing the photo at `path` inside a circle, framed by `crop` (per-mille canvas).
pub fn photo_template(path: &Path, crop: PhotoCrop, version: u128) -> String {
    let side = PER_MILLE as f32;
    let half = side / 2.0;
    let frame = crop.frame(side);
    let href = xml_attr(&path.to_string_lossy());
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 {side} {side}"><!--{version}--><defs><clipPath id="{CLIP_ID}"><circle cx="{half}" cy="{half}" r="{half}"/></clipPath></defs><image x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" preserveAspectRatio="xMidYMid slice" clip-path="url(#{CLIP_ID})" xlink:href="{href}"/></svg>"#,
        frame.x, frame.y, frame.side, frame.side,
    )
}

/// Cache source of one photo framing.
pub fn photo_source(path: &Path, crop: PhotoCrop) -> String {
    let s = SOURCE_SEPARATOR;
    format!("{PHOTO_SOURCE_PREFIX}{}{s}{}{s}{}{s}{}", path.display(), crop.zoom, crop.offset_x, crop.offset_y)
}

/// The cropped photo image rasterized `width` units wide.
pub fn photo_image(path: &Path, crop: PhotoCrop, width: Option<u16>) -> Arc<Image> {
    let source = photo_source(path, crop);
    let build = || Some(photo_template(path, crop, file_version(path)));
    with_active_cache(|cache, colors| cache.load_with(&source, build, colors, &TemplateExtras::none(), width))
}

/// Forgets every cached framing (and the cached file lookup) of the photo at `path`; call
/// after importing or replacing the file. Returns the images for `remove_asset`.
pub fn forget_photo(path: &Path) -> Vec<Arc<Image>> {
    forget_avatar(&path.to_string_lossy());
    forget_illustrations(&format!("{PHOTO_SOURCE_PREFIX}{}{SOURCE_SEPARATOR}", path.display()))
}

/// [`forget_photo`] for a stored (vault-relative or absolute) avatar path.
pub fn forget_stored_photo(stored: &str) -> Vec<Arc<Image>> {
    forget_photo(&resolve_photo_path(vault_dir().as_deref(), stored))
}
