//! Publishing a locked thread from the composer (X3): the drafts are validated on the UI
//! thread, every post is sealed in the background (the key derivation takes a moment), and
//! the sealed posts are stored back on the UI thread. The password moves into the background
//! task and is zeroized when it ends; nothing else keeps it.

use super::super::view::AppView;
use super::composer_modal::thread_from_submission;
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::{Post, PostId};
use crate::error::Result;
use crate::state::store::seal_thread;
use crate::ui::features::composer::ComposerSubmission;
use gpui_kit::*;
use jiff::Timestamp;

impl AppView {
    /// Seals and publishes (or schedules) `submission` under `password`.
    pub(super) fn publish_locked_submission(&mut self, submission: &ComposerSubmission, password: LockPassword, window: &mut Window, cx: &mut Context<Self>) {
        let (thread, anchor) = thread_from_submission(submission.clone(), self.state.preferences().writing.symbol_shorthand);
        let built = match self.state.build_locked_thread(&thread, &anchor) {
            Ok(posts) => posts,
            Err(err) => return self.after_publish(Err(err), submission, window, cx),
        };
        let submission = submission.clone();
        cx.spawn_in(window, async move |view, cx| {
            let sealed = cx.background_executor().spawn(async move { seal_thread(built, &password, KdfParams::DEFAULT) }).await;
            let _ = view.update_in(cx, |this, window, cx| this.store_sealed(sealed, &submission, window, cx));
        })
        .detach();
    }

    /// Stores the sealed thread and reports like any publish.
    fn store_sealed(&mut self, sealed: Result<Vec<Post>>, submission: &ComposerSubmission, window: &mut Window, cx: &mut Context<Self>) {
        let published = sealed.and_then(|posts| self.state.publish_locked(posts, submission.publish_at, Timestamp::now()));
        let last: Result<Option<PostId>> = published.map(|ids| if submission.publish_at.is_some() { None } else { ids.last().cloned() });
        self.composer_modal.update(cx, |panel, cx| panel.finish_sealing(cx));
        self.after_publish(last, submission, window, cx);
    }
}
