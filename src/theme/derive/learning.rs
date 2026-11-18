//! Palette-independent sizes of the Review page's Learn tab (sb E2): the tutor's question
//! bubbles, answer box and Bloom ladder, the goals' side column and date field, the gap
//! radar and its ranked bars, and the memory palace's scenes, locus markers, room
//! thumbnails and cue pictures.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Every Learn tab size token.
pub(super) fn apply_learning_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        learning_bubble_max_width: px(560.0), learning_answer_min_height: px(84.0), learning_ladder_dot_size: px(10.0),
        learning_side_width: px(300.0), learning_date_field_width: px(150.0),
        learning_radar_min: px(240.0), learning_radar_max: px(360.0), learning_rank_bar_width: px(120.0), learning_rank_label_width: px(150.0),
        learning_scene_width: px(440.0), learning_walk_scene_width: px(480.0), learning_marker_size: px(26.0),
        learning_room_thumb_width: px(96.0), learning_cue_image_size: px(72.0), learning_list_max_height: px(320.0),
    });
}
