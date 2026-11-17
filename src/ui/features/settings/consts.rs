//! Element ids, grid shape and labels of the settings modal.
//! Every size is a theme token (`settings_*`, `font_size_*`, `icon_size_*`).

/// Number of provider buttons rendered on the first grid row.
pub(super) const PROVIDERS_PER_ROW: usize = 3;

pub(super) const BACKDROP_ID: &str = "settings-modal-backdrop";
pub(super) const DIALOG_ID: &str = "settings-modal-dialog";
/// Layout id of the rail | content split (sizes persist).
pub(super) const SPLIT_ID: &str = "settings-modal-split";
pub(super) const CLOSE_BUTTON_ID: &str = "btn-close-settings-x";
pub(super) const CANCEL_BUTTON_ID: &str = "btn-cancel-settings";
pub(super) const SAVE_BUTTON_ID: &str = "btn-save-settings";
pub(super) const TAB_BUTTON_ID_PREFIX: &str = "settings-tab-btn-";
pub(super) const PROVIDER_BUTTON_ID_PREFIX: &str = "provider-btn-";
pub(super) const PRESET_CHIP_ID_PREFIX: &str = "preset-";
pub(super) const SCHEDULER_SWITCH_ID: &str = "settings-scheduler-switch";
