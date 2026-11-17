//! [`PaletteHost`]: what the shell keeps for the palette and the shortcut sheet between
//! openings, and the edge functions that load and save the palette history.

use super::model::PaletteHistory;
use super::view::CommandPalette;
use crate::storage::{SettingKey, Vault};
use gpui_kit::*;

/// The open palette (if any), its persisted history, the shortcut sheet flag, and the focus
/// handles of the shell root and the sheet.
pub struct PaletteHost {
    pub palette: Option<Entity<CommandPalette>>,
    pub history: PaletteHistory,
    pub is_sheet_open: bool,
    /// Focus of the window root: keeps global shortcuts reachable when nothing else is focused.
    pub shell_focus: FocusHandle,
    pub sheet_focus: FocusHandle,
    /// Keeps the open palette's event subscription alive.
    pub subscription: Option<Subscription>,
}

impl PaletteHost {
    /// A closed palette with the history stored in `vault`.
    pub fn new(vault: Option<&Vault>, cx: &mut App) -> Self {
        Self {
            palette: None,
            history: load_history(vault),
            is_sheet_open: false,
            shell_focus: cx.focus_handle(),
            sheet_focus: cx.focus_handle(),
            subscription: None,
        }
    }
}

/// The stored palette history (empty without a vault or when unreadable).
pub fn load_history(vault: Option<&Vault>) -> PaletteHistory {
    vault
        .and_then(|vault| vault.get_setting(SettingKey::PaletteHistory).ok().flatten())
        .map(|raw| PaletteHistory::from_json(&raw))
        .unwrap_or_default()
}

/// Queues `history` for the vault.
pub fn save_history(vault: Option<&Vault>, history: &PaletteHistory) {
    let Some(vault) = vault else { return };
    if let Err(err) = vault.set_setting(SettingKey::PaletteHistory, &history.to_json()) {
        eprintln!("simulacra: palette history not saved: {err}");
    }
}
