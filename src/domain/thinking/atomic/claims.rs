//! Which pieces of a note are claims, and how the pieces group into one post per claim.
//!
//! A piece is a claim unless it is a question (it asks, it does not assert) or a fragment:
//! fewer than [`MIN_CLAIM_WORDS`] words in a script that separates words with spaces, or
//! fewer than [`MIN_UNSPACED_CLAIM_CHARS`] characters in one that does not (Chinese,
//! Japanese, Korean). Wikilink brackets and hashtag signs are ignored when counting. Equal
//! claims (same letters and digits, any case) count once.

use super::clauses::{pieces, word_count};
use crate::domain::post::{HASHTAG_SIGIL, WIKILINK_CLOSE, WIKILINK_OPEN};
use crate::import::segment::SentenceLocale;

/// Fewest words of a claim in a spaced script.
pub const MIN_CLAIM_WORDS: usize = 3;
/// Fewest characters of a claim in an unspaced script.
pub const MIN_UNSPACED_CLAIM_CHARS: usize = 5;
/// Question marks (Latin, full width, Arabic, interrobang).
const QUESTION_MARKS: [char; 4] = ['?', '？', '؟', '‽'];
/// Closing quotes and brackets that may follow the final mark.
const CLOSERS: [char; 8] = ['"', '\'', '”', '’', '»', ')', ']', '」'];
/// Unicode blocks of unspaced scripts: kana, CJK ideographs, Hangul syllables.
const UNSPACED_RANGES: [(char, char); 3] = [('\u{3040}', '\u{30ff}'), ('\u{4e00}', '\u{9fff}'), ('\u{ac00}', '\u{d7af}')];

/// `text` without wikilink brackets and hashtag signs (for counting only).
pub fn counting_text(text: &str) -> String {
    text.replace(WIKILINK_OPEN, " ").replace(WIKILINK_CLOSE, " ").replace(HASHTAG_SIGIL, "")
}

fn is_unspaced(c: char) -> bool {
    UNSPACED_RANGES.iter().any(|(low, high)| (*low..=*high).contains(&c))
}

pub fn is_question(piece: &str) -> bool {
    piece.trim_end().trim_end_matches(CLOSERS).ends_with(QUESTION_MARKS)
}

pub fn is_fragment(piece: &str) -> bool {
    let counted = counting_text(piece);
    let unspaced = counted.chars().any(is_unspaced) && !counted.trim().contains(char::is_whitespace);
    if unspaced {
        counted.chars().filter(|c| c.is_alphanumeric()).count() < MIN_UNSPACED_CLAIM_CHARS
    } else {
        word_count(&counted) < MIN_CLAIM_WORDS
    }
}

pub fn is_claim(piece: &str) -> bool {
    !is_question(piece) && !is_fragment(piece)
}

/// Letters and digits of `text`, lower case: equal claims share it.
fn claim_key(text: &str) -> String {
    counting_text(text).chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

/// The distinct claims of `text`, in order.
pub fn claims_of(text: &str, locale: SentenceLocale) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    pieces(text, locale)
        .into_iter()
        .filter(|piece| is_claim(piece))
        .filter(|piece| {
            let key = claim_key(piece);
            let fresh = !seen.contains(&key);
            seen.push(key);
            fresh
        })
        .collect()
}

/// One group of pieces: a claim with the questions and fragments around it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimGroup {
    pub pieces: Vec<String>,
    pub has_claim: bool,
}

/// The pieces of `text` grouped so each group holds one claim: a question or a fragment
/// joins the group before it, and a leading one waits for the first claim.
pub fn claim_groups(text: &str, locale: SentenceLocale) -> Vec<ClaimGroup> {
    pieces(text, locale).into_iter().fold(Vec::new(), |mut groups: Vec<ClaimGroup>, piece| {
        let claim = is_claim(&piece);
        match groups.last_mut() {
            Some(last) if !claim || !last.has_claim => {
                last.pieces.push(piece);
                last.has_claim |= claim;
            }
            _ => groups.push(ClaimGroup { pieces: vec![piece], has_claim: claim }),
        }
        groups
    })
}
