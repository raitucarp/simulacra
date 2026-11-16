//! The single constructor for agent reply posts.

use crate::domain::{NetworkId, Post, PostId, Profile};
use chrono::Utc;
use uuid::Uuid;

/// Prefix of every generated post identifier (`post_<uuid>`).
const POST_ID_PREFIX: &str = "post_";

/// Builds a fresh reply `Post` authored by `agent_profile`, threaded under and
/// linked to `parent_post`, mentioning the parent's author.
pub(super) fn build_reply(
    parent_post: &Post,
    agent_profile: Profile,
    network_id: NetworkId,
    content: String,
) -> Post {
    let new_id = PostId::new(format!("{POST_ID_PREFIX}{}", Uuid::new_v4().simple()));

    Post {
        id: new_id,
        network_id,
        author: agent_profile,
        content,
        created_at: Utc::now(),
        likes: 0,
        is_liked: false,
        replies_count: 0,
        mentions: vec![parent_post.author.handle.clone()],
        linked_posts: vec![parent_post.id.clone()],
        reply_to_id: Some(parent_post.id.clone()),
        quote_id: None,
        repost_id: None,
        link_url: None,
        image_path: None,
        reposts_count: 0,
        is_reposted: false,
    }
}
