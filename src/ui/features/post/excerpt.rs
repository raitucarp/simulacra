//! Human-readable excerpts for Obsidian `[[post_id]]` wikilinks.

use super::consts::{HASHTAG_MARKER, WIKILINK_OPEN};
use crate::domain::Post;

/// Prefix marking a wikilink target as a post id.
const POST_ID_PREFIX: &str = "post_";
/// Number of leading words used for a resolved excerpt.
const EXCERPT_WORD_COUNT: usize = 3;
/// Suffix appended to a resolved excerpt.
const EXCERPT_ELLIPSIS: &str = "...";
/// Slugs longer than this many characters are shortened.
const SLUG_MAX_CHARS: usize = 6;
/// Number of characters kept when shortening a slug.
const SHORT_SLUG_CHARS: usize = 4;

/// Resolves an Obsidian `[[post_id]]` into a human-readable 3-word excerpt of the referenced thought
/// or a clean short slug.
pub fn resolve_wikilink_excerpt(target_id: &str, all_posts: &[Post]) -> String {
    excerpt_with(target_id, |id| all_posts.iter().find(|p| p.id.as_str() == id))
}

/// Shared excerpt resolution over any post finder.
pub(crate) fn excerpt_with<'a>(target_id: &str, find: impl Fn(&str) -> Option<&'a Post>) -> String {
    let clean = target_id.trim();
    if clean.starts_with(POST_ID_PREFIX) {
        if let Some(excerpt) = find(clean).and_then(leading_words_excerpt) {
            return excerpt;
        }
        let slug = clean.strip_prefix(POST_ID_PREFIX).unwrap_or(clean);
        if slug.chars().count() > SLUG_MAX_CHARS {
            let short_slug: String = slug.chars().take(SHORT_SLUG_CHARS).collect();
            return format!("{HASHTAG_MARKER}{short_slug}");
        }
    }
    clean.to_string()
}

/// First words of a post (skipping links and hashtags), or `None` when nothing remains.
fn leading_words_excerpt(linked: &Post) -> Option<String> {
    let words: Vec<&str> = linked
        .content
        .split_whitespace()
        .filter(|w| !w.starts_with(WIKILINK_OPEN) && !w.starts_with(HASHTAG_MARKER))
        .take(EXCERPT_WORD_COUNT)
        .collect();
    (!words.is_empty()).then(|| format!("{}{EXCERPT_ELLIPSIS}", words.join(" ")))
}
