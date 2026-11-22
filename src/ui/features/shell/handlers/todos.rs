//! To-dos in the shell (D6): clicking a box of one of the user's own posts ticks it, kept
//! with the post without a new version; completing a repeating item says when it comes
//! round next (D8).

use super::super::view::AppView;
use crate::domain::preferences::ToastKind;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::post::todo_due_label;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Ticks (or unticks) the `index`-th to-do of `id`, saying why when it cannot.
    pub(in crate::ui::features::shell) fn toggle_todo(&mut self, id: &PostId, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let rounds = |this: &Self| this.state.post_todos(id).get(index).map(|entry| entry.completions);
        let before = rounds(self);
        match self.state.toggle_todo(id, index) {
            Ok(_) if rounds(self) > before => self.say_next_round(id, index, window, cx),
            Ok(_) => {}
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
        cx.notify();
    }

    /// "Next: Thursday, 2026-11-26" once a repeating item came round again.
    fn say_next_round(&mut self, id: &PostId, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(id) else { return };
        let written = crate::time::local_date(post.created_at);
        let Some(due) = self.state.post_todos(id).get(index).and_then(|entry| entry.due(written)) else { return };
        let text = t!("todo.repeat.next", when = todo_due_label(due, crate::time::local_now().date())).to_string();
        self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
    }
}
