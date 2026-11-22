//! The composer's to-do helpers (D1): whether the syntax help card is open.

use super::ComposerModalPanel;
use gpui_kit::*;

/// The to-do helpers' state.
#[derive(Debug, Clone, Default)]
pub(super) struct TodoAssist {
    /// The syntax help card is shown.
    pub help: bool,
}

impl ComposerModalPanel {
    /// Opens or closes the to-do syntax help card.
    pub(super) fn toggle_todo_help(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.todo.help = !self.todo.help;
        cx.notify();
    }

    /// Whether the syntax help card is open.
    pub(super) fn todo_help_open(&self) -> bool {
        self.todo.help
    }
}
