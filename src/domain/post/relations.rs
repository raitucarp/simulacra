//! Convenience constructors for reposts, threadline replies, and quotes.

use super::Post;
use crate::domain::ids::{NetworkId, PostId};
use crate::domain::profile::Profile;
use crate::error::Result;

/// A repost carries no text of its own.
const EMPTY_REPOST_CONTENT: &str = "";

impl Post {
    /// Convenience constructor for an instant repost thought.
    pub fn create_repost(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: Profile,
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
        author: Profile,
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
        author: Profile,
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
