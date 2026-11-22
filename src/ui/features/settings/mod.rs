//! The settings window: a searchable category rail (General, Appearance, Reading & Writing,
//! AI & Models, Embeddings, Images, Agents & Simulation, Learning, Import, Privacy & Data,
//! Notifications, Shortcuts, Advanced) beside grouped cards of setting rows.
//!
//! - `catalog`: the pure information architecture, commands, search and palette sections.
//! - `row` / `controls`: the setting row (label, description, control, reset to default).
//! - `layout`: frame, rail, header, body and footer; `panes`: the hosted AI provider,
//!   scheduler status and Appearance panes; `credentials`: the stored keys.
//!
//! Every preference is a typed `domain::preferences::SettingField`, saved as it changes;
//! only the AI provider draft waits for "Save & apply".

mod callbacks;
pub mod catalog;
mod consts;
mod controls;
mod credentials;
mod layout;
mod modal;
mod panes;
mod props;
mod provider_grid;
mod row;
mod tabs;
mod text;

pub use callbacks::{
    CommandAction, EditAction, PlainAction, RowAction, SelectPresetModelCallback, SelectProviderCallback, SettingsActions, TabAction,
};
pub use catalog::{settings_sections, SettingRow, SettingsCommand, SettingsSectionEntry, ThemeScale};
pub use credentials::{CredentialEditorView, CredentialRowView, CredentialsPaneProps};
pub use modal::SettingsModal;
pub use panes::{action_set, set_label, AppearanceEvent, AppearancePanel, LicenceView, SetDownload, SymbolSetAction, SymbolSetCallback, SymbolSetsProps};
pub use props::{AiPaneProps, PersonaOption, SchedulerStatus, SettingsData, SettingsInputs};
pub use tabs::SettingsTab;

#[cfg(test)]
mod tests;
