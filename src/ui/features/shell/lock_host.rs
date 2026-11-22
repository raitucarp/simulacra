//! Locked posts in the shell (X4): the session's unlocked texts (memory only), the open
//! unlock sheet and its masked field (Enter unlocks).

use super::view::AppView;
use crate::state::unlocked::UnlockedPosts;
use crate::ui::features::data::{UnlockDialog, UnlockEvent};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;

/// What the shell keeps about locked posts.
pub struct LockHost {
    pub unlocked: UnlockedPosts,
    pub dialog: Option<UnlockDialog>,
    pub password: Entity<InputState>,
    _enter: Subscription,
}

impl LockHost {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> Self {
        let placeholder = t!("posts.lock.password_placeholder").to_string();
        let password = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder).masked(true));
        let enter = cx.subscribe_in(&password, window, |this: &mut AppView, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                this.on_unlock_event(UnlockEvent::Submit, window, cx);
            }
        });
        Self { unlocked: UnlockedPosts::default(), dialog: None, password, _enter: enter }
    }
}
