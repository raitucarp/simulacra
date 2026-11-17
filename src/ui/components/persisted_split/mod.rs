//! Resizable splits whose panel sizes survive restarts.
//!
//! [`persisted_h_resizable`] / [`persisted_v_resizable`] build a gpui-kit resizable group
//! bound to an app-wide [`ResizableState`](gpui_kit::component::resizable::ResizableState) per
//! layout id. Sized panes reopen at their saved size (clamped to their token range); every
//! drag records the sizes and saves them, debounced, under
//! [`SettingKey::PanelLayouts`](crate::storage::SettingKey). Double-clicking a splitter
//! restores that split's defaults; [`reset_panel_layouts`] restores them all.

mod fit;
mod handle;
mod layouts;
mod split;
mod store;

#[cfg(test)]
mod tests;

pub use layouts::{restored_size, PanelLayouts};
pub use split::{persisted_h_resizable, persisted_v_resizable, PersistedSplit, SplitPane};
pub use store::{init_panel_layouts, reset_panel_layout, reset_panel_layouts};
