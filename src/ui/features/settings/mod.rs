//! Global settings modal dialog featuring vertical category tabs
//! (General, AI Intelligence, Appearance, Knowledge Vault) with true Threads OLED styling.

mod callbacks;
mod consts;
mod content;
mod general_props;
mod modal;
mod panes;
mod provider_grid;
mod rail;
mod tabs;

pub use callbacks::{
    ModalCloseCallback, SaveSettingsCallback, SelectPresetModelCallback, SelectProviderCallback,
    SelectTabCallback, ToggleSchedulerCallback,
};
pub use panes::{AppearanceEvent, AppearancePanel};
pub use general_props::GeneralPaneProps;
pub use modal::SettingsModal;
pub use tabs::SettingsTab;
