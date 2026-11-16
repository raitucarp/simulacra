//! Composer modal handlers: opening and closing the panel and publishing its drafts.

use super::super::view::AppView;
use crate::ui::features::composer::{
    ComposerModalEvent, ComposerModalMode, ComposerModalPanel, ComposerSubmission,
};
use gpui_kit::*;

impl AppView {
    /// Opens the Threads-style composer modal in the specified mode (NewThread, Reply, Quote).
    pub fn open_composer_modal(
        &mut self,
        mode: ComposerModalMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.composer_modal.update(cx, |panel, cx| panel.open(mode, window, cx));
        cx.notify();
    }

    /// Closes the composer modal and clears modal state.
    pub fn close_composer_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_modal.update(cx, |panel, cx| panel.close(window, cx));
        cx.notify();
    }

    /// Submits the composer modal content according to its mode (NewThread, Reply, Quote).
    pub fn submit_composer_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_modal.update(cx, |panel, cx| panel.submit(window, cx));
    }

    /// Publishes a submitted draft; on success focuses the new thread and closes the modal.
    pub(crate) fn publish_composer_draft(
        &mut self,
        _: &Entity<ComposerModalPanel>,
        event: &ComposerModalEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ComposerModalEvent::Submit(draft) = event;
        let ComposerSubmission { mode, content, link, image } = draft.clone();
        let result = match &mode {
            ComposerModalMode::NewThread => {
                self.state.submit_post_with_attachments(&content, link, image, None, None)
            }
            ComposerModalMode::Reply(parent) => {
                self.state.submit_reply(&parent.id, &content, link, image)
            }
            ComposerModalMode::Quote(quoted) => {
                self.state.submit_quote(&quoted.id, &content, link, image)
            }
        };
        if let Ok(post_id) = result {
            if !matches!(mode, ComposerModalMode::Reply(_)) {
                self.feed.selected_thread_post_id = Some(post_id);
            }
            self.close_composer_modal(window, cx);
        }
    }
}
