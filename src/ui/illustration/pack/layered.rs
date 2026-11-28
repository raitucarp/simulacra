//! Composed avatars from raster layers. Portraits and forms stack 256-unit layers; a raster
//! pack draws them only when it covers every drawable layer of the look (a layer whose
//! Classic template is blank, like `accessory/none`, is skipped), otherwise the whole avatar
//! stays Classic so vector and raster parts never mix. Each layer is scaled to the shared
//! canvas, tinted with the persona's colours, moved by its face anchor and clipped to the
//! body's silhouette like its SVG counterpart.

use super::raster::{decode, encode_png, file_format, resized_to, target_width};
use super::source::RasterPack;
use crate::domain::packs::key_of_asset_path;
use crate::domain::packs::tint::{tint_rgba, LayerRole, Tint};
use crate::ui::assets::template_text;
use crate::ui::illustration::portrait::layer_body;
use gpui_kit::gpui::{Image, ImageFormat};
use image::imageops::{self, FilterType};
use image::RgbaImage;

/// SVG units of the layer canvas.
const LAYER_CANVAS: f32 = 256.0;

/// One layer of a composed avatar: its Classic asset path, tint, face-anchor transform
/// (`matrix(a b c d e f)` in canvas units, uniform scale) and whether it is clipped to the body.
#[derive(Debug, Clone, PartialEq)]
pub struct LayerPlan {
    pub path: String,
    pub tint: Tint,
    pub matrix: Option<[f32; 6]>,
    pub clip: bool,
}

/// Whether the Classic template at `path` draws nothing (an intentionally empty variant).
pub fn is_blank_layer(path: &str) -> bool {
    template_text(path).is_some_and(|text| layer_body(&text).is_some_and(|body| body.trim().is_empty()))
}

/// Whether `pack` covers every drawable layer of `layers` (and at least one).
pub fn covers_all(pack: &RasterPack, layers: &[LayerPlan]) -> bool {
    let covered = |plan: &LayerPlan| key_of_asset_path(&plan.path).is_some_and(|key| pack.covers(key));
    layers.iter().any(covered) && layers.iter().all(|plan| covered(plan) || is_blank_layer(&plan.path))
}

/// The composed avatar prepared for `bucket`, or `None` when a drawable layer is missing.
pub fn compose(pack: &RasterPack, layers: &[LayerPlan], bucket: u16) -> Option<Image> {
    if !covers_all(pack, layers) {
        return None;
    }
    let stored = layers.iter().filter_map(|plan| pack.manifest.file(key_of_asset_path(&plan.path)?)).map(|file| file.width).max()?;
    let size = target_width(stored, bucket);
    let mut canvas = RgbaImage::new(size, size);
    let mut body_alpha: Option<RgbaImage> = None;
    for plan in layers {
        let Some(key) = key_of_asset_path(&plan.path).filter(|key| pack.covers(key)) else { continue };
        let entry = pack.manifest.file(key)?;
        let mut layer = resized_to(decode(&pack.bytes(key)?, file_format(&entry.file))?, size, size);
        tint_rgba(&mut layer, &plan.tint);
        if let Some(matrix) = plan.matrix {
            layer = transformed(&layer, matrix, size);
        }
        if let (true, Some(mask)) = (plan.clip, &body_alpha) {
            clip(&mut layer, mask);
        }
        if LayerRole::of_key(key) == LayerRole::Body {
            body_alpha = Some(layer.clone());
        }
        imageops::overlay(&mut canvas, &layer, 0, 0);
    }
    encode_png(&canvas).map(|png| Image::from_bytes(ImageFormat::Png, png))
}

/// `layer` scaled by the matrix's `a` and moved by its `e`, `f` (canvas units).
fn transformed(layer: &RgbaImage, [scale, _, _, _, dx, dy]: [f32; 6], size: u32) -> RgbaImage {
    let units = size as f32 / LAYER_CANVAS;
    let scaled_side = (size as f32 * scale).round().max(1.0) as u32;
    let scaled = imageops::resize(layer, scaled_side, scaled_side, FilterType::CatmullRom);
    let mut out = RgbaImage::new(size, size);
    imageops::overlay(&mut out, &scaled, (dx * units).round() as i64, (dy * units).round() as i64);
    out
}

/// Multiplies `layer`'s alpha by `mask`'s (the body silhouette).
fn clip(layer: &mut RgbaImage, mask: &RgbaImage) {
    for (pixel, mask) in layer.pixels_mut().zip(mask.pixels()) {
        pixel[3] = ((u16::from(pixel[3]) * u16::from(mask[3])) / 255) as u8;
    }
}
