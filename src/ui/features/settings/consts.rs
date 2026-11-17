//! Element ids and key names of the settings window. Every size is a theme token.

/// Number of provider buttons rendered on the first grid row.
pub(super) const PROVIDERS_PER_ROW: usize = 3;
/// Choices with at most this many options are chips; longer lists open a menu.
pub(super) const MAX_INLINE_CHOICES: usize = 3;

pub(super) const BACKDROP_ID: &str = "settings-modal-backdrop";
pub(super) const DIALOG_ID: &str = "settings-modal-dialog";
/// Layout id of the rail | content split (sizes persist).
pub(super) const SPLIT_ID: &str = "settings-modal-split";
pub(super) const RAIL_ID: &str = "settings-rail";
pub(super) const BODY_ID: &str = "settings-body";
pub(super) const CLOSE_BUTTON_ID: &str = "btn-close-settings-x";
pub(super) const CANCEL_BUTTON_ID: &str = "btn-cancel-settings";
pub(super) const SAVE_BUTTON_ID: &str = "btn-save-settings";
pub(super) const TAB_BUTTON_ID_PREFIX: &str = "settings-tab-btn-";
pub(super) const PROVIDER_BUTTON_ID_PREFIX: &str = "provider-btn-";
pub(super) const PRESET_CHIP_ID_PREFIX: &str = "preset-";
/// Prefix of every row control id (`settings-row-<row id>-<part>`).
pub(super) const ROW_ID_PREFIX: &str = "settings-row-";

/// gpui key names handled by the window.
pub(super) const KEY_ESCAPE: &str = "escape";
pub(super) const KEY_UP: &str = "up";
pub(super) const KEY_DOWN: &str = "down";
/// Hour readout of an hour-of-day number (`22:00`).
pub(super) const HOUR_SUFFIX: &str = ":00";
