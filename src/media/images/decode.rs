//! Bounded decoding, downscaling and encoding of raster images.

use image::{DynamicImage, ImageFormat, ImageReader, Limits};
use std::io::Cursor;
use std::path::Path;

/// Largest width or height decoded, and the most memory one decode may take.
const MAX_DIMENSION: u32 = 16_384;
const MAX_ALLOC_BYTES: u64 = 512 * 1024 * 1024;
/// JPEG quality of thumbnails and preview images.
const JPEG_QUALITY: u8 = 85;
/// Raster formats accepted as attachments (vector and exotic formats are not).
const ACCEPTED: [ImageFormat; 5] = [ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::Gif, ImageFormat::WebP, ImageFormat::Bmp];

fn limits() -> Limits {
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_DIMENSION);
    limits.max_image_height = Some(MAX_DIMENSION);
    limits.max_alloc = Some(MAX_ALLOC_BYTES);
    limits
}

/// The accepted raster format of `bytes`.
pub(super) fn accepted_format(bytes: &[u8]) -> Result<ImageFormat, String> {
    let format = image::guess_format(bytes).map_err(|e| e.to_string())?;
    ACCEPTED.contains(&format).then_some(format).ok_or_else(|| format!("unsupported image format {format:?}"))
}

/// The format of a site icon (icon files as well as the accepted raster formats).
pub(super) fn icon_format(bytes: &[u8]) -> Result<ImageFormat, String> {
    match image::guess_format(bytes).map_err(|e| e.to_string())? {
        ImageFormat::Ico => Ok(ImageFormat::Ico),
        _ => accepted_format(bytes),
    }
}

/// Width and height from the header of `bytes`, without decoding the pixels.
pub(super) fn dimensions(bytes: &[u8]) -> Result<(u32, u32), String> {
    let mut reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format().map_err(|e| e.to_string())?;
    reader.limits(limits());
    reader.into_dimensions().map_err(|e| e.to_string())
}

/// Every pixel of `bytes`, within the decode limits.
pub(super) fn decode(bytes: &[u8]) -> Result<DynamicImage, String> {
    let mut reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format().map_err(|e| e.to_string())?;
    reader.limits(limits());
    reader.decode().map_err(|e| e.to_string())
}

/// `image` shrunk to fit `max_edge` (never enlarged).
pub(super) fn fit(image: DynamicImage, max_edge: u32) -> DynamicImage {
    if image.width().max(image.height()) <= max_edge {
        image
    } else {
        image.thumbnail(max_edge, max_edge)
    }
}

/// The file extension `image` is written with: PNG keeps transparency, JPEG otherwise.
pub(super) fn encoded_extension(image: &DynamicImage) -> &'static str {
    if image.color().has_alpha() {
        "png"
    } else {
        "jpg"
    }
}

/// Writes `image` to `path` (PNG with transparency, else JPEG), via a temporary file so a
/// half-written thumbnail is never picked up.
pub(super) fn write(image: &DynamicImage, path: &Path) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let partial = path.with_extension("part");
    let file = std::fs::File::create(&partial).map_err(|e| e.to_string())?;
    let mut out = std::io::BufWriter::new(file);
    let written = if image.color().has_alpha() {
        image.write_to(&mut out, ImageFormat::Png).map_err(|e| e.to_string())
    } else {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, JPEG_QUALITY);
        image.to_rgb8().write_with_encoder(encoder).map_err(|e| e.to_string())
    };
    written?;
    drop(out);
    std::fs::rename(&partial, path).map_err(|e| e.to_string())
}
