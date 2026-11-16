//! [`IllustrationCache`]: themed, rasterizable SVG images keyed by
//! (template source, palette, extras, raster width).

use super::template::{fill_template, with_raster_width, TemplateExtras};
use crate::theme::{PaletteColorKey, PaletteColors};
use crate::ui::assets::template_text;
use gpui_kit::gpui::{Image, ImageFormat};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Neutral stand-in drawn for a missing or unreadable template (themed like the real ones).
pub const FALLBACK_TEMPLATE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 120" fill="none"><rect x="8" y="8" width="144" height="104" rx="16" fill="{{elevated}}"/><circle cx="80" cy="60" r="22" fill="{{fg_muted}}" fill-opacity=".25"/></svg>"#;

/// Stable hash of a palette's fourteen colors.
pub fn palette_hash(colors: &PaletteColors) -> u64 {
    let mut hasher = DefaultHasher::new();
    for key in PaletteColorKey::ALL {
        let color = key.get(colors);
        [color.r, color.g, color.b, color.a].map(f32::to_bits).hash(&mut hasher);
    }
    hasher.finish()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    source: String,
    palette: u64,
    extras: TemplateExtras,
    width: Option<u16>,
}

/// Filled-and-encoded illustration images, so each (template, theme, look, size class)
/// is built once and shared by every frame.
#[derive(Debug, Default)]
pub struct IllustrationCache {
    entries: HashMap<CacheKey, Arc<Image>>,
    hits: usize,
    misses: usize,
}

impl IllustrationCache {
    /// The asset template at `path` themed with `colors` and `extras`, at its canvas size.
    pub fn load(&mut self, path: &str, colors: &PaletteColors, extras: &TemplateExtras) -> Arc<Image> {
        self.load_sized(path, colors, extras, None)
    }

    /// [`Self::load`] with an explicit intrinsic raster `width` (see `template::raster_width`).
    /// Unknown paths render [`FALLBACK_TEMPLATE`]; this never panics.
    pub fn load_sized(&mut self, path: &str, colors: &PaletteColors, extras: &TemplateExtras, width: Option<u16>) -> Arc<Image> {
        self.load_with(path, || template_text(path).map(|t| t.into_owned()), colors, extras, width)
    }

    /// A generated template (emblem, composed portrait, photo frame) cached under `source`;
    /// `build` only runs on a miss and may return `None` to draw the fallback.
    pub fn load_with(
        &mut self,
        source: &str,
        build: impl FnOnce() -> Option<String>,
        colors: &PaletteColors,
        extras: &TemplateExtras,
        width: Option<u16>,
    ) -> Arc<Image> {
        let key = CacheKey { source: source.to_string(), palette: palette_hash(colors), extras: extras.clone(), width };
        if let Some(hit) = self.entries.get(&key) {
            self.hits += 1;
            return hit.clone();
        }
        self.misses += 1;
        let template = build().unwrap_or_else(|| FALLBACK_TEMPLATE.to_string());
        let svg = with_raster_width(&fill_template(&template, colors, extras), width);
        let image = Arc::new(Image::from_bytes(ImageFormat::Svg, svg.into_bytes()));
        self.entries.insert(key, image.clone());
        image
    }

    /// Drops every entry (after a theme change); returns them so the caller can release
    /// gpui's rasterized copies (`Image::remove_asset`).
    pub fn invalidate(&mut self) -> Vec<Arc<Image>> {
        self.entries.drain().map(|(_, image)| image).collect()
    }

    /// Drops the entries whose source starts with `prefix` (e.g. one re-imported photo).
    pub fn forget_source(&mut self, prefix: &str) -> Vec<Arc<Image>> {
        let stale: Vec<CacheKey> = self.entries.keys().filter(|k| k.source.starts_with(prefix)).cloned().collect();
        stale.iter().filter_map(|key| self.entries.remove(key)).collect()
    }

    /// Number of cached images.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True when nothing is cached.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Lookups served from the cache so far.
    pub fn hits(&self) -> usize {
        self.hits
    }

    /// Lookups that had to build an image so far.
    pub fn misses(&self) -> usize {
        self.misses
    }
}
