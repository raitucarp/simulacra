//! Installed illustration packs: one folder per pack under `packs/` next to the vault (beside
//! the downloaded icon sets), holding its `pack.json` and files. A pack is written to a
//! scratch folder first and moved into place only once complete, so a failed install leaves
//! nothing half done.

use super::archive::VerifiedPack;
use crate::domain::packs::{is_built_in, is_safe_file, is_safe_id, PackError, PackManifest, MANIFEST_FILE};
use std::fs;
use std::path::{Path, PathBuf};

/// The store's folder next to the vault.
pub const PACKS_DIR_NAME: &str = "packs";
/// Suffix of a pack's folder while it is being written.
const PARTIAL_SUFFIX: &str = ".partial";
/// The store folder used when the vault is in memory.
const TEMP_DIR_NAME: &str = "simulacra-packs";

/// The pack folder of the database at `db_path` (a temp folder for an in-memory vault).
pub fn packs_dir_for(db_path: &str) -> PathBuf {
    match Path::new(db_path).parent().filter(|_| !db_path.trim().is_empty()) {
        Some(dir) => dir.join(PACKS_DIR_NAME),
        None => std::env::temp_dir().join(TEMP_DIR_NAME),
    }
}

/// One installed pack: its manifest and folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPack {
    pub manifest: PackManifest,
    pub dir: PathBuf,
}

impl InstalledPack {
    /// The path of the pack file `file` (only plain relative paths, never one that climbs out).
    pub fn file_path(&self, file: &str) -> Option<PathBuf> {
        is_safe_file(file).then(|| file.split('/').fold(self.dir.clone(), |path, segment| path.join(segment)))
    }
}

/// The pack store rooted at one folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackStore {
    root: PathBuf,
}

impl PackStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Every installed pack with a valid manifest naming its own folder, sorted by name.
    pub fn installed(&self) -> Vec<InstalledPack> {
        let Ok(entries) = fs::read_dir(&self.root) else { return Vec::new() };
        let mut packs: Vec<InstalledPack> = entries.flatten().filter_map(|entry| self.get(entry.file_name().to_str()?)).collect();
        packs.sort_by(|a, b| a.manifest.name.cmp(&b.manifest.name).then_with(|| a.manifest.id.cmp(&b.manifest.id)));
        packs
    }

    /// The installed pack `id`, if present and valid.
    pub fn get(&self, id: &str) -> Option<InstalledPack> {
        if !is_safe_id(id) || is_built_in(id) {
            return None;
        }
        let dir = self.root.join(id);
        let manifest = PackManifest::parse(&fs::read_to_string(dir.join(MANIFEST_FILE)).ok()?).ok()?;
        (manifest.id == id).then_some(InstalledPack { manifest, dir })
    }

    /// Installs the verified `pack`, replacing an older copy of the same id.
    pub fn install(&self, pack: &VerifiedPack) -> Result<InstalledPack, PackError> {
        let id = &pack.manifest.id;
        if is_built_in(id) || !is_safe_id(id) {
            return Err(PackError::BuiltIn(id.clone()));
        }
        let io = |e: std::io::Error| PackError::Io(e.to_string());
        let (folder, scratch) = (self.root.join(id), self.root.join(format!("{id}{PARTIAL_SUFFIX}")));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).map_err(io)?;
        let staged = InstalledPack { manifest: pack.manifest.clone(), dir: scratch.clone() };
        for (file, data) in &pack.files {
            let path = staged.file_path(file).ok_or_else(|| PackError::Missing(file.clone()))?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(io)?;
            }
            fs::write(path, data).map_err(io)?;
        }
        fs::write(scratch.join(MANIFEST_FILE), pack.manifest.to_json()).map_err(io)?;
        let _ = fs::remove_dir_all(&folder);
        fs::rename(&scratch, &folder).map_err(io)?;
        Ok(InstalledPack { manifest: pack.manifest.clone(), dir: folder })
    }

    /// Removes the installed pack `id`; `Ok(false)` when it was not installed. Built-in packs
    /// cannot be removed.
    pub fn remove(&self, id: &str) -> Result<bool, PackError> {
        if is_built_in(id) {
            return Err(PackError::BuiltIn(id.to_string()));
        }
        if !is_safe_id(id) {
            return Ok(false);
        }
        let folder = self.root.join(id);
        if !folder.is_dir() {
            return Ok(false);
        }
        fs::remove_dir_all(&folder).map_err(|e| PackError::Io(e.to_string()))?;
        Ok(true)
    }
}
