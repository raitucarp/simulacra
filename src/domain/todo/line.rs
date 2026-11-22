//! Reading one line as a to-do item: indentation, optional bullet, box, text and tokens.

use super::tokens::TokenValue;
use super::{Span, TodoItem, TodoMeta, TodoToken};
use crate::domain::compose::{indent_level, strip_bullet};
use crate::domain::symbols::EntryState;

/// Byte length of a `[c]` box.
const BOX_LEN: usize = 3;

/// The indentation level and item of `line`, which starts at byte `offset` of the text and
/// has no line ending; `None` when the line is not a to-do.
pub(super) fn parse_line(line: &str, offset: usize) -> Option<(usize, TodoItem)> {
    let indent = line.len() - line.trim_start().len();
    let body = line.trim();
    let after_bullet = strip_bullet(body).unwrap_or(body);
    let box_start = indent + (body.len() - after_bullet.len());
    let state = box_state(after_bullet)?;
    let rest = &after_bullet[BOX_LEN..];
    let (text, tokens, meta) = scan(rest, offset + box_start + BOX_LEN);
    let item = TodoItem {
        state,
        text,
        line_span: offset..offset + line.len(),
        marker_span: offset + box_start..offset + box_start + BOX_LEN,
        tokens,
        meta,
        children: Vec::new(),
    };
    Some((indent_level(line), item))
}

/// The state a box at the start of `text` stands for, when it is followed by whitespace or
/// the end of the line.
fn box_state(text: &str) -> Option<EntryState> {
    let mark = text.get(..BOX_LEN)?;
    let spaced = text[BOX_LEN..].chars().next().is_none_or(char::is_whitespace);
    let state = match mark {
        "[ ]" => EntryState::Open,
        "[x]" | "[X]" => EntryState::Done,
        "[>]" => EntryState::Migrated,
        "[<]" => EntryState::Scheduled,
        "[~]" => EntryState::Dropped,
        "[/]" => EntryState::Delegated,
        _ => return None,
    };
    spaced.then_some(state)
}

/// Splits the item text into plain words and tokens; `base` is the byte offset of `text`.
fn scan(text: &str, base: usize) -> (String, Vec<TodoToken>, TodoMeta) {
    let mut words = Vec::new();
    let mut tokens = Vec::new();
    let mut meta = TodoMeta::default();
    for (start, word) in joined_due_words(text, words_with_offsets(text)) {
        match TokenValue::parse(word) {
            Some(value) if meta.accept(value) => {
                let span: Span = base + start..base + start + word.len();
                tokens.push(TodoToken { kind: value.kind(), span });
            }
            _ => words.push(word),
        }
    }
    (words.join(" "), tokens, meta)
}

/// Whitespace-separated words of `text` with their byte offsets.
fn words_with_offsets(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in text.char_indices() {
        if !c.is_whitespace() {
            start.get_or_insert(i);
        } else if let Some(begin) = start.take() {
            out.push((begin, &text[begin..i]));
        }
    }
    if let Some(begin) = start {
        out.push((begin, &text[begin..]));
    }
    out
}

/// `words` with `^next` and the word completing it (`^next month`) joined into one (D3).
fn joined_due_words<'a>(text: &'a str, words: Vec<(usize, &'a str)>) -> Vec<(usize, &'a str)> {
    let mut out: Vec<(usize, &'a str)> = Vec::with_capacity(words.len());
    for (start, word) in words {
        let joins = out.last().is_some_and(|(_, last)| {
            let phrase = last.strip_prefix('^').filter(|p| !p.starts_with('^') && super::due::takes_next_word(p));
            phrase.is_some_and(|p| super::due::resolve_due(&format!("{p} {word}"), jiff::civil::date(2000, 1, 1)).is_some())
        });
        match out.last_mut() {
            Some((first, joined)) if joins => *joined = &text[*first..start + word.len()],
            _ => out.push((start, word)),
        }
    }
    out
}

impl TodoMeta {
    /// Records `value`; `false` when a single-valued slot is already taken.
    fn accept(&mut self, value: TokenValue<'_>) -> bool {
        match value {
            TokenValue::Due(due) => set_once(&mut self.due, due.to_string()),
            TokenValue::Repeat(rule) => set_once(&mut self.repeat, rule.to_string()),
            TokenValue::Priority(level) => set_once(&mut self.priority, level),
            TokenValue::Estimate(minutes) => set_once(&mut self.estimate_minutes, minutes),
            TokenValue::Delegate(name) => set_once(&mut self.delegate, name.to_string()),
            TokenValue::Context(name) => {
                if !self.contexts.iter().any(|c| c == name) {
                    self.contexts.push(name.to_string());
                }
                true
            }
        }
    }
}

fn set_once<T>(slot: &mut Option<T>, value: T) -> bool {
    if slot.is_some() {
        return false;
    }
    *slot = Some(value);
    true
}

/// Whether `line` (without its line ending) is a to-do line.
pub(super) fn is_todo_line(line: &str) -> bool {
    parse_line(line, 0).is_some()
}
