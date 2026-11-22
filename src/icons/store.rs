//! The downloaded icon sets (S12): one folder per set under `symbols/` next to the vault
//! (`<set>/<name>.svg`, the set's `LICENCE.txt`), and an `index.json` of what is installed.
//! A set is written to a scratch folder first and moved into place only once complete, so a
//! failed or cancelled install leaves nothing half done.

use super::install::{build_set, BuiltSet};
use crate::domain::icons::{IconSet, StoreIndex, StoredSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The store's folder next to the vault, its index, and the file holding a set's licence.
pub const SYMBOLS_DIR_NAME: &str = "symbols";
const INDEX_FILE: &str = "index.json";
pub const LICENCE_FILE: &str = "LICENCE.txt";
/// Suffix of a set's folder while it is being written.
const PARTIAL_SUFFIX: &str = ".partial";
/// Icon file extension.
pub const ICON_EXT: &str = "svg";
/// The store folder used when the vault is in memory.
const TEMP_DIR_NAME: &str = "simulacra-symbols";

/// The store folder of the database at `db_path` (a temp folder for an in-memory vault).
pub fn symbols_dir_for(db_path: &str) -> PathBuf {
    match Path::new(db_path).parent().filter(|_| !db_path.trim().is_empty()) {
        Some(dir) => dir.join(SYMBOLS_DIR_NAME),
        None => std::env::temp_dir().join(TEMP_DIR_NAME),
    }
}

/// The icon store rooted at one folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconStore {
    root: PathBuf,
}

impl IconStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// What is installed (empty when nothing is, or the index cannot be read).
    pub fn index(&self) -> StoreIndex {
        fs::read_to_string(self.root.join(INDEX_FILE)).ok().and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default()
    }

    fn save_index(&self, index: &StoreIndex) -> io::Result<()> {
        fs::create_dir_all(&self.root)?;
        let json = serde_json::to_string_pretty(index).map_err(io::Error::other)?;
        let scratch = self.root.join(format!("{INDEX_FILE}{PARTIAL_SUFFIX}"));
        fs::write(&scratch, json)?;
        fs::rename(scratch, self.root.join(INDEX_FILE))
    }

    /// Installs `set` from the verified package tarball `package` (replacing an older copy).
    pub fn install(&self, package: &[u8], set: &IconSet, version: &str, now: jiff::Timestamp) -> Result<StoredSet, InstallError> {
        let built = build_set(package, set).map_err(InstallError::Package)?;
        let stored = self.write_set(set, &built, version, now).map_err(|e| InstallError::Io(e.to_string()))?;
        let mut index = self.index();
        index.put(stored.clone());
        self.save_index(&index).map_err(|e| InstallError::Io(e.to_string()))?;
        Ok(stored)
    }

    fn write_set(&self, set: &IconSet, built: &BuiltSet, version: &str, now: jiff::Timestamp) -> io::Result<StoredSet> {
        let (folder, scratch) = (self.set_dir(set.id), self.root.join(format!("{}{PARTIAL_SUFFIX}", set.id)));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch)?;
        let mut bytes = 0u64;
        for (name, svg) in &built.icons {
            fs::write(scratch.join(format!("{name}.{ICON_EXT}")), svg)?;
            bytes += svg.len() as u64;
        }
        fs::write(scratch.join(LICENCE_FILE), &built.licence)?;
        let _ = fs::remove_dir_all(&folder);
        fs::rename(&scratch, &folder)?;
        Ok(StoredSet { id: set.id.to_string(), icons: built.icons.len() as u32, bytes, version: version.to_string(), installed_at: crate::time::rfc3339(now) })
    }

    /// Removes the set `id`; `Ok(false)` when it was not installed.
    pub fn remove(&self, id: &str) -> io::Result<bool> {
        let mut index = self.index();
        let listed = index.take(id);
        let folder = self.set_dir(id);
        let present = folder.is_dir();
        if present {
            fs::remove_dir_all(&folder)?;
        }
        if listed {
            self.save_index(&index)?;
        }
        Ok(listed || present)
    }

    /// The folder of the set `id` (only plain set ids are accepted, never a path).
    fn set_dir(&self, id: &str) -> PathBuf {
        self.root.join(safe_segment(id).unwrap_or("_"))
    }

    /// The SVG of icon `name` of set `set`, if installed.
    pub fn icon(&self, set: &str, name: &str) -> Option<Vec<u8>> {
        let (set, name) = (safe_segment(set)?, safe_segment(name)?);
        fs::read(self.root.join(set).join(format!("{name}.{ICON_EXT}"))).ok()
    }

    /// The names of every icon of the set `id`, sorted.
    pub fn icon_names(&self, id: &str) -> Vec<String> {
        let Ok(entries) = fs::read_dir(self.set_dir(id)) else { return Vec::new() };
        let mut names: Vec<String> = entries.flatten().filter_map(|entry| entry.file_name().to_str()?.strip_suffix(&format!(".{ICON_EXT}")).map(str::to_string)).collect();
        names.sort();
        names
    }

    /// The licence text stored with the set `id`.
    pub fn licence(&self, id: &str) -> Option<String> {
        fs::read_to_string(self.set_dir(id).join(LICENCE_FILE)).ok()
    }
}

/// `segment` when it is a plain name (letters, digits, `-`, `_`), never a path.
fn safe_segment(segment: &str) -> Option<&str> {
    let plain = !segment.is_empty() && segment.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    plain.then_some(segment)
}

/// Why a set could not be installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallError {
    /// The package does not hold the set, or it is damaged.
    Package(String),
    Io(String),
}
