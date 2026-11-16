//! Resolving which thought a card action (like, reply, quote, repost, open) acts on.

use super::AppState;
use crate::domain::PostId;

impl AppState {
    /// The thought an action on `post_id` targets: the original of an instant repost (when it
    /// is still loaded), `post_id` itself otherwise. Quotes keep their own identity.
    pub fn action_target(&self, post_id: &PostId) -> PostId {
        let Some(post) = self.get_post(post_id) else {
            return post_id.clone();
        };
        let target = post.action_target_id();
        if target != post_id && self.get_post(target).is_none() {
            return post_id.clone();
        }
        target.clone()
    }
}
