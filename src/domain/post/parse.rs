//! Extraction of wikilinks, mentions, and hashtags from post bodies.

use super::Post;
use crate::domain::ids::PostId;

/// Opening delimiter of an Obsidian-style `[[post_id]]` wikilink.
pub const WIKILINK_OPEN: &str = "[[";
/// Closing delimiter of an Obsidian-style `[[post_id]]` wikilink.
pub const WIKILINK_CLOSE: &str = "]]";
/// Sigil prefixing an `@handle` mention.
pub const MENTION_SIGIL: char = '@';
/// Sigil prefixing a `#hashtag`.
pub const HASHTAG_SIGIL: char = '#';
/// Character allowed inside handles besides alphanumerics.
const HANDLE_JOINER: char = '_';
/// Minimum byte length of a sigil-prefixed token (sigil plus at least one character).
const MIN_TAGGED_TOKEN_LEN: usize = 2;

impl Post {
    /// Extracts Obsidian-style `[[post_id]]` mentions.
    pub fn parse_linked_posts(text: &str) -> Vec<PostId> {
        let mut links = Vec::new();
        for part in text.split(WIKILINK_OPEN) {
            if let Some(end) = part.find(WIKILINK_CLOSE) {
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
            .filter(|word| word.starts_with(MENTION_SIGIL) && word.len() >= MIN_TAGGED_TOKEN_LEN)
            .map(|word| {
                word.trim_matches(|c: char| {
                    !c.is_alphanumeric() && c != MENTION_SIGIL && c != HANDLE_JOINER
                })
                .to_string()
            })
            .collect()
    }

    /// Extracts `#hashtag` tags from post body.
    pub fn parse_hashtags(text: &str) -> Vec<String> {
        text.split_whitespace()
            .filter(|word| word.starts_with(HASHTAG_SIGIL) && word.len() >= MIN_TAGGED_TOKEN_LEN)
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric() && c != HASHTAG_SIGIL).to_string())
            .collect()
    }
}
