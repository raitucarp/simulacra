//! Palette-independent sizes of the illustrations spread across the app and of the image
//! ideas sheet (sb D3).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Empty-state, notice and spot art widths; the image ideas card, its pick column, option
/// chips (art and frame) and result thumbnails.
pub(super) fn apply_image_prompt_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        art_empty_width: px(200.0), art_notice_width: px(112.0), art_spot_width: px(88.0), art_compact_width: px(72.0),
        image_ideas_sheet_width: px(980.0), image_ideas_sheet_height: px(640.0), image_ideas_picks_width: px(440.0),
        image_ideas_chip_width: px(92.0), image_ideas_chip_art_width: px(68.0), image_ideas_result_size: px(132.0),
    });
}
