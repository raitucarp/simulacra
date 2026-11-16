//! Post overflow-menu handlers: deleting one of the user's own thoughts.

use super::super::view::AppView;
use crate::domain::PostId;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Deletes `post_id` (confirmed by the card's dialog), clearing it from the inspector.
    pub(crate) fn delete_post(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        let is_own = self.state.get_post(&post_id).is_some_and(|p| self.state.author_of(p).is_human_persona());
        if !is_own {
            return;
        }
        let note = match self.state.delete_post(&post_id) {
            Ok(()) => Notification::success(t!("post.deleted").to_string()),
            Err(err) => Notification::error(t!("post.delete_failed", reason = err.to_string()).to_string()),
        };
        if self.feed.selected_thread_post_id.as_ref() == Some(&post_id) {
            self.feed.selected_thread_post_id = None;
        }
        window.push_notification(note, cx);
        cx.notify();
    }
}
