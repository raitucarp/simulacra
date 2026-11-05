//! Thought post entity, parsing logic, and character limit rules.

use super::ids::{NetworkId, PostId};
use super::profile::Profile;
use crate::error::{AppError, Result};
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

    /// Backwards-compatible constructor for standard thought posts.
    pub fn create(
        id: impl Into<PostId>,
        network_id: impl Into<NetworkId>,
        author: Profile,
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
        author: Profile,
        raw_content: &str,
        link_url: Option<String>,
        image_path: Option<String>,
        reply_to_id: Option<PostId>,
        quote_id: Option<PostId>,
        repost_id: Option<PostId>,
    ) -> Result<Self> {
        let trimmed = raw_content.trim();
        let is_repost = repost_id.is_some();
        let clean_link = link_url.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let clean_image = image_path.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let has_media = clean_link.is_some() || clean_image.is_some();

        if trimmed.is_empty() && !has_media && !is_repost {
            return Err(AppError::EmptyContent);
        }

        let char_count = trimmed.chars().count();
        if char_count > MAX_POST_CHARS {
            return Err(AppError::ContentTooLong {
                max: MAX_POST_CHARS,
                actual: char_count,
            });
        }

        let mut linked_posts = Self::parse_linked_posts(trimmed);
        let mentions = Self::parse_mentions(trimmed);

        // Include referenced post IDs into graph connections
        if let Some(ref qid) = quote_id {
            if !linked_posts.contains(qid) {
                linked_posts.push(qid.clone());
            }
        }
        if let Some(ref rid) = reply_to_id {
            if !linked_posts.contains(rid) {
                linked_posts.push(rid.clone());
            }
        }
        if let Some(ref rep_id) = repost_id {
            if !linked_posts.contains(rep_id) {
                linked_posts.push(rep_id.clone());
            }
        }

        Ok(Self {
            id: id.into(),
            network_id: network_id.into(),
            author,
            content: trimmed.to_string(),
            created_at: Utc::now(),
            likes: 0,
            is_liked: false,
            replies_count: 0,
            mentions,
            linked_posts,
            reply_to_id,
            quote_id,
            repost_id,
            link_url: clean_link,
            image_path: clean_image,
            reposts_count: 0,
            is_reposted: false,
        })
    }

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
            "",
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

    /// Extracts Obsidian-style `[[post_id]]` mentions.
    pub fn parse_linked_posts(text: &str) -> Vec<PostId> {
        let mut links = Vec::new();
        for part in text.split("[[") {
            if let Some(end) = part.find("]]") {
                let link_id = part[..end].trim();
                if !link_id.is_empty() {
                    links.push(PostId::new(link_id));
                }
            }
        }
        links
    }

    /// Extracts `@handle` mentions from post body.
    pub fn parse_mentions(text: &str) -> Vec<String> {
        text.split_whitespace()
            .filter(|word| word.starts_with('@') && word.len() > 1)
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '_').to_string())
            .collect()
    }

    /// Extracts `#hashtag` tags from post body.
    pub fn parse_hashtags(text: &str) -> Vec<String> {
        text.split_whitespace()
            .filter(|word| word.starts_with('#') && word.len() > 1)
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric() && c != '#').to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_post_creation() {
        let author = Profile::new_primary("u1", "Architect", "@arch");
        let post = Post::create(
            "p1",
            "net1",
            author,
            "Mind dump: Testing semiotics and [[post_42]] with #AI @friend",
        )
        .expect("Valid post");

        assert_eq!(post.content, "Mind dump: Testing semiotics and [[post_42]] with #AI @friend");
        assert_eq!(post.linked_posts, vec![PostId::new("post_42")]);
        assert_eq!(post.mentions, vec!["@friend".to_string()]);
        assert_eq!(post.reply_to_id, None);
        assert_eq!(post.quote_id, None);
        assert_eq!(post.repost_id, None);
        assert_eq!(post.reposts_count, 0);
        assert!(!post.is_reposted);
    }

    #[test]
    fn test_empty_post_fails() {
        let author = Profile::new_primary("u1", "Architect", "@arch");
        let result = Post::create("p1", "net1", author, "    ");
        assert_eq!(result, Err(AppError::EmptyContent));
    }

    #[test]
    fn test_exceeding_character_limit_fails() {
        let author = Profile::new_primary("u1", "Architect", "@arch");
        let long_text = "a".repeat(281);
        let result = Post::create("p1", "net1", author, &long_text);
        assert_eq!(
            result,
            Err(AppError::ContentTooLong {
                max: 280,
                actual: 281
            })
        );
    }

    #[test]
    fn test_create_with_attachments_and_links() {
        let author = Profile::new_primary("u1", "Architect", "@arch");
        let post = Post::create_with_attachments(
            "p_attach",
            "net1",
            author,
            "Check this link and screenshot [[post_42]]",
            Some("https://example.com/paper.pdf".to_string()),
            Some("/tmp/diagram.png".to_string()),
            Some(PostId::new("post_parent")),
            Some(PostId::new("post_quote")),
            None,
        )
        .expect("Should create post with attachments");

        assert_eq!(post.link_url, Some("https://example.com/paper.pdf".to_string()));
        assert_eq!(post.image_path, Some("/tmp/diagram.png".to_string()));
        assert_eq!(post.reply_to_id, Some(PostId::new("post_parent")));
        assert_eq!(post.quote_id, Some(PostId::new("post_quote")));
        assert!(post.linked_posts.contains(&PostId::new("post_42")));
        assert!(post.linked_posts.contains(&PostId::new("post_parent")));
        assert!(post.linked_posts.contains(&PostId::new("post_quote")));
        assert!(post.has_attachments());
        assert!(post.is_reply());
        assert!(post.is_quote());
        assert!(!post.is_repost());
    }

    #[test]
    fn test_create_repost_allows_empty_content() {
        let author = Profile::new_primary("u1", "Architect", "@arch");
        let repost = Post::create_repost("p_rep", "net1", author, "post_original")
            .expect("Instant repost allows empty content");

        assert_eq!(repost.content, "");
        assert_eq!(repost.repost_id, Some(PostId::new("post_original")));
        assert!(repost.is_repost());
    }

    #[test]
    fn test_post_serde_backward_compatibility() {
        let legacy_json = r#"{
            "id": "post_old",
            "network_id": "net_philosophy",
            "author": {
                "id": "u1",
                "name": "Architect",
                "handle": "@arch",
                "avatar_initial": "A",
                "role": "UserPrimary"
            },
            "content": "Legacy post content",
            "created_at": "2026-01-01T00:00:00Z",
            "likes": 5,
            "is_liked": true,
            "replies_count": 2,
            "mentions": [],
            "linked_posts": []
        }"#;

        let post: Post = serde_json::from_str(legacy_json).expect("Deserialize legacy JSON");
        assert_eq!(post.reply_to_id, None);
        assert_eq!(post.quote_id, None);
        assert_eq!(post.repost_id, None);
        assert_eq!(post.link_url, None);
        assert_eq!(post.image_path, None);
        assert_eq!(post.reposts_count, 0);
        assert!(!post.is_reposted);
    }

    #[test]
    fn test_post_toggle_repost() {
        let author = Profile::new_primary("u1", "Architect", "@arch");
        let mut post = Post::create("p1", "net1", author, "Test content").unwrap();
        assert_eq!(post.reposts_count, 0);
        assert!(!post.is_reposted);

        post.toggle_repost();
        assert_eq!(post.reposts_count, 1);
        assert!(post.is_reposted);

        post.toggle_repost();
        assert_eq!(post.reposts_count, 0);
        assert!(!post.is_reposted);
    }
}
