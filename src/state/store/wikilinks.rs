//! Obsidian-style wikilink excerpts and resolution.

use super::{AppState, POST_ID_PREFIX};
use crate::domain::{Post, PostId};

/// Number of content words shown in a wikilink preview.
const EXCERPT_WORD_COUNT: usize = 3;
/// Suffix appended to a word-based preview.
const EXCERPT_ELLIPSIS: &str = "...";
/// Words starting with these markers are skipped in previews.
const WIKILINK_OPEN: &str = "[[";
const HASHTAG_MARKER: char = '#';
/// Slugs longer than this many characters are abbreviated.
const SLUG_ABBREVIATION_THRESHOLD: usize = 6;
/// Number of slug characters kept in an abbreviation.
const SLUG_ABBREVIATION_LEN: usize = 4;

/// Abbreviates a long slug to `#xxxx`, or returns `None` if it is short.
fn abbreviate_slug(slug: &str) -> Option<String> {
    if slug.chars().count() > SLUG_ABBREVIATION_THRESHOLD {
        let short_slug: String = slug.chars().take(SLUG_ABBREVIATION_LEN).collect();
        Some(format!("{HASHTAG_MARKER}{short_slug}"))
    } else {
        None
    }
}

impl AppState {
    /// Computes the standard wikilink excerpt for a thought post.
    pub fn compute_wikilink_excerpt(post: &Post) -> String {
        let words: Vec<&str> = post
            .content
            .split_whitespace()
            .filter(|w| !w.starts_with(WIKILINK_OPEN) && !w.starts_with(HASHTAG_MARKER))
            .take(EXCERPT_WORD_COUNT)
            .collect();
        if !words.is_empty() {
            format!("{}{EXCERPT_ELLIPSIS}", words.join(" "))
        } else {
            let clean = post.id.as_str();
            let slug = clean.strip_prefix(POST_ID_PREFIX).unwrap_or(clean);
            abbreviate_slug(slug).unwrap_or_else(|| clean.to_string())
        }
    }

    /// Resolves an Obsidian wikilink excerpt in O(1) time using the precomputed cache.
    ///
    /// If the referenced target post exists in the vault, returns the cached 3-word preview.
    /// Otherwise, generates a fallback 4-character slug without linear searching.
    pub fn resolve_wikilink_fast(&self, target_id: &str) -> String {
        let clean = target_id.trim();
        if let Some(meta) = self.index.meta(clean) {
            return meta.excerpt.clone();
        }
        if clean.starts_with(POST_ID_PREFIX) {
            let slug = clean.strip_prefix(POST_ID_PREFIX).unwrap_or(clean);
            if let Some(short) = abbreviate_slug(slug) {
                return short;
            }
        }
        clean.to_string()
    }

    /// What a `[[target]]` wikilink names: a post id, else a document id or (case-insensitive) title.
    pub fn resolve_wikilink_target(&self, target: &str) -> Option<WikilinkTarget> {
        let clean = target.trim();
        if let Some(post) = self.get_post_by_id(clean) {
            return Some(WikilinkTarget::Post(post.id.clone()));
        }
        let title = clean.to_lowercase();
        self.documents
            .iter()
            .find(|d| d.id == clean || d.title.trim().to_lowercase() == title)
            .map(|d| WikilinkTarget::Document(d.id.clone()))
    }
}

/// The entity a wikilink points at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WikilinkTarget {
    Post(PostId),
    Document(String),
}
