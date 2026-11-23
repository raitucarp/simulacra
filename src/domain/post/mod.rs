//! Thought post entity, parsing logic, and character limit rules.

pub mod anchor; // N2: margin replies
mod author_ref;
pub mod auto_delete; // Y7
mod authored;
pub mod depth; // N5
pub mod format;
pub mod garden; // N3
pub mod highlight;
mod constructors;
mod extras;
mod kind;
mod lifecycle;
pub mod lock; // X2: password-locked posts
mod parse;
mod provenance;
pub mod refs; // N1: embeds, quotes and wikilinks, and backlinks
mod relations;
pub mod schedule;
pub mod sanitize;
pub mod seed;
mod tint;
mod tokens;

pub use tint::{lenient as lenient_tint, Tint};

pub use authored::AuthoredPost;
pub use extras::{PostExtras, SourceKind, SourceRef, ToolUse};
pub use kind::{FeelingProblem, Intensity, KindKey, PostKind, MAX_FEELING_EMOJIS, MAX_FEELING_NOTE_CHARS}; // M1: feelings
pub use lifecycle::{PostLifecycle, PostStatus};
pub use parse::{HASHTAG_SIGIL, WIKILINK_CLOSE, WIKILINK_OPEN};
pub use provenance::{PostOrigin, SourceLocator};
pub use seed::{PostSeed, ResolvedSeed, SeedDocument, TriggerMode};
pub use tokens::{
    embed_markup, embed_targets, has_rich_markup, pin_embeds, is_trailing_punctuation, tokenize_post_body, visible_len, visible_text, visible_text_len, wikilink_targets,
    BodyToken, Mark, EMBED_OPEN,
};

use super::ids::{NetworkId, PostId, ProfileId};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Maximum allowed character limit per post, enforcing micro-blog discipline.
pub const MAX_POST_CHARS: usize = 280;

/// Atomic thought unit in the second-brain feed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub network_id: NetworkId,
    /// Author identity; the profile itself is resolved through the store.
    #[serde(alias = "author", deserialize_with = "author_ref::deserialize")]
    pub author_id: ProfileId,
    pub content: String,
    pub created_at: Timestamp,
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
    /// Engine metadata (mood, image prompt, tools used, sources); empty for human posts.
    #[serde(default, skip_serializing_if = "PostExtras::is_empty")]
    pub extras: PostExtras,
    /// The form of the thought (`posts.kind`).
    #[serde(default, skip_serializing_if = "PostKind::is_standard")]
    pub kind: PostKind,
    /// Publication state and due moments (`posts.status`, `publish_at`, `archive_at`, `delete_at`).
    #[serde(default, skip_serializing_if = "PostLifecycle::is_default")]
    pub lifecycle: PostLifecycle,
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

    /// Whether this thought has any media attachment (link, local image or local file).
    #[inline]
    pub fn has_attachments(&self) -> bool {
        self.link_url.is_some() || self.image_path.is_some() || !self.extras.files.is_empty()
    }
}

#[cfg(test)]
mod tests;
