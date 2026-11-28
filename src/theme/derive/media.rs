//! Palette-independent sizes of post media (sb C3): the link and video cards, gallery seams,
//! the lightbox chrome and the composer's attachment rows.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Link card thumbnail and site icon, gallery seams, play badge, lightbox, composer rows, the file card art (L4) and how far a missing file's art is dimmed (L11).
pub(super) fn apply_media_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        link_card_thumb_size: px(104.0), link_card_icon_size: px(14.0), media_gallery_gap: px(2.0),
        video_play_badge_size: px(52.0), lightbox_inset: px(56.0), lightbox_button_size: px(44.0),
        lightbox_caption_width: px(640.0), composer_attachment_thumb_size: px(56.0), composer_meter_row_height: px(24.0), media_skeleton_line_width: px(220.0),
        media_single_max_height: px(520.0), file_card_art_width: px(64.0),
        file_missing_opacity: 0.45, clap_idle_opacity: 0.5,
    });
}
