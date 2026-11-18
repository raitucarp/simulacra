//! Cloze deletions: `{{c1::answer}}` or `{{c1::answer::hint}}` inside a text. Every distinct
//! number becomes its own card; on that card its deletions are blanked and the others shown.

use std::collections::BTreeSet;

const OPEN: &str = "{{c";
const SEPARATOR: &str = "::";
const CLOSE: &str = "}}";

/// A piece of a cloze text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClozePart {
    Plain(String),
    Deletion { ordinal: u32, answer: String, hint: Option<String> },
}

/// Parses one `{{cN::…}}` body (`N::answer[::hint]`) after the opening `{{c`.
fn parse_deletion(body: &str) -> Option<ClozePart> {
    let (number, rest) = body.split_once(SEPARATOR)?;
    let ordinal = number.trim().parse::<u32>().ok().filter(|n| *n > 0)?;
    let (answer, hint) = match rest.split_once(SEPARATOR) {
        Some((answer, hint)) => (answer, Some(hint.trim().to_string()).filter(|h| !h.is_empty())),
        None => (rest, None),
    };
    (!answer.trim().is_empty()).then(|| ClozePart::Deletion { ordinal, answer: answer.to_string(), hint })
}

/// The parts of `text`, in order; malformed markup stays plain text.
pub fn parse_cloze(text: &str) -> Vec<ClozePart> {
    let mut parts = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find(OPEN) {
        let after = &rest[start + OPEN.len()..];
        let Some(end) = after.find(CLOSE) else { break };
        let Some(deletion) = parse_deletion(&after[..end]) else {
            push_plain(&mut parts, &rest[..start + OPEN.len()]);
            rest = after;
            continue;
        };
        push_plain(&mut parts, &rest[..start]);
        parts.push(deletion);
        rest = &after[end + CLOSE.len()..];
    }
    push_plain(&mut parts, rest);
    parts
}

fn push_plain(parts: &mut Vec<ClozePart>, text: &str) {
    if text.is_empty() {
        return;
    }
    match parts.last_mut() {
        Some(ClozePart::Plain(previous)) => previous.push_str(text),
        _ => parts.push(ClozePart::Plain(text.to_string())),
    }
}

/// Every deletion number in `text`, ascending.
pub fn cloze_ordinals(text: &str) -> Vec<u32> {
    let ordinals: BTreeSet<u32> = parse_cloze(text)
        .into_iter()
        .filter_map(|part| match part {
            ClozePart::Deletion { ordinal, .. } => Some(ordinal),
            ClozePart::Plain(_) => None,
        })
        .collect();
    ordinals.into_iter().collect()
}

/// The text with every deletion replaced by its answer.
pub fn plain_text(text: &str) -> String {
    parse_cloze(text)
        .into_iter()
        .map(|part| match part {
            ClozePart::Plain(text) => text,
            ClozePart::Deletion { answer, .. } => answer,
        })
        .collect()
}

/// The answers deleted on card `ordinal`, joined in reading order.
pub fn deleted_answer(text: &str, ordinal: u32) -> String {
    let answers: Vec<String> = parse_cloze(text)
        .into_iter()
        .filter_map(|part| match part {
            ClozePart::Deletion { ordinal: n, answer, .. } if n == ordinal => Some(answer),
            _ => None,
        })
        .collect();
    answers.join(" … ")
}

/// Words of `sentence` split on whitespace (a picked word's trailing punctuation stays
/// outside its deletion, see [`cloze_from_words`]).
pub fn cloze_words(sentence: &str) -> Vec<String> {
    sentence.split_whitespace().map(str::to_string).collect()
}

/// `words` joined back into a sentence, each picked word wrapped in its own deletion
/// (numbered in reading order).
pub fn cloze_from_words(words: &[String], picked: &BTreeSet<usize>) -> String {
    let mut next = 0u32;
    let rendered: Vec<String> = words
        .iter()
        .enumerate()
        .map(|(index, word)| {
            if !picked.contains(&index) {
                return word.clone();
            }
            let trimmed = word.trim_end_matches(|c: char| c.is_ascii_punctuation());
            let core = if trimmed.is_empty() { word.as_str() } else { trimmed };
            let tail = &word[core.len()..];
            next += 1;
            format!("{OPEN}{next}{SEPARATOR}{core}{CLOSE}{tail}")
        })
        .collect();
    rendered.join(" ")
}
