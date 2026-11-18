//! Composer modal handlers: opening and closing the panel and publishing its thread.

use super::super::view::AppView;
use crate::state::{ThreadAnchor, ThreadDraft};
use crate::ui::features::composer::{
    publish_error_message, ComposerModalEvent, ComposerModalMode, ComposerModalPanel,
    ComposerSubmission,
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

    /// Publishes a submitted thread; on success closes the modal and reveals the last post,
    /// otherwise shows the store's rejection inside the composer.
    pub(crate) fn publish_composer_draft(
        &mut self,
        _: &Entity<ComposerModalPanel>,
        event: &ComposerModalEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let submission = match event {
            ComposerModalEvent::Submit(submission) => submission,
            ComposerModalEvent::ImageIdeas(text) => return self.open_image_ideas_for_draft(text.clone(), window, cx),
        };
        let (thread, anchor) = thread_from_submission(submission.clone());
        match self.state.submit_thread(&thread, &anchor) {
            Ok(ids) => {
                self.close_composer_modal(window, cx);
                if let Some(last) = ids.last().cloned() {
                    self.reveal_post(last, window, cx);
                }
            }
            Err(err) => {
                let message = publish_error_message(&err);
                self.composer_modal.update(cx, |panel, cx| panel.set_error(message, cx));
            }
        }
    }
}

/// Store drafts (attachments on the first post) and the anchor implied by the modal mode.
fn thread_from_submission(submission: ComposerSubmission) -> (Vec<ThreadDraft>, ThreadAnchor) {
    let ComposerSubmission { mode, drafts, link, images } = submission;
    let mut thread: Vec<ThreadDraft> = drafts.into_iter().map(ThreadDraft::text).collect();
    if let Some(first) = thread.first_mut() {
        first.link = link;
        first.image = images.first().map(|image| image.path.clone());
        first.images = images;
    }
    let anchor = match mode {
        ComposerModalMode::NewThread => ThreadAnchor::default(),
        ComposerModalMode::Reply(parent) => ThreadAnchor { reply_to: Some(parent.post.id), quote_of: None },
        ComposerModalMode::Quote(quoted) => ThreadAnchor { reply_to: None, quote_of: Some(quoted.post.id) },
    };
    (thread, anchor)
}
