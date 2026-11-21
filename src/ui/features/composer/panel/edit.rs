//! Edit mode (sb F2): the composer opened on one of the user's own posts, its text in the
//! single editor and its tint in the picker (sb C3). Submitting emits the usual
//! [`super::ComposerModalEvent::Submit`] in [`ComposerModalMode::Edit`]; the host saves the
//! new text, keeping the old one as a revision, and the tint. Attachments and added posts
//! are not offered while editing.

use super::super::modal::ComposerModalMode;
use super::ComposerModalPanel;
use crate::domain::post::Tint;
use crate::domain::AuthoredPost;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Opens the composer on `target`'s text and tint, the cursor after its last character.
    pub fn open_edit(&mut self, target: AuthoredPost, window: &mut Window, cx: &mut Context<Self>) {
        let (text, tint) = (target.post.content.clone(), target.post.extras.tint);
        let waiting = target.post.lifecycle.publish_at.filter(|_| target.post.lifecycle.is_scheduled());
        self.open(ComposerModalMode::Edit(Box::new(target)), window, cx);
        self.tint = tint;
        if let Some(at) = waiting {
            self.open_schedule_at(at, window, cx); // Y4: a waiting post keeps its moment
        }
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

    /// Picks the container tint (sb C3; `None` clears it).
    pub(super) fn pick_tint(&mut self, tint: Option<Tint>, cx: &mut Context<Self>) {
        self.tint = tint;
        cx.notify();
    }
}
