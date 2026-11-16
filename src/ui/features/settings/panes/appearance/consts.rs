//! Element ids and option lists of the Appearance pane (sizes are theme tokens).

/// Interface font families offered (installed with Windows; the mono family is fixed).
pub(super) const FONT_FAMILIES: [&str; 5] = ["Segoe UI", "Calibri", "Georgia", "Verdana", "Trebuchet MS"];

/// WCAG level labels (identical in every language).
pub(super) const GRADE_AAA: &str = "AAA";
pub(super) const GRADE_AA: &str = "AA";
/// Ratio readout format suffix (`4.50:1`).
pub(super) const RATIO_SUFFIX: &str = ":1";

/// File dialog extension for theme files.
pub(super) const THEME_FILE_EXTENSIONS: [&str; 1] = ["json"];
/// Suffix appended to the palette id for the suggested export file name.
pub(super) const THEME_FILE_SUFFIX: &str = ".json";

pub(super) const CARD_ID_PREFIX: &str = "appearance-card-";
pub(super) const FILTER_ID_PREFIX: &str = "appearance-filter";
pub(super) const FONT_ID_PREFIX: &str = "appearance-font";
pub(super) const TYPE_SCALE_ID_PREFIX: &str = "appearance-type";
pub(super) const DENSITY_ID_PREFIX: &str = "appearance-density";
pub(super) const RADIUS_ID_PREFIX: &str = "appearance-radius";
pub(super) const DUPLICATE_BUTTON_ID: &str = "appearance-duplicate";
pub(super) const RESET_BUTTON_ID: &str = "appearance-reset";
pub(super) const DELETE_BUTTON_ID: &str = "appearance-delete";
pub(super) const COPY_JSON_ID: &str = "appearance-copy-json";
pub(super) const PASTE_JSON_ID: &str = "appearance-paste-json";
pub(super) const SAVE_FILE_ID: &str = "appearance-save-file";
pub(super) const OPEN_FILE_ID: &str = "appearance-open-file";
/// Joins an id prefix and an option key.
pub(super) const ID_SEPARATOR: &str = "-";
/// Prefix of the dynamic i18n keys of scale options and palette colors.
pub(super) const I18N_PREFIX: &str = "appearance.";
/// Joins an option group and an option key inside an i18n key (`density_cozy`).
pub(super) const I18N_JOINER: &str = "_";
pub(super) const COLOR_I18N_PREFIX: &str = "appearance.color_";
