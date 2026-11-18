//! Edit mode (sb F2): the composer opened on one of the user's own posts, its text in the
//! single editor. Submitting emits the usual [`super::ComposerModalEvent::Submit`] in
//! [`ComposerModalMode::Edit`]; the host saves the new text and keeps the old one as a
//! revision. Attachments and added posts are not offered while editing.

use super::super::modal::ComposerModalMode;
use super::ComposerModalPanel;
use crate::domain::AuthoredPost;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Opens the composer on `target`'s text, the cursor after its last character.
    pub fn open_edit(&mut self, target: AuthoredPost, window: &mut Window, cx: &mut Context<Self>) {
        let text = target.post.content.clone();
        self.open(ComposerModalMode::Edit(Box::new(target)), window, cx);
        if let Some(input) = self.drafts.first().map(|slot| slot.input.clone()) {
            input.update(cx, |state, cx| {
                state.insert(text, window, cx);
                state.focus(window, cx);
            });
        }
        cx.notify();
    }

    /// Whether the open composer edits a post.
    pub fn is_editing(&self) -> bool {
        matches!(self.mode, ComposerModalMode::Edit(_))
    }
}
