//! Hoisted literal sizes, element ids and labels of the settings modal.
//! Values are unchanged from the original inline literals; real tokenization is a later phase.

/// Smallest annotation text (e.g. the "Recommended" provider hint).
pub(super) const TEXT_MICRO: f32 = 9.0;
/// Caption text (rail subtitle, field hints, badges).
pub(super) const TEXT_CAPTION: f32 = 10.0;
/// Secondary/explanatory text.
pub(super) const TEXT_SMALL: f32 = 11.0;
/// Standard body and section label text.
pub(super) const TEXT_BODY: f32 = 12.0;
/// Tab labels and theme card titles.
pub(super) const TEXT_LABEL: f32 = 13.0;
/// Rail brand title.
pub(super) const TEXT_TITLE: f32 = 14.0;
/// Active tab heading.
pub(super) const TEXT_HEADING: f32 = 15.0;

pub(super) const ICON_XS: f32 = 11.0;
pub(super) const ICON_SM: f32 = 12.0;
pub(super) const ICON_MD: f32 = 13.0;
pub(super) const ICON_LG: f32 = 14.0;
pub(super) const ICON_XL: f32 = 15.0;

/// Active tab indicator bar dimensions.
pub(super) const TAB_INDICATOR_WIDTH: f32 = 3.0;
pub(super) const TAB_INDICATOR_HEIGHT: f32 = 16.0;
/// Rail brand badge square size.
pub(super) const BRAND_BADGE_SIZE: f32 = 26.0;
/// Content pane header height.
pub(super) const CONTENT_HEADER_HEIGHT: f32 = 64.0;
/// Provider selection button height.
pub(super) const PROVIDER_BUTTON_HEIGHT: f32 = 40.0;
/// Text input height for API key / endpoint / model fields.
pub(super) const INPUT_HEIGHT: f32 = 34.0;
/// Number of provider buttons rendered on the first grid row.
pub(super) const PROVIDERS_PER_ROW: usize = 3;
/// Monospace font stack for the database path readout.
pub(super) const MONO_FONT_STACK: &str = "Consolas, monospace";

pub(super) const BACKDROP_ID: &str = "settings-modal-backdrop";
pub(super) const DIALOG_ID: &str = "settings-modal-dialog";
pub(super) const CLOSE_BUTTON_ID: &str = "btn-close-settings-x";
pub(super) const CANCEL_BUTTON_ID: &str = "btn-cancel-settings";
pub(super) const SAVE_BUTTON_ID: &str = "btn-save-settings";
pub(super) const TAB_BUTTON_ID_PREFIX: &str = "settings-tab-btn-";
pub(super) const PROVIDER_BUTTON_ID_PREFIX: &str = "provider-btn-";
pub(super) const PRESET_CHIP_ID_PREFIX: &str = "preset-";
pub(super) const SCHEDULER_SWITCH_ID: &str = "settings-scheduler-switch";
