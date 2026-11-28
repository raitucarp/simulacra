//! The UI thread's illustration packs: every available raster pack, the active one, and the
//! images drawn from them. Raster images do not depend on the palette, so they live in their
//! own cache that a theme change leaves alone; switching, installing or removing a pack
//! clears it and hands the old images back for `remove_asset`.

use super::source::RasterPack;
use crate::domain::packs::CLASSIC_PACK_ID;
use crate::packs::PackStore;
use crate::ui::illustration::TemplateExtras;
use gpui_kit::gpui::Image;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// A cached raster: pack, key or composed-avatar source, tint extras and size bucket.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RasterKey {
    pack: String,
    source: String,
    extras: TemplateExtras,
    width: u16,
}

#[derive(Default)]
struct Registry {
    packs: Vec<Arc<RasterPack>>,
    /// The pack the user chose (may be unknown: then Classic draws).
    chosen: String,
    active: Option<Arc<RasterPack>>,
    /// Built images; `None` remembers a key that could not be drawn (Classic draws it).
    cache: HashMap<RasterKey, Option<Arc<Image>>>,
}

impl Registry {
    fn resolve(&self, id: &str) -> Option<Arc<RasterPack>> {
        self.packs.iter().find(|pack| pack.id() == id).cloned()
    }

    fn evict(&mut self) -> Vec<Arc<Image>> {
        self.cache.drain().filter_map(|(_, image)| image).collect()
    }
}

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

/// Makes `packs` the available raster packs (the active choice is resolved again) and
/// returns the evicted images.
pub fn set_available_packs(packs: Vec<Arc<RasterPack>>) -> Vec<Arc<Image>> {
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        registry.packs = packs;
        registry.active = registry.resolve(&registry.chosen.clone());
        registry.evict()
    })
}

/// Draws illustrations from pack `id` (`classic` or an unknown id: Classic everywhere).
/// When that changes the pack in use, returns the evicted images.
pub fn set_active_pack(id: &str) -> Vec<Arc<Image>> {
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        registry.chosen = id.to_string();
        let next = registry.resolve(id);
        let same = match (&next, &registry.active) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        registry.active = next;
        if same { Vec::new() } else { registry.evict() }
    })
}

/// Loads the built-in Flat pack and every pack installed in `packs_dir`, then activates
/// `chosen`; returns the evicted images.
pub fn configure_packs(packs_dir: &Path, chosen: &str) -> Vec<Arc<Image>> {
    let installed = PackStore::new(packs_dir).installed().into_iter().map(RasterPack::installed);
    let packs = RasterPack::flat().into_iter().chain(installed).map(Arc::new).collect();
    let mut stale = set_available_packs(packs);
    stale.extend(set_active_pack(chosen));
    stale
}

/// The raster pack in use (`None` while Classic draws everything).
pub fn active_pack() -> Option<Arc<RasterPack>> {
    REGISTRY.with(|registry| registry.borrow().active.clone())
}

/// Id of the pack in use (`classic` when the choice is unavailable).
pub fn active_pack_id() -> String {
    active_pack().map(|pack| pack.id().to_string()).unwrap_or_else(|| CLASSIC_PACK_ID.to_string())
}

/// The available raster pack `id`.
pub fn find_pack(id: &str) -> Option<Arc<RasterPack>> {
    REGISTRY.with(|registry| registry.borrow().resolve(id))
}

/// Every available raster pack (built-in first, then installed by name).
pub fn available_packs() -> Vec<Arc<RasterPack>> {
    REGISTRY.with(|registry| registry.borrow().packs.clone())
}

/// The cached raster of `source` from `pack`, building it with `build` on a miss (a failed
/// build is remembered, so Classic draws it without retrying every frame).
pub fn load_cached(pack: &RasterPack, source: &str, extras: &TemplateExtras, width: u16, build: impl FnOnce() -> Option<Image>) -> Option<Arc<Image>> {
    let key = RasterKey { pack: pack.id().to_string(), source: source.to_string(), extras: extras.clone(), width };
    if let Some(hit) = REGISTRY.with(|registry| registry.borrow().cache.get(&key).cloned()) {
        return hit;
    }
    let image = build().map(Arc::new);
    REGISTRY.with(|registry| registry.borrow_mut().cache.insert(key, image.clone()));
    image
}

/// Number of cached raster entries (drawn or failed).
pub fn raster_cache_len() -> usize {
    REGISTRY.with(|registry| registry.borrow().cache.len())
}
