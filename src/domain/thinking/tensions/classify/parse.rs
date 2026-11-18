//! Reading a classification reply leniently: the JSON array (also inside code fences, prose
//! or a wrapping object), else one verdict per line ("1. contradicts — reason"). Pair numbers
//! out of range, repeated numbers and unknown verdicts are dropped.

use super::super::rank::Candidate;
use super::{Classification, Verdict};
use serde_json::Value;
use std::collections::BTreeSet;

/// Longest reason kept, in characters.
pub const MAX_REASON_CHARS: usize = 240;
const PAIR_FIELDS: &[&str] = &["pair", "index", "id"];
const VERDICT_FIELDS: &[&str] = &["verdict", "label", "relation"];
const REASON_FIELDS: &[&str] = &["reason", "explanation", "why"];
/// Marks before a line's pair number, and the separators after it.
const BULLETS: &[char] = &['-', '*', '•', ' '];
const SEPARATORS: &[char] = &['.', ':', ')', '-', '–', '—', '#', ' ', '\t'];
const PAIR_WORD: &str = "pair ";

/// One verdict as read, before it is matched to its pair.
struct Entry {
    number: usize,
    verdict: Verdict,
    reason: String,
}

/// The classifications in `raw` for the pairs of `batch` (numbered from 1).
pub fn parse_classifications(raw: &str, batch: &[Candidate]) -> Vec<Classification> {
    let entries = json_array(raw)
        .map(|values| values.iter().enumerate().filter_map(|(at, value)| json_entry(value, at)).collect::<Vec<_>>())
        .filter(|entries| !entries.is_empty())
        .unwrap_or_else(|| raw.lines().filter_map(line_entry).collect());
    let mut seen = BTreeSet::new();
    entries
        .into_iter()
        .filter(|entry| (1..=batch.len()).contains(&entry.number) && seen.insert(entry.number))
        .map(|entry| Classification { pair: batch[entry.number - 1].key(), verdict: entry.verdict, reason: cut(&entry.reason) })
        .collect()
}

/// The JSON between the first `open` and the last `close` of `raw`, if it parses.
fn json_between(raw: &str, open: char, close: char) -> Option<Value> {
    let (start, end) = (raw.find(open)?, raw.rfind(close)?);
    (end > start).then(|| serde_json::from_str::<Value>(&raw[start..=end]).ok()).flatten()
}

/// The array of the reply: a top-level array, or the first array inside an object.
fn json_array(raw: &str) -> Option<Vec<Value>> {
    match json_between(raw, '[', ']') {
        Some(Value::Array(values)) => Some(values),
        _ => json_between(raw, '{', '}').map(unwrap_object),
    }
}

/// The first array field of an object, or the object itself as a one-item list.
fn unwrap_object(value: Value) -> Vec<Value> {
    let nested = value.as_object().and_then(|map| map.values().find_map(|field| field.as_array().cloned()));
    nested.unwrap_or_else(|| vec![value])
}

fn field<'a>(value: &'a Value, names: &[&str]) -> Option<&'a Value> {
    names.iter().find_map(|name| value.get(*name))
}

/// One JSON object as an entry (a missing pair number means its position).
fn json_entry(value: &Value, position: usize) -> Option<Entry> {
    let number = match field(value, PAIR_FIELDS) {
        Some(Value::Number(number)) => number.as_u64().and_then(|n| usize::try_from(n).ok())?,
        Some(Value::String(text)) => first_number(text)?,
        _ => position + 1,
    };
    let verdict = field(value, VERDICT_FIELDS).and_then(Value::as_str).and_then(Verdict::read)?;
    let reason = field(value, REASON_FIELDS).and_then(Value::as_str).unwrap_or_default().to_string();
    Some(Entry { number, verdict, reason })
}

/// The first run of digits in `text`.
fn first_number(text: &str) -> Option<usize> {
    let digits: String = text.chars().skip_while(|c| !c.is_ascii_digit()).take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// "1. contradicts — reason", "Pair 2: compatible: reason" or "- 3) unrelated".
fn line_entry(line: &str) -> Option<Entry> {
    let line = line.trim().trim_start_matches(BULLETS);
    let line = match line.get(..PAIR_WORD.len()) {
        Some(head) if head.eq_ignore_ascii_case(PAIR_WORD) => &line[PAIR_WORD.len()..],
        _ => line,
    };
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    let number: usize = line[..digits].parse().ok()?;
    let rest = line[digits..].trim_start_matches(SEPARATORS);
    let word_end = rest.find(|c: char| !c.is_alphabetic()).unwrap_or(rest.len());
    let verdict = Verdict::read(&rest[..word_end])?;
    Some(Entry { number, verdict, reason: rest[word_end..].trim_start_matches(SEPARATORS).trim().to_string() })
}

/// `reason` on one line, cut to [`MAX_REASON_CHARS`].
fn cut(reason: &str) -> String {
    reason.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(MAX_REASON_CHARS).collect()
}
