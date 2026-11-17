//! The app-wide store of resizable layouts: saved sizes loaded from the vault's settings table
//! at startup, one [`ResizableState`] per layout id, and debounced saves after every drag.

use super::layouts::PanelLayouts;
use crate::storage::{SettingKey, Vault};
use gpui_kit::component::resizable::ResizableState;
use gpui_kit::*;
use std::collections::HashMap;
use std::time::Duration;

/// Quiet time after the last resize before the layouts are written to the vault.
const SAVE_DEBOUNCE: Duration = Duration::from_millis(400);

/// Saved sizes, live split states and the pending save.
#[derive(Default)]
struct PanelLayoutStore {
    layouts: PanelLayouts,
    states: HashMap<SharedString, Entity<ResizableState>>,
    vault: Option<Vault>,
    pending_save: Option<Task<()>>,
}

impl Global for PanelLayoutStore {}

/// Loads the saved layouts from `vault` (no vault: sizes live for this session only).
pub fn init_panel_layouts(vault: Option<&Vault>, cx: &mut App) {
    let raw = vault.and_then(|vault| vault.get_setting(SettingKey::PanelLayouts).ok().flatten());
    let store = cx.default_global::<PanelLayoutStore>();
    store.layouts = raw.as_deref().map(PanelLayouts::from_json).unwrap_or_default();
    store.states.clear();
    store.vault = vault.cloned();
}

/// The split state of layout `id` (created on first use).
pub(super) fn state_for(id: &SharedString, cx: &mut App) -> Entity<ResizableState> {
    if let Some(state) = cx.try_global::<PanelLayoutStore>().and_then(|store| store.states.get(id)) {
        return state.clone();
    }
    let state = cx.new(|_| ResizableState::default());
    cx.default_global::<PanelLayoutStore>().states.insert(id.clone(), state.clone());
    state
}

/// Saved size of panel `ix` of layout `id`.
pub(super) fn saved_size(id: &str, ix: usize, cx: &App) -> Option<f32> {
    cx.try_global::<PanelLayoutStore>().and_then(|store| store.layouts.size_of(id, ix))
}

/// Records the sizes of layout `id` after a drag and schedules a save.
pub(super) fn record(id: &str, sizes: &[Pixels], cx: &mut App) {
    let sizes = sizes.iter().map(|size| size.as_f32()).collect();
    if cx.default_global::<PanelLayoutStore>().layouts.set(id, sizes) {
        schedule_save(cx);
    }
}

/// Restores layout `id` to its default sizes (a fresh split state on the next frame).
pub fn reset_panel_layout(id: &str, cx: &mut App) {
    let store = cx.default_global::<PanelLayoutStore>();
    store.states.remove(id);
    if store.layouts.remove(id) {
        schedule_save(cx);
    }
    cx.refresh_windows();
}

/// Restores every layout to its default sizes and forgets the saved ones.
pub fn reset_panel_layouts(cx: &mut App) {
    let store = cx.default_global::<PanelLayoutStore>();
    store.states.clear();
    store.layouts.clear();
    store.pending_save = None;
    save_now(cx);
    cx.refresh_windows();
}

/// Writes the layouts once no resize happened for [`SAVE_DEBOUNCE`].
fn schedule_save(cx: &mut App) {
    let task = cx.spawn(async move |cx| {
        cx.background_executor().timer(SAVE_DEBOUNCE).await;
        cx.update(save_now);
    });
    cx.default_global::<PanelLayoutStore>().pending_save = Some(task);
}

/// Queues the current layouts for the vault.
fn save_now(cx: &mut App) {
    let store = cx.default_global::<PanelLayoutStore>();
    let Some(vault) = store.vault.as_ref() else { return };
    if let Err(err) = vault.set_setting(SettingKey::PanelLayouts, &store.layouts.to_json()) {
        eprintln!("simulacra: panel layout not saved: {err}");
    }
}
