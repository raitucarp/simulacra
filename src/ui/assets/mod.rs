//! The application's asset source, consulted in order:
//! 1. the app's own embedded assets (`assets/illustrations/**/*.svg` templates),
//! 2. the Lucide icons the icon catalog uses ([`icons::CatalogIcons`]),
//! 3. gpui-kit's default component icons.
//!
//! Illustration templates are also read directly (without a `cx`) through [`template_text`].

mod icons;

pub use icons::CatalogIcons;

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
        if let Some(file) = EmbeddedAssets::get(path) {
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
        paths.extend(CatalogIcons.list(path)?);
        paths.extend(KitAssets.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
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
