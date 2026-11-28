//! Pack files as gpui images. A file at most twice the display bucket is handed to gpui as is
//! (gpui decodes it off the main thread); a larger one is decoded, downscaled with
//! premultiplied alpha (no dark fringes) and re-encoded once per size bucket, so small
//! displays stay crisp instead of aliasing. Layer tints are applied on the way.

use crate::domain::packs::tint::{tint_rgba, Tint};
use gpui_kit::gpui::{Image, ImageFormat};
use image::codecs::png::{CompressionType, FilterType as PngFilter, PngEncoder};
use image::imageops::{self, FilterType};
use image::{ExtendedColorType, ImageEncoder, RgbaImage};

/// Device pixels prepared per logical pixel (matches the SVG path's 2x rasterization).
const DENSITY: u32 = 2;
const PNG_EXT: &str = ".png";
const OPAQUE: f32 = 255.0;

/// Pixel width to prepare a `stored`-wide file at for the raster bucket `bucket`.
pub fn target_width(stored: u32, bucket: u16) -> u32 {
    stored.min(u32::from(bucket) * DENSITY).max(1)
}

/// The image format of pack file `file`.
pub fn file_format(file: &str) -> ImageFormat {
    if file.ends_with(PNG_EXT) { ImageFormat::Png } else { ImageFormat::Webp }
}

/// RGBA pixels of a pack file.
pub fn decode(bytes: &[u8], format: ImageFormat) -> Option<RgbaImage> {
    let format = if format == ImageFormat::Png { image::ImageFormat::Png } else { image::ImageFormat::WebP };
    image::load_from_memory_with_format(bytes, format).ok().map(|image| image.to_rgba8())
}

/// `image` scaled to `width` × `height` with premultiplied alpha.
pub fn resized_to(mut image: RgbaImage, width: u32, height: u32) -> RgbaImage {
    if image.dimensions() == (width, height) {
        return image;
    }
    premultiply(&mut image, true);
    let mut out = imageops::resize(&image, width.max(1), height.max(1), FilterType::CatmullRom);
    premultiply(&mut out, false);
    out
}

/// `image` scaled down to at most `width` wide, keeping its aspect.
pub fn resized(image: RgbaImage, width: u32) -> RgbaImage {
    if image.width() <= width {
        return image;
    }
    let height = (image.height() as f32 * width as f32 / image.width() as f32).round().max(1.0) as u32;
    resized_to(image, width, height)
}

/// Multiplies (or, with `forward` false, divides) colour by alpha.
fn premultiply(image: &mut RgbaImage, forward: bool) {
    for pixel in image.pixels_mut() {
        let alpha = f32::from(pixel[3]) / OPAQUE;
        for channel in 0..3 {
            let value = f32::from(pixel[channel]);
            let next = if forward { value * alpha } else if alpha > 0.0 { value / alpha } else { 0.0 };
            pixel[channel] = next.round().clamp(0.0, OPAQUE) as u8;
        }
    }
}

/// `image` as a quickly compressed PNG.
pub fn encode_png(image: &RgbaImage) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    let encoder = PngEncoder::new_with_quality(&mut bytes, CompressionType::Fast, PngFilter::Adaptive);
    encoder.write_image(image.as_raw(), image.width(), image.height(), ExtendedColorType::Rgba8).ok()?;
    Some(bytes)
}

/// The gpui image of one pack file prepared for `bucket`, tinted by `tint`.
pub fn prepare(bytes: Vec<u8>, file: &str, stored_width: u32, bucket: u16, tint: &Tint) -> Option<Image> {
    let format = file_format(file);
    let width = target_width(stored_width, bucket);
    if width >= stored_width && tint.is_identity() {
        return Some(Image::from_bytes(format, bytes));
    }
    let mut image = resized(decode(&bytes, format)?, width);
    tint_rgba(&mut image, tint);
    encode_png(&image).map(|png| Image::from_bytes(ImageFormat::Png, png))
}
