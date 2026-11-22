//! Palette-independent sizes of the data views (sb F2): the shared sheet (history, export,
//! snapshot, restore) and its version list, diff and label columns, the passphrase strength
//! meter, the Knowledge page's Library / Saved switch and the Saved view's column, chips,
//! drag grip and note field.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Every data view size token.
pub(super) fn apply_data_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        data_sheet_width: px(620.0), data_sheet_max_height: px(680.0), history_list_width: px(240.0),
        history_diff_min_height: px(160.0), data_label_width: px(116.0), data_count_width: px(56.0),
        data_date_field_width: px(148.0), export_pick_max_height: px(176.0), strength_meter_height: px(6.0),
        knowledge_switch_width: px(260.0), saved_page_max_width: px(860.0), saved_chip_max_width: px(220.0),
        saved_grip_width: px(20.0), saved_note_min_height: px(64.0), history_lane_width: px(14.0), history_lane_dot: px(8.0),
    });
}
