//! The constructors for agent posts: replies, new top-level posts and chained follow-ups.

use crate::domain::{AuthoredPost, NetworkId, Post, PostId, Profile, ProfileId};
use jiff::Timestamp;
use uuid::Uuid;

/// Prefix of every generated post identifier (`post_<uuid>`).
const POST_ID_PREFIX: &str = "post_";
/// Spacing between chained follow-ups so they keep their order.
const FOLLOWUP_SPACING_MS: i64 = 1;

fn new_post_id() -> PostId {
    PostId::new(format!("{POST_ID_PREFIX}{}", Uuid::new_v4().simple()))
}

/// Builds a fresh reply `Post` authored by `agent_profile`, threaded under and
/// linked to `parent_post`, mentioning the parent's author.
pub(super) fn build_reply(
    parent_post: &AuthoredPost,
    agent_profile: Profile,
    network_id: NetworkId,
    content: String,
) -> Post {
    Post {
        mentions: vec![parent_post.author.handle.clone()],
        linked_posts: vec![parent_post.id.clone()],
        reply_to_id: Some(parent_post.id.clone()),
        ..build_root_post(agent_profile, network_id, content)
    }
}

/// Builds a fresh top-level `Post` authored by `agent_profile`: it replies to, links and
/// mentions no one (its thread parts are chained under it with [`build_followup`]).
pub(super) fn build_root_post(agent_profile: Profile, network_id: NetworkId, content: String) -> Post {
    Post {
        id: new_post_id(),
        network_id,
        author_id: agent_profile.id,
        content,
        created_at: Timestamp::now(),
        likes: 0,
        is_liked: false,
        replies_count: 0,
        mentions: Vec::new(),
        linked_posts: Vec::new(),
        reply_to_id: None,
        quote_id: None,
        repost_id: None,
        link_url: None,
        image_path: None,
        reposts_count: 0,
        is_reposted: false,
        extras: Default::default(),
        kind: Default::default(),
        lifecycle: Default::default(),
    }
}

/// A follow-up by the same author chained under `previous` (a self-reply thread part).
pub(super) fn build_followup(previous: &Post, author_id: ProfileId, content: String) -> Post {
    Post {
        id: new_post_id(),
        author_id,
        content,
        created_at: previous.created_at + jiff::SignedDuration::from_millis(FOLLOWUP_SPACING_MS),
        replies_count: 0,
        mentions: Vec::new(),
        linked_posts: vec![previous.id.clone()],
        reply_to_id: Some(previous.id.clone()),
        image_path: None,
        extras: Default::default(),
        ..previous.clone()
    }
}
