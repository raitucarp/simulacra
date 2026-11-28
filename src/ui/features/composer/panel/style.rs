//! The composer's "Style" toggle: the tint and symbol rows stay folded until asked for, so a
//! new thread opens on its text and its few essential controls. A thread that already has a
//! tint or symbols keeps the rows open, so a choice is never hidden.

use super::ComposerModalPanel;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Folds or unfolds the tint and symbol rows.
    pub(super) fn toggle_style(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.style_open = !self.style_open;
        cx.notify();
    }

    /// Whether the tint and symbol rows show.
    pub(super) fn style_shown(&self) -> bool {
        self.style_open || self.tint.is_some() || self.symbols.chosen.face().is_some()
    }
}
