//! A post's entry moves in the shell (S7): the "Entry" submenu of the user's own task or event
//! moves it to another state, a patch version of the post.

use super::super::view::AppView;
use crate::domain::preferences::ToastKind;
use crate::domain::symbols::EntryState;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::symbols::state_label;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use jiff::Timestamp;
use rust_i18n::t;

impl AppView {
    /// Moves the entry of `id` to `to`, counting the new version on the card.
    pub(in crate::ui::features::shell) fn set_entry_state(&mut self, id: &PostId, to: EntryState, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.set_entry_state(id, to, Timestamp::now()) {
            Ok(_) => {
                *self.data.revision_counts.entry(id.as_str().to_string()).or_default() += 1;
                let text = t!("symbols.state_saved", state = state_label(to)).to_string();
                self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
            }
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
        cx.notify();
    }
}
