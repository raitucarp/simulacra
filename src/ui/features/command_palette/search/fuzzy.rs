//! Pure fuzzy matching: does every query character appear in order in the candidate, and how
//! good is the match? Case-insensitive; whitespace in the query is ignored.
//!
//! The matcher finds the leftmost match, then tightens it backwards from its end (so "mg"
//! in "Mind Graph" lands on the word starts rather than a scattered early run), and scores
//! the positions: word starts, consecutive runs and an early first hit score higher, gaps
//! cost a little.

/// Points for every matched character.
const MATCH_POINTS: i32 = 16;
/// Bonus when a match directly follows the previous one.
const CONSECUTIVE_BONUS: i32 = 14;
/// Bonus when a match starts a word (after a separator or at a lower-to-upper case change).
const WORD_START_BONUS: i32 = 10;
/// Bonus when the first match is the candidate's first character.
const PREFIX_BONUS: i32 = 12;
/// Bonus when the whole candidate equals the query.
const EXACT_BONUS: i32 = 40;
/// Cost per skipped character between two matches.
const GAP_PENALTY: i32 = 2;
/// Cost per character before the first match, and its cap.
const LEADING_PENALTY: i32 = 1;
const MAX_LEADING_PENALTY: i32 = 12;

/// A successful match: its score and the matched character positions (char indices).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzyMatch {
    pub score: i32,
    pub positions: Vec<usize>,
}

/// Matches `query` against `candidate`; `None` when some query character is missing.
/// An empty query matches everything with score zero.
pub fn fuzzy_match(query: &str, candidate: &str) -> Option<FuzzyMatch> {
    let needle: Vec<char> = query.chars().filter(|c| !c.is_whitespace()).map(fold_case).collect();
    if needle.is_empty() {
        return Some(FuzzyMatch { score: 0, positions: Vec::new() });
    }
    let original: Vec<char> = candidate.chars().collect();
    let hay: Vec<char> = original.iter().copied().map(fold_case).collect();
    let end = forward_end(&needle, &hay)?;
    let start = backward_start(&needle, &hay, end);
    let positions = forward_positions(&needle, &hay, start);
    let exact = needle.len() == hay.len();
    Some(FuzzyMatch { score: score(&positions, &original, exact), positions })
}

/// Lower-cases one character, keeping a one-to-one char mapping.
fn fold_case(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// Index of the last character of the leftmost greedy match.
fn forward_end(needle: &[char], hay: &[char]) -> Option<usize> {
    let mut next = 0;
    for (index, c) in hay.iter().enumerate() {
        if *c == needle[next] {
            next += 1;
            if next == needle.len() {
                return Some(index);
            }
        }
    }
    None
}

/// Start of the tightest match ending at `end`, scanning backwards.
fn backward_start(needle: &[char], hay: &[char], end: usize) -> usize {
    let mut remaining = needle.len();
    for index in (0..=end).rev() {
        if hay[index] == needle[remaining - 1] {
            remaining -= 1;
            if remaining == 0 {
                return index;
            }
        }
    }
    0
}

/// Greedy match positions from `start`.
fn forward_positions(needle: &[char], hay: &[char], start: usize) -> Vec<usize> {
    let mut next = 0;
    let mut positions = Vec::with_capacity(needle.len());
    for (index, c) in hay.iter().enumerate().skip(start) {
        if next < needle.len() && *c == needle[next] {
            positions.push(index);
            next += 1;
        }
    }
    positions
}

fn is_word_start(original: &[char], index: usize) -> bool {
    let Some(previous) = index.checked_sub(1).map(|at| original[at]) else { return true };
    !previous.is_alphanumeric() || (previous.is_lowercase() && original[index].is_uppercase())
}

fn score(positions: &[usize], original: &[char], exact: bool) -> i32 {
    let first = positions.first().copied().unwrap_or(0);
    let leading = (first as i32 * LEADING_PENALTY).min(MAX_LEADING_PENALTY);
    let per_char: i32 = positions.iter().enumerate().map(|(nth, &at)| {
        let follows = nth > 0 && positions[nth - 1] + 1 == at;
        let gap = if nth > 0 && !follows { (at - positions[nth - 1] - 1) as i32 * GAP_PENALTY } else { 0 };
        let word = if is_word_start(original, at) { WORD_START_BONUS } else { 0 };
        MATCH_POINTS + word - gap + if follows { CONSECUTIVE_BONUS } else { 0 }
    }).sum();
    let prefix = if first == 0 { PREFIX_BONUS } else { 0 };
    per_char + prefix - leading + if exact { EXACT_BONUS } else { 0 }
}
