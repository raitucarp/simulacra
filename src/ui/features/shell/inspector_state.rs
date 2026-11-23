//! The thread inspector's view state (sb C2): cards or outline (remembered in the settings
//! table), the outline's folded branches and cursor per thread (this session only), and the
//! outline's keyboard focus.

use crate::storage::{SettingKey, Vault};
use crate::domain::views::PostView;
use crate::ui::features::feed::inspector_outline::{InspectorView, OutlineMemory};
use gpui_kit::{App, FocusHandle};

/// What the inspector remembers.
pub struct InspectorState {
    pub mode: PostView,
    pub outlines: OutlineMemory,
    pub focus: FocusHandle,
}

impl InspectorState {
    /// The saved mode (cards when unset or unreadable).
    pub(super) fn load(vault: Option<&Vault>, cx: &mut App) -> Self {
        let saved = vault.and_then(|vault| vault.get_setting(SettingKey::InspectorMode).ok().flatten());
        let mode = saved.and_then(|key| PostView::from_key(&key)).unwrap_or_default();
        Self { mode, outlines: OutlineMemory::default(), focus: cx.focus_handle() }
    }

    /// Queues `mode` for the settings table.
    pub(super) fn save_mode(vault: Option<&Vault>, mode: PostView) {
        let Some(vault) = vault else { return };
        if let Err(err) = vault.set_setting(SettingKey::InspectorMode, mode.key()) {
            eprintln!("simulacra: inspector mode not saved: {err}");
        }
    }

    /// The view the feed renders.
    pub fn view(&self) -> InspectorView {
        InspectorView { mode: self.mode, outlines: self.outlines.clone(), focus: self.focus.clone(), argument: None }
    }
}
