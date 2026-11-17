//! Palette-independent sizes of the semantic search surfaces (sb B3): the similarity bar,
//! the indexing indicator and the "find similar" sheet.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Similarity bar and readout, the floating indexing pill, the similar-items sheet.
pub(super) fn apply_semantic_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        similarity_bar_width: px(56.0), similarity_bar_height: px(4.0), similarity_readout_width: px(30.0),
        indexing_indicator_width: px(260.0),
        similar_sheet_width: px(600.0), similar_sheet_max_height: px(480.0),
    });
}
