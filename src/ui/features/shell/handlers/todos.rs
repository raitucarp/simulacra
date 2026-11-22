//! To-dos in the shell (D6): clicking a box of one of the user's own posts ticks it, kept
//! with the post without a new version.

use super::super::view::AppView;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use gpui_kit::*;

impl AppView {
    /// Ticks (or unticks) the `index`-th to-do of `id`, saying why when it cannot.
    pub(in crate::ui::features::shell) fn toggle_todo(&mut self, id: &PostId, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if let Err(err) = self.state.toggle_todo(id, index) {
            self.notify_data(publish_error_message(&err), window, cx);
        }
        cx.notify();
    }
}
