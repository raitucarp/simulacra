//! Vault-relative avatar photos: where the vault folder is, and importing a picked photo.

use super::AppState;
use crate::domain::portrait::AvatarMode;
use crate::domain::ProfileId;
use crate::storage::avatars::{import_photo, vault_dir_for};
use std::io;
use std::path::{Path, PathBuf};

/// Error text when photos cannot be imported because the store has no vault folder.
const NO_VAULT_FOLDER: &str = "no vault folder (in-memory store)";

impl AppState {
    /// Folder holding the open database (`None` for an in-memory store).
    pub fn vault_dir(&self) -> Option<PathBuf> {
        self.db.as_ref().and_then(|vault| vault_dir_for(&vault.path()))
    }

    /// Copies the picked photo at `source` into the vault (`avatars/<profile_id>.<ext>`) and
    /// returns the photo avatar to store in the profile's `config.avatar`. Callers replacing an
    /// earlier photo should then call `ui::illustration::forget_stored_photo` on the stored path.
    pub fn import_avatar_photo(&self, profile_id: &ProfileId, source: &Path) -> io::Result<AvatarMode> {
        let vault = self.vault_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, NO_VAULT_FOLDER))?;
        import_photo(&vault, profile_id, source).map(AvatarMode::photo)
    }
}
