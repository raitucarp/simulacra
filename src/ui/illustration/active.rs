//! The UI thread's illustration cache and the palette it is themed with.
//!
//! Rendering helpers (avatars, `Illustration`) take no `cx`, so the cache lives in a
//! thread-local next to the active palette. The shell calls [`set_active_palette`] at startup
//! and after every theme change; a different palette invalidates the cache.

use super::cache::{palette_hash, IllustrationCache};
use crate::theme::{PaletteColors, ThemeRegistry};
use gpui_kit::gpui::Image;
use std::cell::RefCell;
use std::sync::Arc;

struct Active {
    cache: IllustrationCache,
    colors: PaletteColors,
    hash: u64,
}

impl Active {
    fn new() -> Self {
        let colors = ThemeRegistry::global().default_palette().colors;
        Self { cache: IllustrationCache::default(), colors, hash: palette_hash(&colors) }
    }
}

thread_local! {
    static ACTIVE: RefCell<Active> = RefCell::new(Active::new());
}

/// Themes illustrations with `colors`. When they differ from the current palette the cache
/// is cleared and the evicted images are returned, so the caller can release gpui's
/// rasterized copies with `image.remove_asset(cx)`.
pub fn set_active_palette(colors: &PaletteColors) -> Vec<Arc<Image>> {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let hash = palette_hash(colors);
        if hash == active.hash {
            return Vec::new();
        }
        active.colors = *colors;
        active.hash = hash;
        active.cache.invalidate()
    })
}

/// The palette illustrations are currently themed with.
pub fn active_palette() -> PaletteColors {
    ACTIVE.with(|active| active.borrow().colors)
}

/// Runs `f` with this thread's cache and active palette.
pub fn with_active_cache<R>(f: impl FnOnce(&mut IllustrationCache, &PaletteColors) -> R) -> R {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let Active { cache, colors, .. } = &mut *active;
        f(cache, colors)
    })
}

/// Drops cached images whose source starts with `prefix` (e.g. after re-importing a photo)
/// and returns them for `remove_asset`.
pub fn forget_illustrations(prefix: &str) -> Vec<Arc<Image>> {
    with_active_cache(|cache, _| cache.forget_source(prefix))
}
