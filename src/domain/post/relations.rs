//! Convenience constructors for reposts, threadline replies, and quotes.

use super::Post;
use crate::domain::ids::{NetworkId, PostId, ProfileId};
use crate::error::Result;

/// A repost carries no text of its own.
const EMPTY_REPOST_CONTENT: &str = "";

impl Post {
    /// Convenience constructor for an instant repost thought.
    pub fn create_repost(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: impl Into<ProfileId>,
        target_post_id: impl Into<PostId>,
    ) -> Result<Self> {
        Self::create_with_attachments(
            id,
            network_id,
            author,
            EMPTY_REPOST_CONTENT,
            None,
            None,
            None,
            None,
            Some(target_post_id.into()),
        )
    }

    /// Convenience constructor for a threadline reply thought.
    pub fn create_reply(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: impl Into<ProfileId>,
        raw_content: &str,
        parent_id: impl Into<PostId>,
        link_url: Option<String>,
        image_path: Option<String>,
    ) -> Result<Self> {
        Self::create_with_attachments(
            id,
            network_id,
            author,
            raw_content,
            link_url,
            image_path,
            Some(parent_id.into()),
            None,
            None,
        )
    }

    /// Convenience constructor for a quoted thought.
    pub fn create_quote(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: impl Into<ProfileId>,
        raw_content: &str,
        quoted_id: impl Into<PostId>,
        link_url: Option<String>,
        image_path: Option<String>,
    ) -> Result<Self> {
        Self::create_with_attachments(
            id,
            network_id,
            author,
            raw_content,
            link_url,
            image_path,
            None,
            Some(quoted_id.into()),
            None,
        )
    }
}

impl Post {
    /// Whether this thought is a content-less instant repost (a quote keeps its own identity).
    #[inline]
    pub fn is_instant_repost(&self) -> bool {
        self.repost_id.is_some() && self.quote_id.is_none() && self.content.trim().is_empty()
    }

    /// The thought that likes, replies, quotes and reposts of this card act on: the original
    /// for an instant repost, this thought otherwise.
    #[inline]
    pub fn action_target_id(&self) -> &PostId {
        match (&self.repost_id, self.is_instant_repost()) {
            (Some(original), true) => original,
            _ => &self.id,
        }
    }
}
