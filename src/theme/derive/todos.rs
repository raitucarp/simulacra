//! Palette-independent sizes of to-dos in posts (epic D).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// The sample column of the syntax help card, the indent of each nesting level and a box.
pub(super) fn apply_todos_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        todo_help_sample_width: px(188.0), todo_indent: px(22.0), todo_box_size: px(18.0),
    });
}
