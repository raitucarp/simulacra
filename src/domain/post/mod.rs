//! Thought post entity, parsing logic, and character limit rules.

mod constructors;
mod parse;
mod relations;

pub use parse::HASHTAG_SIGIL;

use super::ids::{NetworkId, PostId};
use super::profile::Profile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Maximum allowed character limit per post, enforcing micro-blog discipline.
pub const MAX_POST_CHARS: usize = 280;

/// Atomic thought unit in the second-brain feed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub network_id: NetworkId,
    pub author: Profile,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub likes: usize,
    pub is_liked: bool,
    pub replies_count: usize,
    pub mentions: Vec<String>,
    pub linked_posts: Vec<PostId>,
    #[serde(default)]
    pub reply_to_id: Option<PostId>,
    #[serde(default)]
    pub quote_id: Option<PostId>,
    #[serde(default)]
    pub repost_id: Option<PostId>,
    #[serde(default)]
    pub link_url: Option<String>,
    #[serde(default)]
    pub image_path: Option<String>,
    #[serde(default)]
    pub reposts_count: usize,
    #[serde(default)]
    pub is_reposted: bool,
}

impl Post {
    /// Toggles the like state of the post and increments/decrements the counter.
    pub fn toggle_like(&mut self) {
        self.is_liked = !self.is_liked;
        if self.is_liked {
            self.likes += 1;
        } else if self.likes > 0 {
            self.likes -= 1;
        }
    }

    /// Toggles the repost state of the post and increments/decrements the counter.
    pub fn toggle_repost(&mut self) {
        self.is_reposted = !self.is_reposted;
        if self.is_reposted {
            self.reposts_count += 1;
        } else if self.reposts_count > 0 {
            self.reposts_count -= 1;
        }
    }

    /// Whether this thought is an instant repost of another thought.
    #[inline]
    pub fn is_repost(&self) -> bool {
        self.repost_id.is_some()
    }

    /// Whether this thought is a direct threadline reply to a parent thought.
    #[inline]
    pub fn is_reply(&self) -> bool {
        self.reply_to_id.is_some()
    }

    /// Whether this thought quotes and embeds another thought.
    #[inline]
    pub fn is_quote(&self) -> bool {
        self.quote_id.is_some()
    }

    /// Whether this thought has any media attachment (link or local image).
    #[inline]
    pub fn has_attachments(&self) -> bool {
        self.link_url.is_some() || self.image_path.is_some()
    }
}

#[cfg(test)]
mod tests;
