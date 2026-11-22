//! Custom symbols as assets (S14): `sym:<set>/<name>` resolves to the icon's SVG in the
//! downloaded icon store. The shell points the asset source at the store once it knows where
//! the vault lives; until then (and for anything not installed) nothing resolves.

use crate::domain::symbols::SYMBOL_ASSET_SCHEME;
use crate::icons::IconStore;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::RwLock;

/// The icon store's folder, once known.
static STORE_ROOT: RwLock<Option<PathBuf>> = RwLock::new(None);

/// Points `sym:` assets at the store in `root` (`None` forgets it).
pub fn set_symbol_store(root: Option<PathBuf>) {
    if let Ok(mut current) = STORE_ROOT.write() {
        *current = root;
    }
}

/// The SVG of the `sym:<set>/<name>` asset `path`; `None` for other paths and for icons not
/// in the store.
pub fn load_symbol(path: &str) -> Option<Cow<'static, [u8]>> {
    let (set, name) = path.strip_prefix(SYMBOL_ASSET_SCHEME)?.split_once('/')?;
    let root = STORE_ROOT.read().ok()?.clone()?;
    IconStore::new(root).icon(set, name).map(Cow::Owned)
}
