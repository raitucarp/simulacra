//! Right-to-left text for a renderer that only lays text out left to right (T21): the window
//! draws every string in the order of its characters, so Arabic came out backwards and with
//! its letters unjoined. [`visual_order`] joins the letters ([`super::arabic::shape`]), then
//! runs the Unicode bidirectional algorithm on a right-to-left paragraph and writes each run
//! in the order it is seen: Arabic runs reversed (marks kept on their letter, brackets
//! mirrored), Latin words and numbers as they are. `%{name}` placeholders are kept whole so
//! the translation can still be filled in afterwards.

use super::arabic::{is_mark, shape};
use std::borrow::Cow;
use unicode_bidi::{BidiInfo, Level};

/// First private-use code point standing in for a placeholder (a strong left-to-right
/// character to the bidi algorithm, so it stays one unbroken unit).
const SLOT_BASE: u32 = 0xE000;
const PLACEHOLDER_OPEN: &str = "%{";
const PLACEHOLDER_CLOSE: char = '}';

/// Arabic script (letters, punctuation and presentation forms).
fn is_arabic(c: char) -> bool {
    matches!(c, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{FB50}'..='\u{FDFF}' | '\u{FE70}'..='\u{FEFF}')
}

/// `text` as the left-to-right renderer must draw it to read right to left; unchanged (and
/// not copied) when it has no Arabic in it.
pub fn visual_order(text: &str) -> Cow<'_, str> {
    if !text.chars().any(is_arabic) {
        return Cow::Borrowed(text);
    }
    let (protected, slots) = protect(text);
    let lines: Vec<String> = protected.split('\n').map(visual_line).collect();
    Cow::Owned(restore(&lines.join("\n"), &slots))
}

/// `text` with each `%{…}` replaced by one private-use character, and the placeholders.
fn protect(text: &str) -> (String, Vec<String>) {
    let (mut out, mut slots, mut rest) = (String::with_capacity(text.len()), Vec::new(), text);
    while let Some(start) = rest.find(PLACEHOLDER_OPEN) {
        let Some(len) = rest[start..].find(PLACEHOLDER_CLOSE) else { break };
        let slot = u32::try_from(slots.len()).ok().and_then(|n| char::from_u32(SLOT_BASE + n));
        let Some(slot) = slot else { break };
        out.push_str(&rest[..start]);
        out.push(slot);
        slots.push(rest[start..=start + len].to_string());
        rest = &rest[start + len + 1..];
    }
    out.push_str(rest);
    (out, slots)
}

fn restore(text: &str, slots: &[String]) -> String {
    text.chars()
        .map(|c| {
            let index = (c as u32).checked_sub(SLOT_BASE).and_then(|n| slots.get(n as usize));
            index.cloned().unwrap_or_else(|| c.to_string())
        })
        .collect()
}

fn visual_line(line: &str) -> String {
    let shaped: String = shape(&line.chars().collect::<Vec<_>>()).into_iter().collect();
    let info = BidiInfo::new(&shaped, Some(Level::rtl()));
    let Some(paragraph) = info.paragraphs.first() else { return shaped };
    let (levels, runs) = info.visual_runs(paragraph, paragraph.range.clone());
    let mut out = String::with_capacity(shaped.len());
    for run in runs {
        let text = &shaped[run.clone()];
        if levels[run.start].is_rtl() {
            push_reversed(text, &mut out);
        } else {
            out.push_str(text);
        }
    }
    out
}

/// `text` reversed letter by letter, each mark staying after its letter, brackets mirrored.
fn push_reversed(text: &str, out: &mut String) {
    let mut clusters: Vec<Vec<char>> = Vec::new();
    for c in text.chars() {
        match clusters.last_mut() {
            Some(cluster) if is_mark(c) => cluster.push(c),
            _ => clusters.push(vec![c]),
        }
    }
    for cluster in clusters.iter().rev() {
        out.extend(cluster.iter().enumerate().map(|(at, c)| if at == 0 { mirrored(*c) } else { *c }));
    }
}

fn mirrored(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '«' => '»',
        '»' => '«',
        other => other,
    }
}
