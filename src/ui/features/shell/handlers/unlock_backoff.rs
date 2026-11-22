//! Wrong-password backoff on the unlock sheet (X7): while a post's attempts have to wait,
//! the sheet says for how long and its Unlock button is off; a timer brings it back.

use super::super::view::AppView;
use crate::domain::PostId;
use gpui_kit::*;
use rust_i18n::t;
use std::time::{Duration, Instant};

/// "Try again in N s" for `wait`, rounded up to whole seconds.
pub(super) fn wait_text(wait: Duration) -> String {
    let seconds = wait.as_millis().div_ceil(1000).max(1);
    t!("posts.lock.wait", seconds = seconds).to_string()
}

impl AppView {
    /// How long an attempt on `id` still has to wait, if at all.
    pub(super) fn unlock_wait(&self, id: &PostId) -> Option<Duration> {
        self.data.locks.attempts.wait(id, Instant::now())
    }

    /// Notes a wrong password for `id`: the sheet shows the wait it now has, if any.
    pub(super) fn note_wrong_password(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let wait = self.data.locks.attempts.failed(id, Instant::now());
        self.show_unlock_wait(wait, window, cx);
    }

    /// Shows `wait` on the open sheet and refreshes it once the wait is over.
    pub(super) fn show_unlock_wait(&mut self, wait: Option<Duration>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.locks.dialog.as_mut() else { return };
        dialog.wait = wait.map(wait_text);
        let Some(wait) = wait else { return };
        cx.spawn_in(window, async move |view, cx| {
            cx.background_executor().timer(wait).await;
            let _ = view.update(cx, |this, cx| this.refresh_unlock_wait(cx));
        })
        .detach();
    }

    fn refresh_unlock_wait(&mut self, cx: &mut Context<Self>) {
        let now = Instant::now();
        if let Some(dialog) = self.data.locks.dialog.as_mut() {
            dialog.wait = self.data.locks.attempts.wait(&dialog.post_id, now).map(wait_text);
        }
        cx.notify();
    }
}
