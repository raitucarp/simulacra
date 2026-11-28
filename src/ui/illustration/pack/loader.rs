//! How an illustration resolves: the chosen pack (the active one unless a preview names
//! another), then the key (the Classic asset path without root and extension), then the
//! pack's file; anything a pack does not cover, or cannot draw, falls back to the Classic
//! template themed with the active palette.

use super::layered::{compose, LayerPlan};
use super::raster::prepare;
use super::registry::{active_pack, find_pack, load_cached};
use super::source::RasterPack;
use crate::domain::packs::tint::{parse_hex, LayerRole, Tint};
use crate::domain::packs::{key_of_asset_path, CLASSIC_PACK_ID};
use crate::ui::assets::EmbeddedAssets;
use crate::ui::illustration::template::{HAIR_PLACEHOLDER, PRIMARY_PLACEHOLDER, SECONDARY_PLACEHOLDER, SKIN_PLACEHOLDER};
use crate::ui::illustration::{with_active_cache, TemplateExtras};
use gpui_kit::gpui::Image;
use std::sync::{Arc, OnceLock};

/// The image of the illustration at `path` for raster bucket `width`, from pack `pack`
/// (`None`: the active pack).
pub fn load_illustration(path: &str, extras: &TemplateExtras, width: u16, pack: Option<&str>) -> Arc<Image> {
    if let Some(image) = raster_illustration(path, extras, width, pack) {
        return image;
    }
    with_active_cache(|cache, colors| cache.load_sized(path, colors, extras, Some(width)))
}

/// The pack a lookup draws from.
fn pick(pack: Option<&str>) -> Option<Arc<RasterPack>> {
    match pack {
        Some(CLASSIC_PACK_ID) => None,
        Some(id) => find_pack(id),
        None => active_pack(),
    }
}

fn raster_illustration(path: &str, extras: &TemplateExtras, width: u16, pack: Option<&str>) -> Option<Arc<Image>> {
    let pack = pick(pack)?;
    let key = key_of_asset_path(path).filter(|key| pack.covers(key))?;
    let tint = layer_tint(key, extras);
    load_cached(&pack, key, extras, width, || {
        let entry = pack.manifest.file(key)?;
        prepare(pack.bytes(key)?, &entry.file, entry.width, width, &tint)
    })
}

/// A composed avatar (`source` names its look) from the active pack's raster layers, when the
/// pack covers every drawable layer; `None` keeps it Classic. `layers` is only planned on a
/// cache miss, and a look the pack cannot draw is remembered as such.
pub fn raster_avatar(source: &str, layers: impl FnOnce() -> Vec<LayerPlan>, extras: &TemplateExtras, width: u16) -> Option<Arc<Image>> {
    let pack = active_pack()?;
    load_cached(&pack, source, extras, width, || compose(&pack, &layers(), width))
}

/// The persona colours the layer `key` takes from `extras`.
pub fn layer_tint(key: &str, extras: &TemplateExtras) -> Tint {
    let color = |name: &str| extras.get(name).and_then(parse_hex);
    match LayerRole::of_key(key) {
        LayerRole::Skin => Tint { primary: color(SKIN_PLACEHOLDER), secondary: None },
        LayerRole::Hair => Tint { primary: color(HAIR_PLACEHOLDER), secondary: None },
        LayerRole::Body => Tint { primary: color(PRIMARY_PLACEHOLDER), secondary: color(SECONDARY_PLACEHOLDER) },
        LayerRole::Markings => Tint { primary: None, secondary: color(SECONDARY_PLACEHOLDER) },
        LayerRole::Fixed => Tint::default(),
    }
}

/// The plan of one layer of a composed avatar.
pub fn layer_plan(path: String, extras: &TemplateExtras, matrix: Option<[f32; 6]>, clip: bool) -> LayerPlan {
    let tint = key_of_asset_path(&path).map(|key| layer_tint(key, extras)).unwrap_or_default();
    LayerPlan { path, tint, matrix, clip }
}

/// Every Classic key (one per embedded template), sorted.
pub fn classic_keys() -> &'static [String] {
    static KEYS: OnceLock<Vec<String>> = OnceLock::new();
    KEYS.get_or_init(|| {
        let mut keys: Vec<String> = EmbeddedAssets::iter().filter_map(|path| key_of_asset_path(&path).map(str::to_string)).collect();
        keys.sort();
        keys
    })
}

/// How many Classic keys `pack` covers, and how many there are (`None`: Classic itself).
pub fn coverage(pack: Option<&RasterPack>) -> (usize, usize) {
    let keys = classic_keys();
    let covered = pack.map_or(keys.len(), |pack| pack.manifest.covered_of(keys.iter().map(String::as_str)));
    (covered, keys.len())
}
