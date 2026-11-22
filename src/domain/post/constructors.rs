//! Validated construction of thought posts.

use super::{visible_text_len, Post, MAX_POST_CHARS};
use crate::domain::ids::{NetworkId, PostId, ProfileId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl Post {
    /// Backwards-compatible constructor for standard thought posts.
    pub fn create(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: impl Into<ProfileId>,
        raw_content: &str,
    ) -> Result<Self> {
        Self::create_with_attachments(
            id,
            network_id,
            author,
            raw_content,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Instantiates a post with optional media attachments and dialectic relationships.
    /// Allows empty text when `repost_id.is_some()` or when attachments are present.
    #[allow(clippy::too_many_arguments)]
    pub fn create_with_attachments(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: impl Into<ProfileId>,
        raw_content: &str,
        link_url: Option<String>,
        image_path: Option<String>,
        reply_to_id: Option<PostId>,
        quote_id: Option<PostId>,
        repost_id: Option<PostId>,
    ) -> Result<Self> {
        Self::create_with_files(id, network_id, author, raw_content, link_url, image_path, reply_to_id, quote_id, repost_id, false)
    }

    /// [`Self::create_with_attachments`] for a post that will carry local files (L12) when
    /// `has_files`: they count as media, so the post may have no text.
    #[allow(clippy::too_many_arguments)]
    pub fn create_with_files(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: impl Into<ProfileId>,
        raw_content: &str,
        link_url: Option<String>,
        image_path: Option<String>,
        reply_to_id: Option<PostId>,
        quote_id: Option<PostId>,
        repost_id: Option<PostId>,
        has_files: bool,
    ) -> Result<Self> {
        let trimmed = raw_content.trim();
        let clean_link = clean_attachment(link_url);
        let clean_image = clean_attachment(image_path);
        let has_media = clean_link.is_some() || clean_image.is_some() || has_files;
        validate_content(trimmed, has_media, repost_id.is_some())?;

        let linked_posts = collect_linked_posts(trimmed, [&quote_id, &reply_to_id, &repost_id]);
        let mentions = Self::parse_mentions(trimmed);

        Ok(Self {
            id: id.into(),
            network_id: network_id.into(),
            author_id: author.into(),
            content: trimmed.to_string(),
            created_at: Timestamp::now(),
            likes: 0, is_liked: false, replies_count: 0,
            mentions,
            linked_posts,
            reply_to_id,
            quote_id,
            repost_id,
            link_url: clean_link,
            image_path: clean_image,
            reposts_count: 0, is_reposted: false,
            extras: Default::default(), kind: Default::default(), lifecycle: Default::default(),
        })
    }
}

/// Body wikilinks followed by any referenced (quote, reply, repost) IDs not already linked.
fn collect_linked_posts(body: &str, referenced: [&Option<PostId>; 3]) -> Vec<PostId> {
    let mut linked_posts = Post::parse_linked_posts(body);
    for id in referenced.into_iter().flatten() {
        if !linked_posts.contains(id) {
            linked_posts.push(id.clone());
        }
    }
    linked_posts
}

/// Trims an optional attachment value, discarding it when blank.
fn clean_attachment(value: Option<String>) -> Option<String> {
    value.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// Rejects empty posts (unless media or a repost carries them) and posts whose visible text is over the limit.
fn validate_content(trimmed: &str, has_media: bool, is_repost: bool) -> Result<()> {
    if trimmed.is_empty() && !has_media && !is_repost {
        return Err(AppError::EmptyContent);
    }
    let char_count = visible_text_len(trimmed);
    if char_count > MAX_POST_CHARS {
        return Err(AppError::ContentTooLong {
            max: MAX_POST_CHARS,
            actual: char_count,
        });
    }
    Ok(())
}
