//! Missing context: what a note needs so it can be read on its own, later, by someone who
//! does not remember where it came from.
//!
//! - A link: a `[[wikilink]]`, a URL or a `#hashtag` places the note among other notes.
//! - A source: a note that sounds empirical (a number, a percentage, "studies show",
//!   "according to") should say where that comes from: a URL or a citation marker such as
//!   "(2019)", "et al.", "source:", "via", "doi".
//! - An opening that stands alone: a note that starts with "This", "It" or "They" points at
//!   something the reader of the note alone never saw.

use crate::domain::post::{HASHTAG_SIGIL, WIKILINK_CLOSE, WIKILINK_OPEN};

/// Marks of a web link.
const URL_MARKS: [&str; 2] = ["://", "www."];
/// Words (lower case) of a claim about evidence, in the app's languages.
const EMPIRICAL_WORDS: [&str; 16] = [
    "study", "studies", "research", "data", "evidence", "survey", "experiment", "statistic", "according to", "percent",
    "penelitian", "studie", "étude", "estudio", "estudo", "исследован",
];
/// Citation markers (lower case).
const CITATION_MARKS: [&str; 8] = ["et al", "source:", "sources:", "via ", "doi:", "cf.", "ibid", "quoted in"];
/// A bracketed year such as "(2019)" opens with this and has this many digits.
const YEAR_OPEN: char = '(';
const YEAR_DIGITS: usize = 4;
/// Opening words (lower case) that refer back to something earlier. Words that also open
/// impersonal sentences in their language ("es", "il", "das") are left out.
const DANGLING_PRONOUNS: [&str; 26] = [
    "this", "that", "these", "those", "it", "they", "them", "he", "she", "ini", "itu", "dia", "mereka", "dies", "diese",
    "cela", "ça", "ceci", "elle", "esto", "eso", "ello", "isso", "isto", "это", "они",
];
/// Punctuation stripped from the opening word.
const WORD_TRIM: [char; 8] = ['"', '\'', '“', '‘', '«', '(', ',', ':'];

/// Whether `text` holds a wikilink, a URL or a hashtag.
pub fn has_link(text: &str) -> bool {
    let wikilink = text.find(WIKILINK_OPEN).is_some_and(|at| text[at..].contains(WIKILINK_CLOSE));
    let hashtag = text.split_whitespace().any(|word| word.strip_prefix(HASHTAG_SIGIL).is_some_and(|tag| tag.chars().next().is_some_and(char::is_alphanumeric)));
    wikilink || hashtag || URL_MARKS.iter().any(|mark| text.contains(mark))
}

/// The prose of `text`: wikilinks, hashtags, mentions and URLs left out (their digits are
/// identifiers, not figures).
fn prose_only(text: &str) -> String {
    let unlinked: String = text.split(WIKILINK_OPEN).enumerate().map(|(at, part)| if at == 0 { part } else { part.split_once(WIKILINK_CLOSE).map_or("", |(_, rest)| rest) }).collect::<Vec<_>>().join(" ");
    let is_reference = |word: &&str| word.starts_with(HASHTAG_SIGIL) || word.starts_with('@') || URL_MARKS.iter().any(|mark| word.contains(mark));
    unlinked.split_whitespace().filter(|word| !is_reference(word)).collect::<Vec<_>>().join(" ")
}

/// Whether `text` sounds like a claim about evidence or numbers.
pub fn sounds_empirical(text: &str) -> bool {
    let prose = prose_only(text);
    let lower = prose.to_lowercase();
    prose.chars().any(|c| c.is_ascii_digit() || c == '%') || EMPIRICAL_WORDS.iter().any(|word| lower.contains(word))
}

/// Whether `text` holds a bracketed four-digit year such as "(2019)" or "(Smith, 2019)".
fn has_bracketed_year(text: &str) -> bool {
    text.split(YEAR_OPEN).skip(1).any(|inside| {
        let inside = inside.split(')').next().unwrap_or_default();
        inside.split(|c: char| !c.is_ascii_digit()).any(|run| run.len() == YEAR_DIGITS)
    })
}

/// Whether `text` cites where its claim comes from.
pub fn has_source(text: &str) -> bool {
    let lower = text.to_lowercase();
    URL_MARKS.iter().any(|mark| lower.contains(mark)) || CITATION_MARKS.iter().any(|mark| lower.contains(mark)) || has_bracketed_year(text)
}

/// The opening word of `text` when it is a pronoun that points back at nothing.
pub fn dangling_pronoun(text: &str) -> Option<String> {
    let first = text.split_whitespace().next()?.trim_matches(WORD_TRIM);
    let bare = first.trim_end_matches(|c: char| !c.is_alphanumeric());
    let lower = bare.to_lowercase();
    DANGLING_PRONOUNS.contains(&lower.as_str()).then(|| bare.to_string())
}
