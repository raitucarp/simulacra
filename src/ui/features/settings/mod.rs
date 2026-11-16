//! Global settings modal dialog featuring vertical category tabs
//! (General, AI Intelligence, Appearance, Knowledge Vault) with true Threads OLED styling.

mod callbacks;
mod consts;
mod content;
mod modal;
mod panes;
mod provider_grid;
mod rail;
mod tabs;
mod theme_card;

pub use callbacks::{
    ModalCloseCallback, SaveSettingsCallback, SelectPresetModelCallback, SelectProviderCallback,
    SelectTabCallback, SelectThemeCallback,
};
pub use modal::SettingsModal;
pub use tabs::SettingsTab;
