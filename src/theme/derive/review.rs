//! Palette-independent sizes of the Review page (sb C1): the page and item measure, the
//! grade buttons, the statistics charts and heatmap, and the sheets' height; plus (sb C1b)
//! the learn sheet and the connection map canvas.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Page and item measure, grade buttons, chart heights and bar bounds, heatmap cells,
/// legend swatches, the empty-state art and the sheets' height.
pub(super) fn apply_review_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        review_page_max_width: px(920.0), review_card_max_width: px(680.0), review_card_min_height: px(220.0),
        review_progress_height: px(4.0), review_grade_button_height: px(60.0),
        review_chart_height: px(132.0), review_bar_min_height: px(2.0), review_bar_max_width: px(26.0),
        review_heatmap_cell_min: px(9.0), review_heatmap_cell_max: px(18.0), review_heatmap_gap: px(3.0),
        review_swatch_size: px(10.0), review_empty_art_width: px(240.0),
        review_maker_max_height: px(660.0), review_sentence_list_height: px(168.0),
        review_count_tile_width: px(148.0), review_stat_label_width: px(84.0),
    });
}

/// The learn sheet's width, and the connection map: canvas height, point chips, relation
/// labels and connection strokes (sb C1b).
pub(super) fn apply_review_map_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        review_learn_sheet_width: px(600.0), review_map_height: px(440.0), review_map_point_width: px(168.0),
        review_map_point_height: px(66.0), review_map_label_width: px(112.0), review_map_edge_width: px(2.0),
    });
}
