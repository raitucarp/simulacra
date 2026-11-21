//! The preview toggle: every draft shown as it will read once posted, through the same
//! body renderer as the feed, and back to the editors.

use super::ComposerModalPanel;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Switches between the editors and the preview; back in the editors, the last focused
    /// draft gets the focus again.
    pub(super) fn toggle_preview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.preview = !self.preview;
        if !self.preview {
            self.focus_draft(self.active_draft.min(self.drafts.len().saturating_sub(1)), window, cx);
        }
        cx.notify();
    }

    /// Whether the drafts are shown as a preview.
    pub fn is_previewing(&self) -> bool {
        self.preview
    }
}
