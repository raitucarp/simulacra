//! Loading and saving [`SidebarPrefs`] (`SettingKey::SidebarNavigation` in the settings table).

use super::SidebarPrefs;
use crate::storage::{SettingKey, Vault};

/// The stored prefs (defaults without a vault or when unreadable).
pub fn load_prefs(vault: Option<&Vault>) -> SidebarPrefs {
    vault
        .and_then(|vault| vault.get_setting(SettingKey::SidebarNavigation).ok().flatten())
        .map(|raw| SidebarPrefs::from_json(&raw))
        .unwrap_or_default()
}

/// Queues `prefs` for the vault.
pub fn save_prefs(vault: Option<&Vault>, prefs: &SidebarPrefs) {
    let Some(vault) = vault else { return };
    if let Err(err) = vault.set_setting(SettingKey::SidebarNavigation, &prefs.to_json()) {
        eprintln!("simulacra: sidebar preferences not saved: {err}");
    }
}
