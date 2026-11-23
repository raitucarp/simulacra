//! The Posts tab of a profile (W7): its view, the feelings filter, paging and the calendar
//! month, and opening a post from it (a scheduled one in the scheduled editor).

use super::super::view::AppView;
use crate::domain::PostId;
use crate::ui::features::roster::{ProfilePosts, ProfilePostsEvent};
use gpui_kit::*;

impl AppView {
    /// Runs one event of the Posts tab.
    pub(crate) fn profile_posts_event(&mut self, event: ProfilePostsEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.lab.posts = self.lab.posts.after(event);
        cx.notify();
    }

    /// Opens `id` from a profile's views: a post waiting to be published in its editor, any
    /// other in the timeline.
    pub(crate) fn open_profile_post(&mut self, id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.scheduled_posts().iter().any(|post| post.id == id) {
            return self.open_scheduled_editor(&id, window, cx);
        }
        self.reveal_post(id, window, cx);
    }

    /// A newly shown profile starts on its first page, every post, its newest month.
    pub(in crate::ui::features::shell) fn reset_profile_posts(&mut self) {
        self.lab.posts = ProfilePosts { view: self.lab.posts.view, ..ProfilePosts::default() };
    }
}
