//! Cutting text into pieces: list items stay whole, prose paragraphs go through the
//! locale-aware sentence splitter, and each sentence is cut further at `;` and at a
//! coordinating conjunction that joins two independent clauses.
//!
//! A conjunction cut is only taken when both sides have at least [`MIN_CLAUSE_WORDS`] words,
//! a cheap stand-in for "both sides have their own subject and verb": "tea, and coffee"
//! stays whole, "cats sleep all day, but dogs need walks" becomes two clauses. The cut drops
//! the comma and the conjunction itself.

use crate::import::segment::{split_sentences, SentenceLocale};

/// Bullets that open a list item.
const BULLETS: [&str; 4] = ["- ", "* ", "• ", "– "];
/// What follows the number of a numbered item.
const NUMBER_ENDS: [&str; 2] = [". ", ") "];
/// Clause separators inside a sentence (ASCII and full width).
const SEMICOLONS: [char; 2] = [';', '；'];
/// Coordinating conjunctions between independent clauses, lower case with their spaces:
/// English, Indonesian, German, French, Spanish, Portuguese.
const CONJUNCTIONS: [&str; 13] = [
    ", and ", ", but ", ", so ", ", yet ", " whereas ", ", tetapi ", ", namun ", ", aber ", ", sondern ", ", mais ",
    ", pero ", ", sino ", ", mas ",
];
/// Fewest words on each side of a conjunction cut.
pub const MIN_CLAUSE_WORDS: usize = 3;

/// The text of a list item line (without its bullet or number), if it is one.
pub fn list_item(line: &str) -> Option<&str> {
    let line = line.trim_start();
    if let Some(rest) = BULLETS.iter().find_map(|bullet| line.strip_prefix(bullet)) {
        return Some(rest.trim());
    }
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    let rest = &line[digits..];
    NUMBER_ENDS.iter().find_map(|end| rest.strip_prefix(end)).filter(|_| digits > 0).map(str::trim)
}

/// Words of `text` that carry a letter or a digit.
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().filter(|word| word.chars().any(char::is_alphanumeric)).count()
}

/// The first conjunction cut of `sentence` whose two sides are both long enough.
fn conjunction_cut(sentence: &str) -> Option<(&str, &str)> {
    let lower = sentence.to_ascii_lowercase();
    CONJUNCTIONS
        .iter()
        .flat_map(|marker| lower.match_indices(marker).map(move |(at, _)| (at, at + marker.len())))
        .filter(|(at, end)| word_count(&sentence[..*at]) >= MIN_CLAUSE_WORDS && word_count(&sentence[*end..]) >= MIN_CLAUSE_WORDS)
        .min_by_key(|(at, _)| *at)
        .map(|(at, end)| (sentence[..at].trim(), sentence[end..].trim()))
}

/// `sentence` cut at every independent-clause conjunction.
fn conjunction_clauses(sentence: &str) -> Vec<&str> {
    match conjunction_cut(sentence) {
        Some((left, right)) => std::iter::once(left).chain(conjunction_clauses(right)).collect(),
        None => vec![sentence.trim()],
    }
}

/// The clauses of one sentence, in order.
pub fn clauses_of(sentence: &str) -> Vec<&str> {
    sentence
        .split(SEMICOLONS)
        .flat_map(conjunction_clauses)
        .map(str::trim)
        .filter(|clause| !clause.is_empty())
        .collect()
}

/// Moves the gathered prose lines into `blocks` as one paragraph.
fn flush(prose: &mut Vec<&str>, blocks: &mut Vec<String>) {
    if !prose.is_empty() {
        blocks.push(prose.join(" "));
        prose.clear();
    }
}

/// Paragraphs (blank-line separated, lines joined) and list items of `text`, in order.
fn blocks(text: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    let mut prose: Vec<&str> = Vec::new();
    for line in text.lines().map(str::trim) {
        match list_item(line) {
            Some(item) => {
                flush(&mut prose, &mut blocks);
                blocks.push(item.to_string());
            }
            None if line.is_empty() => flush(&mut prose, &mut blocks),
            None => prose.push(line),
        }
    }
    flush(&mut prose, &mut blocks);
    blocks
}

/// Every piece of `text` (list items, then sentences cut into clauses), trimmed, in order.
pub fn pieces(text: &str, locale: SentenceLocale) -> Vec<String> {
    blocks(text)
        .iter()
        .flat_map(|block| split_sentences(block, locale).into_iter().flat_map(clauses_of).map(str::to_string).collect::<Vec<_>>())
        .filter(|piece| !piece.is_empty())
        .collect()
}
