//! The view state of the thread inspector and the profile page: the last view chosen in the
//! inspector and the view chosen per thread and per profile (W8, both in the settings table),
//! the outline's folded branches and cursor per thread and the calendar month (this session
//! only), and the outline's keyboard focus.

use crate::domain::views::calendar::YearMonth;
use crate::domain::views::memory::{ViewMemory, ViewOwner};
use crate::domain::views::PostView;
use crate::domain::PostId;
use crate::storage::{SettingKey, Vault};
use crate::ui::features::feed::inspector_outline::{InspectorView, OutlineMemory};
use gpui_kit::{App, FocusHandle};

/// What the inspector remembers.
pub struct InspectorState {
    /// The view chosen last (what a thread never switched opens in).
    pub mode: PostView,
    /// W8: the view chosen per thread and per profile.
    pub memory: ViewMemory,
    pub outlines: OutlineMemory,
    pub focus: FocusHandle,
    /// W4: the calendar month chosen in one thread (this session).
    pub month: Option<(PostId, YearMonth)>,
}

impl InspectorState {
    /// The saved mode (cards when unset or unreadable) and the saved choices.
    pub(super) fn load(vault: Option<&Vault>, cx: &mut App) -> Self {
        let read = |key: SettingKey| vault.and_then(|vault| vault.get_setting(key).ok().flatten());
        let mode = read(SettingKey::InspectorMode).and_then(|key| PostView::from_key(&key)).unwrap_or_default();
        let memory = ViewMemory::from_json(read(SettingKey::ViewChoices).as_deref());
        Self { mode, memory, outlines: OutlineMemory::default(), focus: cx.focus_handle(), month: None }
    }

    /// Queues `mode` for the settings table.
    pub(super) fn save_mode(vault: Option<&Vault>, mode: PostView) {
        let Some(vault) = vault else { return };
        if let Err(err) = vault.set_setting(SettingKey::InspectorMode, mode.key()) {
            eprintln!("simulacra: inspector mode not saved: {err}");
        }
    }

    /// W8: remembers `view` for `owner` and queues the choices for the settings table.
    pub(super) fn remember(&mut self, vault: Option<&Vault>, owner: &ViewOwner, view: PostView) {
        self.memory.remember(owner, view);
        let Some(vault) = vault else { return };
        if let Err(err) = vault.set_setting(SettingKey::ViewChoices, &self.memory.to_json()) {
            eprintln!("simulacra: view choices not saved: {err}");
        }
    }

    /// The view the feed renders.
    pub fn view(&self) -> InspectorView {
        InspectorView { mode: self.mode, outlines: self.outlines.clone(), focus: self.focus.clone(), argument: None, month: self.month.clone() }
    }
}
