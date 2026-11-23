//! Data derived once from a post body (on insert / load) instead of on every render.

use super::AppState;
use crate::domain::post::refs::{outgoing_refs, PostRef};
use crate::domain::post::{tokenize_post_body, BodyToken};
use crate::domain::Post;
use std::rc::Rc;

/// Render and search data derived from one post body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostMeta {
    /// Inline body tokens (text, wikilinks, hashtags), shared with every render.
    pub tokens: Rc<[BodyToken]>,
    /// Lower-cased body, matched by timeline search.
    pub search_text: String,
    /// Short preview shown by wikilink pills pointing at this post.
    pub excerpt: String,
    /// N1: the posts it embeds, quotes and links to (none read from a locked post's text).
    pub refs: Vec<PostRef>,
}

impl PostMeta {
    /// Tokenizes and normalizes `post` once.
    pub fn of(post: &Post) -> Self {
        let tokens: Rc<[BodyToken]> = tokenize_post_body(&post.content).into();
        let refs = outgoing_refs(post, &tokens);
        Self { tokens, search_text: post.content.to_lowercase(), excerpt: AppState::compute_wikilink_excerpt(post), refs }
    }
}
