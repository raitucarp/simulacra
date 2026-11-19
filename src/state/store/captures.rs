//! Quick capture: dumped text, links and images become posts by the active profile, marked
//! as captures so the Inbox lists them.

use super::AppState;
use crate::domain::post::PostOrigin;
use crate::domain::Post;
use crate::error::Result;
use crate::import::capture::{classify, Capture};
use crate::import::capture_posts::{capture_posts, CaptureDraft};
use crate::import::plan::effective_post_limit;

impl AppState {
    /// Saves `text` (and an optional stored image) to the Inbox; returns how many posts it took.
    pub fn capture(&mut self, text: &str, image_path: Option<String>) -> Result<usize> {
        let author = self.current_profile_or_err()?.id;
        let draft = CaptureDraft { capture: classify(text), image_path };
        let limit = effective_post_limit(self.preferences().writing.post_length_limit);
        let posts = capture_posts(&draft, limit, &author, &self.active_network_id, chrono::Utc::now(), Self::fresh_post_id)?;
        self.publish_batch(posts)
    }

    /// Whether `post` was captured (an Inbox post).
    pub fn is_capture(post: &Post) -> bool {
        post.extras.origin == Some(PostOrigin::Capture)
    }

    /// Captured posts of the active network.
    pub fn inbox_count(&self) -> usize {
        self.current_network_posts().filter(|post| Self::is_capture(post)).count()
    }

    /// Whether `text` would make a capture at all.
    pub fn is_capturable(text: &str) -> bool {
        classify(text) != Capture::Empty
    }
}
