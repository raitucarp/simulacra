//! The composer's lock state (X3): whether "Lock with password" is open and its two masked
//! fields. The password is read once, when the thread is submitted, into a zeroizing
//! [`LockPassword`], and both fields are emptied at once; it is never kept anywhere else.

use super::super::lock_picker::LockView;
use super::ComposerModalPanel;
use crate::domain::data::snapshot::strength;
use crate::domain::post::lock::{lock_problem, LockPassword, LockProblem};
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;
use zeroize::Zeroizing;

/// The lock row's state.
pub(super) struct LockPicker {
    pub open: bool,
    pub password: Entity<InputState>,
    pub repeat: Entity<InputState>,
    /// The submitted thread is being sealed; posting waits.
    pub sealing: bool,
}

fn masked(placeholder: String, window: &mut Window, cx: &mut Context<ComposerModalPanel>) -> Entity<InputState> {
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder).masked(true))
}

impl LockPicker {
    pub fn new(window: &mut Window, cx: &mut Context<ComposerModalPanel>) -> Self {
        let password = masked(t!("composer.lock.password_placeholder").to_string(), window, cx);
        let repeat = masked(t!("composer.lock.repeat_placeholder").to_string(), window, cx);
        Self { open: false, password, repeat, sealing: false }
    }

    fn typed(&self, cx: &App) -> (Zeroizing<String>, Zeroizing<String>) {
        (Zeroizing::new(self.password.read(cx).value().to_string()), Zeroizing::new(self.repeat.read(cx).value().to_string()))
    }
}

impl ComposerModalPanel {
    /// Opens the lock row (focusing the password) or closes it, forgetting the password.
    pub(super) fn toggle_lock(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.lock.open {
            self.reset_lock(window, cx);
        } else {
            self.lock.open = true;
            self.lock.password.update(cx, |state, cx| state.focus(window, cx));
        }
        cx.notify();
    }

    /// Opens the row for an edit of a locked post (X5): saving seals the new text again, and
    /// the footer offers no toggle while editing, so it stays open.
    pub fn require_lock(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.lock.open = true;
        self.lock.password.update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    /// Closes the row and empties both fields.
    pub(super) fn reset_lock(&mut self, window: &mut Window, cx: &mut App) {
        (self.lock.open, self.lock.sealing) = (false, false);
        for input in [&self.lock.password, &self.lock.repeat] {
            input.update(cx, |state, cx| state.set_value("", window, cx));
        }
    }

    /// Why the open row cannot lock the thread; `None` while it can (or is closed).
    pub(super) fn lock_problem(&self, cx: &App) -> Option<LockProblem> {
        let (password, repeat) = self.lock.typed(cx);
        self.lock.open.then(|| lock_problem(&password, &repeat, self.has_media(cx))).flatten()
    }

    /// Whether posting must wait for the lock row (a problem, or sealing under way).
    pub(super) fn lock_blocks(&self, cx: &App) -> bool {
        self.lock.sealing || self.lock_problem(cx).is_some()
    }

    /// The password to seal the thread with, read once: both fields are emptied and the
    /// row shows "sealing" until the host reports back. `None` while the row is closed.
    pub(super) fn take_lock_password(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<LockPassword> {
        if !self.lock.open {
            return None;
        }
        let (password, _) = self.lock.typed(cx);
        for input in [&self.lock.password, &self.lock.repeat] {
            input.update(cx, |state, cx| state.set_value("", window, cx));
        }
        self.lock.sealing = true;
        Some(LockPassword::new(password.as_str()))
    }

    /// The host finished sealing: on failure the row asks for the password again.
    pub fn finish_sealing(&mut self, cx: &mut Context<Self>) {
        self.lock.sealing = false;
        cx.notify();
    }

    /// The open row as the dialog renders it.
    pub(super) fn lock_view(&self, cx: &App) -> Option<LockView> {
        let problem = (!self.lock.sealing).then(|| self.lock_problem(cx)).flatten();
        self.lock.open.then(|| LockView { password: self.lock.password.clone(), repeat: self.lock.repeat.clone(), strength: strength(&self.lock.typed(cx).0), problem, sealing: self.lock.sealing })
    }
}
