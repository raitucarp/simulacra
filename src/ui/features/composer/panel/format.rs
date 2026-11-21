//! Formatting the draft that last had focus (the footer toolbar and its shortcuts).

use super::super::format::apply_format;
use super::ComposerModalPanel;
use crate::domain::post::format::FormatMark;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Toggles `mark` on the selection of the last focused draft.
    pub(super) fn format_active(&mut self, mark: FormatMark, window: &mut Window, cx: &mut Context<Self>) {
        let index = self.active_draft.min(self.drafts.len().saturating_sub(1));
        if let Some(input) = self.drafts.get(index).map(|slot| slot.input.clone()) {
            apply_format(&input, mark, window, cx);
        }
    }
}
