//! Unlocking posts (X4): the sheet reads the password once into a zeroizing [`LockPassword`],
//! empties its field, and opens the post's lock in the background (the key derivation takes
//! a moment). The opened text goes to the session cache only: never to the store, the vault,
//! an index or an agent. Everything locks again after a while without activity.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::post::lock::{open_text, LockPassword, UnlockError};
use crate::domain::PostId;
use crate::ui::features::data::{render_unlock_sheet, tr, UnlockAction, UnlockDialog, UnlockEvent};
use crate::ui::features::post::LockSource;
use gpui_kit::*;
use std::rc::Rc;
use std::time::Instant;

impl AppView {
    /// Routes one unlock event.
    pub(crate) fn on_unlock_event(&mut self, event: UnlockEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            UnlockEvent::Open(id) => self.open_unlock(id, window, cx),
            UnlockEvent::Submit => self.submit_unlock(window, cx),
            UnlockEvent::Close => self.close_unlock(window, cx),
            UnlockEvent::Relock(id) => {
                self.data.locks.unlocked.relock(&id);
                self.feed.stream.remeasure();
            }
        }
        cx.notify();
    }

    fn open_unlock(&mut self, id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        if self.data.locks.unlocked.is_unlocked(&id) {
            return;
        }
        let password = self.data.locks.password.clone();
        password.update(cx, |input, cx| input.set_value("", window, cx));
        self.data.locks.dialog = Some(UnlockDialog::new(id, password.clone()));
        password.update(cx, |input, cx| input.focus(window, cx));
    }

    fn close_unlock(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.data.locks.password.update(cx, |input, cx| input.set_value("", window, cx));
        self.data.locks.dialog = None;
        window.focus(&self.palette.shell_focus, cx);
    }

    /// Reads the password once, empties the field and opens the lock off the UI thread.
    fn submit_unlock(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.locks.dialog.as_mut().filter(|dialog| !dialog.busy) else { return };
        let id = dialog.post_id.clone();
        let Some(lock) = self.state.get_post(&id).or_else(|| self.state.scheduled_post(&id)).and_then(|post| post.extras.lock.clone()) else { return };
        let password = LockPassword::new(dialog.password.read(cx).value().to_string());
        dialog.password.update(cx, |input, cx| input.set_value("", window, cx));
        (dialog.busy, dialog.problem) = (true, None);
        cx.spawn_in(window, async move |view, cx| {
            let opened = cx.background_executor().spawn(async move { open_text(&lock, &password) }).await;
            let _ = view.update_in(cx, |this, window, cx| this.unlock_finished(id, opened, window, cx));
        })
        .detach();
    }

    fn unlock_finished(&mut self, id: PostId, opened: Result<zeroize::Zeroizing<String>, UnlockError>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.locks.dialog.as_mut().filter(|dialog| dialog.post_id == id) else { return };
        match opened {
            Ok(text) => {
                self.data.locks.unlocked.insert(id, text, Instant::now());
                self.close_unlock(window, cx);
                self.feed.stream.remeasure();
            }
            Err(error) => {
                dialog.busy = false;
                dialog.problem = Some(tr(unlock_problem_key(error)));
            }
        }
        cx.notify();
    }

    /// `root` noting every key press and mouse move as activity: unlocked posts stay open a
    /// while longer.
    pub(in crate::ui::features::shell) fn with_activity_listeners(&self, root: Div, cx: &mut Context<Self>) -> Div {
        root.capture_key_down(cx.listener(|this, _: &KeyDownEvent, _, _| this.data.locks.unlocked.touch(Instant::now())))
            .on_mouse_move(cx.listener(|this, _: &MouseMoveEvent, _, _| this.data.locks.unlocked.touch(Instant::now())))
    }

    /// Locks every post again once the user has been idle long enough (heartbeat).
    pub(in crate::ui::features::shell) fn relock_idle_posts(&mut self, cx: &mut Context<Self>) {
        if self.data.locks.unlocked.relock_if_idle(Instant::now()) {
            self.feed.stream.remeasure();
            cx.notify();
        }
    }

    pub(in crate::ui::features::shell) fn unlock_action(&self, bind: &Binder) -> UnlockAction {
        Rc::new(bind.with(Self::on_unlock_event))
    }

    /// What every locked card needs: its opened text, while unlocked, and the actions.
    pub(in crate::ui::features::shell) fn lock_source(&self, bind: &Binder) -> LockSource {
        LockSource { texts: Rc::new(self.data.locks.unlocked.texts()), on_event: self.unlock_action(bind) }
    }

    /// The unlock sheet, while open.
    pub(in crate::ui::features::shell) fn unlock_overlay(&self, bind: &Binder) -> Option<AnyElement> {
        let dialog = self.data.locks.dialog.as_ref()?;
        Some(render_unlock_sheet(dialog, self.unlock_action(bind), &self.theme).into_any_element())
    }
}

/// Locale key of why an unlock failed.
pub(super) fn unlock_problem_key(error: UnlockError) -> &'static str {
    match error {
        UnlockError::WrongPassword => "posts.lock.wrong_password",
        UnlockError::Damaged => "posts.lock.damaged",
    }
}
