//! [`RasterPack`]: a pack's manifest and where its files are (embedded in the app or in an
//! installed pack's folder). Every file read is checked against its pinned SHA-256, so a
//! damaged or edited file draws Classic instead.

use crate::domain::packs::{sha256_hex, PackManifest, FLAT_PACK_ID, MANIFEST_FILE};
use crate::packs::InstalledPack;
use std::path::Path;

/// The built-in Flat pack (embedded in release builds, read from `assets/packs/flat/` in
/// debug builds).
#[derive(rust_embed::RustEmbed)]
#[folder = "assets/packs/flat"]
#[include = "pack.json"]
#[include = "**/*.webp"]
#[include = "**/*.png"]
struct FlatPackFiles;

/// Where a pack's files live.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Origin {
    BuiltIn,
    Folder(InstalledPack),
}

/// A raster pack the loader can draw from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RasterPack {
    pub manifest: PackManifest,
    origin: Origin,
}

impl RasterPack {
    /// The built-in Flat pack (`None` only if its manifest is broken).
    pub fn flat() -> Option<Self> {
        let json = FlatPackFiles::get(MANIFEST_FILE)?.data;
        let manifest = PackManifest::parse(&String::from_utf8_lossy(&json)).ok()?;
        (manifest.id == FLAT_PACK_ID).then_some(Self { manifest, origin: Origin::BuiltIn })
    }

    /// An installed pack.
    pub fn installed(pack: InstalledPack) -> Self {
        Self { manifest: pack.manifest.clone(), origin: Origin::Folder(pack) }
    }

    /// The pack in folder `dir` (its `pack.json` and files), if its manifest is valid.
    pub fn from_dir(dir: &Path) -> Option<Self> {
        let manifest = PackManifest::parse(&std::fs::read_to_string(dir.join(MANIFEST_FILE)).ok()?).ok()?;
        Some(Self::installed(InstalledPack { manifest, dir: dir.to_path_buf() }))
    }

    pub fn id(&self) -> &str {
        &self.manifest.id
    }

    /// Whether the pack ships with the app.
    pub fn is_built_in(&self) -> bool {
        self.origin == Origin::BuiltIn
    }

    /// Whether the pack draws `key`.
    pub fn covers(&self, key: &str) -> bool {
        self.manifest.covers(key)
    }

    /// The verified bytes of the file covering `key`.
    pub fn bytes(&self, key: &str) -> Option<Vec<u8>> {
        let entry = self.manifest.file(key)?;
        let data = match &self.origin {
            Origin::BuiltIn => FlatPackFiles::get(&entry.file)?.data.into_owned(),
            Origin::Folder(pack) => std::fs::read(pack.file_path(&entry.file)?).ok()?,
        };
        (sha256_hex(&data) == entry.sha256).then_some(data)
    }
}
