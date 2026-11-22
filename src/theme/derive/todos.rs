//! Palette-independent sizes of to-dos in posts (epic D).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// The sample column of the syntax help card.
pub(super) fn apply_todos_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        todo_help_sample_width: px(188.0),
    });
}
