//! Composer modal handlers: opening and closing the panel and publishing its thread.

use super::super::view::AppView;
use crate::state::{ThreadAnchor, ThreadDraft};
use crate::ui::features::composer::{
    publish_error_message, ComposerModalEvent, ComposerModalMode, ComposerModalPanel,
    ComposerSubmission,
};
use crate::domain::post::schedule::format_moment;
use crate::domain::preferences::ToastKind;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use jiff::tz::TimeZone;
use jiff::Timestamp;
use rust_i18n::t;

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
        if let ComposerModalMode::Edit(target) = &submission.mode {
            let text = submission.drafts.first().cloned().unwrap_or_default();
            if target.post.lifecycle.is_scheduled() {
                return self.save_scheduled_edit(target.post.id.clone(), (text, submission.tint), submission.publish_at, window, cx); // Y4
            }
            return self.save_post_edit(target.post.id.clone(), submission.edit.clone().unwrap_or_else(|| crate::state::PostEdit::text(text)), (submission.tint, submission.symbols.clone()), window, cx); // sb F2, C3, S5, V3
        }
        let publish_at = submission.publish_at;
        let (thread, anchor) = thread_from_submission(submission.clone(), self.state.preferences().writing.symbol_shorthand);
        let published = match publish_at {
            Some(at) => self.state.schedule_thread(&thread, &anchor, at, Timestamp::now()).map(|_| None),
            None => self.state.submit_thread(&thread, &anchor).map(|ids| ids.last().cloned()),
        };
        match published {
            Ok(last) => {
                self.remember_symbols(&submission.symbols, cx); // S5
                self.close_composer_modal(window, cx);
                match (last, publish_at) {
                    (Some(last), _) => self.reveal_post(last, window, cx),
                    (None, Some(at)) => self.toast_scheduled(at, window, cx),
                    (None, None) => {}
                }
            }
            Err(err) => {
                let message = publish_error_message(&err);
                self.composer_modal.update(cx, |panel, cx| panel.set_error(message, cx));
            }
        }
    }

    /// Says when a scheduled thread goes out (Y3); it stays out of the feed until then.
    fn toast_scheduled(&mut self, at: Timestamp, window: &mut Window, cx: &mut Context<Self>) {
        let when = format_moment(at, &TimeZone::system());
        let text = t!("composer.schedule.done", when = when).to_string();
        self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
    }
}

/// Store drafts (attachments and symbols on the first post, the tint on every post) and the anchor
/// implied by the modal mode; with `shorthand`, each post's leading markers become symbols.
fn thread_from_submission(submission: ComposerSubmission, shorthand: bool) -> (Vec<ThreadDraft>, ThreadAnchor) {
    let ComposerSubmission { mode, drafts, link, images, tint, publish_at: _, symbols, edit: _ } = submission;
    let mut thread: Vec<ThreadDraft> = drafts.into_iter().map(|text| ThreadDraft { tint, ..ThreadDraft::text(text) }).collect();
    if let Some(first) = thread.first_mut() {
        first.link = link;
        first.image = images.first().map(|image| image.path.clone());
        first.images = images;
        first.symbols = symbols; // S5: the picked symbols go on the first post
    }
    if shorthand {
        thread = thread.into_iter().map(ThreadDraft::with_shorthand).collect(); // S6: markers at the start of each post
    }
    let anchor = match mode {
        ComposerModalMode::NewThread => ThreadAnchor::default(),
        ComposerModalMode::Reply(parent) => ThreadAnchor { reply_to: Some(parent.post.id), quote_of: None },
        ComposerModalMode::Quote(quoted) => ThreadAnchor { reply_to: None, quote_of: Some(quoted.post.id) },
        ComposerModalMode::Edit(_) => ThreadAnchor::default(),
    };
    (thread, anchor)
}
