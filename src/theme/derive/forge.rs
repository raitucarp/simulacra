//! Palette-independent sizes of the Agent Forge character creator.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Column widths, art sizes and opacities of the forge screens.
pub(super) fn apply_forge_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        forge_rail_width: px(212.0), forge_rail_thumb_width: px(64.0), forge_options_width: px(420.0),
        forge_preview_width: px(320.0), forge_topbar_height: px(60.0), forge_portrait_size: px(232.0),
        forge_nameplate_avatar_size: px(44.0), forge_class_card_width: px(164.0),
        forge_option_card_width: px(108.0), forge_part_thumb_size: px(64.0), forge_swatch_size: px(28.0),
        forge_guide_size: px(64.0), forge_radar_size: px(210.0), forge_skill_bar_height: px(8.0),
        forge_trading_card_width: px(360.0), forge_explainer_width: px(300.0),
        forge_empty_art_width: px(240.0), forge_tile_min_height: px(96.0), forge_bubble_max_width: px(340.0),
        forge_preset_card_width: px(104.0), forge_compact_card_width: px(80.0),
        // Daily Life clock ring and week strip, Relationships heart pips
        forge_clock_size: px(168.0), forge_clock_ring_width: px(16.0), forge_week_cell_height: px(12.0),
        forge_heart_size: px(18.0),

        forge_disabled_opacity: 0.45, forge_backdrop_opacity: 0.35,
    });
}
