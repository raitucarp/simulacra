//! Illustration packs (WS5): alternative art for the Classic SVG templates.
//!
//! **Classic** (the palette-tinted templates under `assets/illustrations/`) is always built in
//! and draws every key a pack does not cover. A **raster pack** is a `pack.json` manifest
//! ([`PackManifest`]) plus WebP files with alpha, one per covered key. A key is the Classic
//! asset path without `illustrations/` and `.svg` (`empty/no_posts`), so every illustration
//! keeps working without call-site changes ([`key_of_asset_path`]).
//!
//! Packs travel as `.simpack` files: a zip holding `pack.json` at its root and every covered
//! file at its manifest path. Nothing is installed unless every file matches the SHA-256 the
//! manifest pins for it. Everything here is pure: reading archives and the pack store live in
//! [`crate::packs`], drawing them in `ui::illustration::pack`.
//!
//! **Layered families** (`portrait/**`, `forms/**`) are 256×256-aligned layers drawn in key
//! colours (grey [`PRIMARY_KEY`], cyan [`SECONDARY_KEY`]) that [`tint`] recolours at runtime.

mod checksum;
mod downloadable;
mod error;
mod keys;
mod manifest;
pub mod tint;

pub use checksum::{sha256_hex, sha256_integrity};
pub use downloadable::{builtin_downloadable_packs, parse_downloadable, DownloadablePack, DOWNLOADABLE_JSON};
pub use error::{PackError, PackRule};
pub use keys::{asset_path_of_key, is_layered_key, is_new_art_key, is_safe_file, is_safe_id, is_safe_key, key_of_asset_path, NEW_ART_PREFIXES};
pub use manifest::{PackFile, PackManifest, MAX_FILES, MAX_FILE_BYTES, MAX_PACK_BYTES};
pub use tint::{LayerRole, PRIMARY_KEY, SECONDARY_KEY};

/// The always built-in vector pack (the fallback of every key).
pub const CLASSIC_PACK_ID: &str = "classic";
/// The built-in raster pack, and the default choice.
pub const FLAT_PACK_ID: &str = "flat";
/// The pack a fresh install uses.
pub const DEFAULT_PACK_ID: &str = FLAT_PACK_ID;
/// Packs shipped with the app (never installed or removed by the user).
pub const BUILT_IN_PACK_IDS: [&str; 2] = [CLASSIC_PACK_ID, FLAT_PACK_ID];
/// The manifest's file name, inside a pack folder and at the root of a `.simpack`.
pub const MANIFEST_FILE: &str = "pack.json";
/// File extension of a pack archive.
pub const PACK_ARCHIVE_EXT: &str = "simpack";

/// Whether `id` names a pack shipped with the app.
pub fn is_built_in(id: &str) -> bool {
    BUILT_IN_PACK_IDS.contains(&id)
}

#[cfg(test)]
mod tests;
