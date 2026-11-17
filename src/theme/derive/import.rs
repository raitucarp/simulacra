//! Palette-independent sizes of the import sheet, its unit preview, the quick-capture box and
//! the file-drop overlay.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Import sheet and quick capture geometry.
pub(super) fn apply_import_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        import_sheet_width: px(760.0), import_sheet_max_height: px(720.0),
        import_preview_height: px(300.0), import_preview_row_height: px(52.0),
        import_art_width: px(112.0), import_meta_label_width: px(88.0),
        capture_box_width: px(560.0), capture_box_top: px(96.0), drop_art_width: px(200.0),
    });
}
