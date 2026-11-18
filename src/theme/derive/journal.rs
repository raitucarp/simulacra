//! Palette-independent sizes of the Journal page (sb E3): the page and card measure, the
//! entry editor, the affect grid and its dot and labels, the heat strip, chart heights, the
//! focus timer's figures and progress track, energy buttons, focus mode's column, capsule
//! thumbnails and list heights.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Every Journal size token.
pub(super) fn apply_journal_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        journal_page_max_width: px(960.0), journal_card_max_width: px(700.0), journal_editor_min_height: px(220.0),
        journal_grid_size: px(300.0), journal_grid_dot_size: px(16.0), journal_grid_label_width: px(84.0),
        journal_heat_cell: px(14.0), journal_heat_gap: px(3.0), journal_chart_height: px(120.0),
        journal_timer_text: px(56.0), journal_timer_text_large: px(96.0), journal_progress_height: px(6.0),
        journal_energy_button_size: px(36.0), journal_focus_mode_width: px(620.0), journal_thumb_size: px(72.0),
        journal_list_max_height: px(420.0), journal_row_label_width: px(132.0),
    });
}
