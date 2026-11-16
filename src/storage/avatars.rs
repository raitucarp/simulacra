//! Avatar photos live inside the vault folder (next to `simulacra.db`) as
//! `avatars/<profile_id>.<ext>`, so a vault stays self-contained when it is moved or synced.
//! Profiles store that relative path; [`resolve_photo_path`] turns it back into a file path.

use crate::domain::ProfileId;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// Folder of imported avatar photos inside the vault.
pub const AVATARS_DIR: &str = "avatars";
/// Separator of stored relative paths (portable across platforms).
const STORED_SEPARATOR: char = '/';
/// Replacement for profile-id characters that are unsafe in file names.
const UNSAFE_CHAR_REPLACEMENT: char = '_';
/// Image extensions accepted for import (matches the image picker).
pub const PHOTO_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "webp", "gif", "bmp"];

/// Vault folder of the running app (set once the database is open).
static VAULT_DIR: RwLock<Option<PathBuf>> = RwLock::new(None);

/// Records the vault folder used to resolve relative avatar paths (`None` = in-memory).
pub fn register_vault_dir(dir: Option<PathBuf>) {
    if let Ok(mut slot) = VAULT_DIR.write() {
        *slot = dir;
    }
}

/// The registered vault folder, if any.
pub fn vault_dir() -> Option<PathBuf> {
    VAULT_DIR.read().ok().and_then(|slot| slot.clone())
}

/// The vault folder of a database file path (`None` for an in-memory database).
pub fn vault_dir_for(db_path: &str) -> Option<PathBuf> {
    if db_path.trim().is_empty() {
        return None;
    }
    Path::new(db_path).parent().map(Path::to_path_buf)
}

/// File-name-safe form of a profile id.
fn safe_stem(profile_id: &ProfileId) -> String {
    let safe = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    profile_id.as_str().chars().map(|c| if safe(c) { c } else { UNSAFE_CHAR_REPLACEMENT }).collect()
}

/// Stored (vault-relative) path of a profile's photo with extension `ext`.
pub fn avatar_relative_path(profile_id: &ProfileId, ext: &str) -> String {
    format!("{AVATARS_DIR}{STORED_SEPARATOR}{}.{}", safe_stem(profile_id), ext.to_ascii_lowercase())
}

/// The file a stored avatar path points to: absolute paths as-is, relative ones inside `vault`.
pub fn resolve_photo_path(vault: Option<&Path>, stored: &str) -> PathBuf {
    let path = Path::new(stored.trim());
    match vault {
        Some(dir) if path.is_relative() => dir.join(path),
        _ => path.to_path_buf(),
    }
}

/// Lowercased, supported extension of `source`.
fn photo_extension(source: &Path) -> io::Result<String> {
    let ext = source.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
    if PHOTO_EXTENSIONS.contains(&ext.as_str()) {
        Ok(ext)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, format!("unsupported avatar image `{}`", source.display())))
    }
}

/// Copies `source` into `vault/avatars/<profile_id>.<ext>`, removing the profile's photos with
/// other extensions, and returns the stored relative path.
pub fn import_photo(vault: &Path, profile_id: &ProfileId, source: &Path) -> io::Result<String> {
    let ext = photo_extension(source)?;
    let relative = avatar_relative_path(profile_id, &ext);
    let target = resolve_photo_path(Some(vault), &relative);
    std::fs::create_dir_all(vault.join(AVATARS_DIR))?;
    let same_file = |a: &Path, b: &Path| matches!((a.canonicalize(), b.canonicalize()), (Ok(x), Ok(y)) if x == y);
    if !same_file(source, &target) {
        std::fs::copy(source, &target)?;
    }
    for other in PHOTO_EXTENSIONS.iter().filter(|e| **e != ext) {
        let stale = resolve_photo_path(Some(vault), &avatar_relative_path(profile_id, other));
        if stale.is_file() {
            std::fs::remove_file(stale)?;
        }
    }
    Ok(relative)
}

#[cfg(test)]
mod tests;
