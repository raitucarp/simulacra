//! Palette-independent sizes of the Knowledge Vault: the facet rail, covers, the table
//! columns, the reader's measure and the entry page.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Facet rail and drawer, cover sizes (2:3), table columns, reader measure (about 70
/// characters per line), illustrations and the calm motion of the vault.
pub(super) fn apply_vault_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        vault_facet_rail_width: px(232.0), vault_facet_drawer_width: px(300.0),
        vault_cover_width: px(128.0), vault_cover_height: px(192.0), vault_cover_thumb_width: px(28.0),
        vault_entry_cover_width: px(152.0), reader_measure_width: px(680.0), vault_entry_side_width: px(300.0),
        vault_search_width: px(360.0), vault_editor_width: px(760.0), vault_meta_label_width: px(92.0),
        vault_column_creator_width: px(170.0), vault_column_kind_width: px(104.0), vault_column_year_width: px(64.0),
        vault_column_added_width: px(104.0), vault_column_links_width: px(56.0), vault_progress_height: px(3.0),
        vault_empty_art_width: px(240.0), vault_banner_width: px(320.0), vault_kind_art_width: px(80.0),
        vault_letter_cell_size: px(26.0), vault_tree_indent: px(14.0),

        line_height_reading: 1.7, vault_motion_duration_ms: 220.0, vault_muted_opacity: 0.6,
    });
}
