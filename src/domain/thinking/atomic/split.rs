//! "Split into atomic posts": a proposal of one post per claim, for the user to edit before
//! posting it as a thread that cites the original.
//!
//! Each post is one claim with the questions and fragments that followed it (a leading
//! question waits for the first claim), its own wikilinks and hashtags kept as written. A
//! clause cut out of a longer sentence is made to read as a sentence: its first letter is
//! raised and a full stop added when it ends without one. Posts that open with a pronoun are
//! kept as they are; the user sees the check's hint and rewrites them. At most
//! [`MAX_SPLIT`] posts: the rest merges into the last one.

use super::claims::{claim_groups, ClaimGroup};
use crate::domain::post::{HASHTAG_SIGIL, WIKILINK_CLOSE};
use crate::import::segment::SentenceLocale;

/// Most posts a split proposes.
pub const MAX_SPLIT: usize = 8;
/// Marks that end a sentence (a clause ending with none gets a full stop).
const SENTENCE_ENDS: [char; 9] = ['.', '!', '?', '…', '。', '！', '？', '؟', ':'];
/// Closing quotes and brackets allowed after the final mark.
const CLOSERS: [char; 6] = ['"', '\'', '”', '’', ')', '»'];
const FULL_STOP: char = '.';

/// `piece` as a sentence: first letter raised, a full stop when it ends without a mark (a
/// trailing hashtag or wikilink needs none).
fn as_sentence(piece: &str) -> String {
    let piece = piece.trim();
    let mut chars = piece.chars();
    let raised: String = match chars.next() {
        Some(first) if first.is_lowercase() => first.to_uppercase().chain(chars).collect(),
        _ => piece.to_string(),
    };
    let last_word = raised.split_whitespace().last().unwrap_or_default();
    let reference = last_word.starts_with(HASHTAG_SIGIL) || last_word.ends_with(WIKILINK_CLOSE);
    let ended = reference || raised.trim_end_matches(CLOSERS).ends_with(SENTENCE_ENDS);
    if ended || raised.is_empty() {
        raised
    } else {
        format!("{raised}{FULL_STOP}")
    }
}

fn post_of(group: &ClaimGroup) -> String {
    group.pieces.iter().map(|piece| as_sentence(piece)).collect::<Vec<_>>().join(" ")
}

/// The proposal for `text`: one post per claim (the whole text when it makes one claim,
/// nothing when it is blank).
pub fn split_proposal(text: &str, locale: SentenceLocale) -> Vec<String> {
    let text = text.trim();
    if text.is_empty() {
        return Vec::new();
    }
    let posts: Vec<String> = claim_groups(text, locale).iter().map(post_of).filter(|post| !post.is_empty()).collect();
    if posts.len() <= 1 {
        return vec![text.to_string()];
    }
    let keep = MAX_SPLIT.saturating_sub(1);
    let tail = posts[keep.min(posts.len())..].join(" ");
    posts.into_iter().take(keep).chain((!tail.is_empty()).then_some(tail)).collect()
}
