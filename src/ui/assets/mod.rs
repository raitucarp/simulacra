//! The application's asset source, consulted in order:
//! 1. the app's own embedded assets (`assets/illustrations/**/*.svg` templates, plus the form
//!    catalogs `illustrations/forms/<kind>/catalog.json` embedded by the portrait domain),
//! 2. the Lucide icons the icon catalog uses ([`icons::CatalogIcons`]),
//! 3. gpui-kit's default component icons,
//! 4. downloaded icon set icons (`sym:<set>/<name>`, S14), from the icon store.
//!
//! Illustration templates are also read directly (without a `cx`) through [`template_text`].

mod icons;
mod symbols; // S14

pub use icons::CatalogIcons;
pub use symbols::{load_symbol, set_symbol_store};

use crate::domain::portrait::form::{FormCatalogFiles, FORM_ASSET_ROOT};

use gpui_kit::assets::Assets as KitAssets;
use gpui_kit::gpui::{AssetSource, Result, SharedString};
use std::borrow::Cow;

/// The app's bundled files (embedded in release builds, read from `assets/` in debug builds).
#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
#[include = "illustrations/**/*.svg"]
pub struct EmbeddedAssets;

/// Asset source installed with `application().with_assets(AppAssets)`.
#[derive(Clone, Copy, Debug, Default)]
pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(icon) = load_symbol(path) {
            return Ok(Some(icon)); // S14: a downloaded icon
        }
        if let Some(file) = EmbeddedAssets::get(path) {
            return Ok(Some(file.data));
        }
        if let Some(file) = form_catalog_name(path).and_then(FormCatalogFiles::get) {
            return Ok(Some(file.data));
        }
        if let Some(bytes) = CatalogIcons.load(path)? {
            return Ok(Some(bytes));
        }
        KitAssets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths: Vec<SharedString> =
            EmbeddedAssets::iter().filter(|name| name.starts_with(path)).map(|name| name.to_string().into()).collect();
        paths.extend(FormCatalogFiles::iter().map(|name| format!("{FORM_ASSET_ROOT}/{name}")).filter(|name| name.starts_with(path)).map(Into::into));
        paths.extend(CatalogIcons.list(path)?);
        paths.extend(KitAssets.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

/// The catalog file name (`object/catalog.json`) of a form catalog asset path.
fn form_catalog_name(path: &str) -> Option<&str> {
    path.strip_prefix(FORM_ASSET_ROOT)?.strip_prefix('/')
}

/// UTF-8 text of an embedded asset (`illustrations/...svg`), or `None` when absent or not UTF-8.
pub fn template_text(path: &str) -> Option<Cow<'static, str>> {
    match EmbeddedAssets::get(path)?.data {
        Cow::Borrowed(bytes) => std::str::from_utf8(bytes).ok().map(Cow::Borrowed),
        Cow::Owned(bytes) => String::from_utf8(bytes).ok().map(Cow::Owned),
    }
}

#[cfg(test)]
mod tests;
