//! A rule-based sentence splitter.
//!
//! - A run of terminators (`.`, `!`, `?`, `…`, `؟`, `।`), then any closing quotes or brackets,
//!   ends a sentence when whitespace (or the end) follows and the next word is not lower case.
//! - A single `.` after an abbreviation of the locale (`Mr.`, `z.B.`) or an initial (`J.`)
//!   does not end a sentence.
//! - CJK terminators (`。`, `！`, `？`) end a sentence without needing whitespace.

use super::locale::SentenceLocale;

const TERMINATORS: [char; 7] = ['.', '!', '?', '…', '‽', '؟', '।'];
const CJK_TERMINATORS: [char; 4] = ['。', '！', '？', '｡'];
const CLOSERS: [char; 14] = ['"', '\'', '”', '’', '»', '›', ')', ']', '」', '』', '）', '】', '》', '〉'];
const OPENERS: [char; 8] = ['"', '\'', '“', '‘', '«', '(', '[', '¿'];

/// The word just before the terminator at `at` (without opening quotes or brackets).
fn word_before(chars: &[(usize, char)], at: usize) -> String {
    let start = chars[..at].iter().rposition(|(_, c)| c.is_whitespace() || OPENERS.contains(c)).map_or(0, |i| i + 1);
    chars[start..at].iter().map(|(_, c)| c).collect()
}

/// Whether a single dot at `at` follows an abbreviation or an initial.
fn is_abbreviation_dot(chars: &[(usize, char)], at: usize, locale: SentenceLocale) -> bool {
    let word = word_before(chars, at);
    let mut letters = word.chars();
    let is_initial = matches!((letters.next(), letters.next()), (Some(c), None) if c.is_uppercase());
    is_initial || locale.is_abbreviation(&word)
}

/// Index just past `from` and any run of `set` characters.
fn skip_run(chars: &[(usize, char)], from: usize, set: &[char]) -> usize {
    from + chars[from..].iter().take_while(|(_, c)| set.contains(c)).count()
}

/// Whether the terminator run `start..end` (closers included) ends a sentence.
fn ends_sentence(chars: &[(usize, char)], start: usize, end: usize, locale: SentenceLocale) -> bool {
    if chars.get(end).is_some_and(|(_, c)| !c.is_whitespace()) {
        return false;
    }
    let next = chars[end..].iter().map(|(_, c)| *c).find(|c| !c.is_whitespace());
    if next.is_some_and(char::is_lowercase) {
        return false;
    }
    let single_dot = chars[start].1 == '.' && !chars.get(start + 1).is_some_and(|(_, c)| *c == '.');
    !(single_dot && next.is_some() && is_abbreviation_dot(chars, start, locale))
}

/// The sentences of `text`, trimmed, in order.
pub fn split_sentences(text: &str, locale: SentenceLocale) -> Vec<&str> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let byte_at = |index: usize| chars.get(index).map_or(text.len(), |(byte, _)| *byte);
    let mut cuts = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index].1;
        if CJK_TERMINATORS.contains(&c) {
            index = skip_run(&chars, skip_run(&chars, index, &CJK_TERMINATORS), &CLOSERS);
            cuts.push(byte_at(index));
        } else if TERMINATORS.contains(&c) {
            let end = skip_run(&chars, skip_run(&chars, index, &TERMINATORS), &CLOSERS);
            if ends_sentence(&chars, index, end, locale) {
                cuts.push(byte_at(end));
            }
            index = end;
        } else {
            index += 1;
        }
    }
    cuts.push(text.len());
    let mut start = 0;
    cuts.into_iter()
        .filter_map(|cut| {
            let piece = text[start..cut].trim();
            start = cut;
            (!piece.is_empty()).then_some(piece)
        })
        .collect()
}
