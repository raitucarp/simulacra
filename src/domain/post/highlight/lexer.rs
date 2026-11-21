//! One left-to-right pass over the code: comments, strings, numbers and words.

use super::grammar::Grammar;
use super::{CodeHighlight, CodeStyle};

const LINE_BREAK: char = '\n';
const ESCAPE: char = '\\';
const CHAR_QUOTE: char = '\'';
/// Longest one-character literal with an escape (`'\u{10FFFF}'`).
const MAX_ESCAPED_CHAR_LEN: usize = 12;

/// The highlights of `code` under `grammar`, in order.
pub(super) fn scan(grammar: &Grammar, code: &str) -> Vec<CodeHighlight> {
    let mut out = Vec::new();
    let mut at = 0;
    while let Some(c) = code[at..].chars().next() {
        let rest = &code[at..];
        let token = comment(grammar, rest)
            .map(|len| (len, Some(CodeStyle::Comment)))
            .or_else(|| string(grammar, rest, c).map(|len| (len, Some(CodeStyle::String))))
            .unwrap_or_else(|| word(grammar, rest, c, previous_char(code, at)));
        let (len, style) = token;
        if let Some(style) = style {
            out.push((at..at + len, style));
        }
        at += len.max(c.len_utf8());
    }
    out
}

fn previous_char(code: &str, at: usize) -> Option<char> {
    code[..at].chars().next_back()
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Length of a comment starting at `rest` (to the line end, or to the block closer).
fn comment(grammar: &Grammar, rest: &str) -> Option<usize> {
    if grammar.line_comments.iter().any(|marker| rest.starts_with(marker)) {
        return Some(rest.find(LINE_BREAK).unwrap_or(rest.len()));
    }
    let (open, close) = grammar.block_comment?;
    let body = rest.strip_prefix(open)?;
    Some(open.len() + body.find(close).map_or(body.len(), |end| end + close.len()))
}

/// Length of a string literal opened by `quote` at `rest`; an unclosed one runs to the line end.
fn string(grammar: &Grammar, rest: &str, quote: char) -> Option<usize> {
    if quote == CHAR_QUOTE && grammar.char_literals {
        return char_literal(rest);
    }
    if !grammar.quotes.contains(&quote) {
        return None;
    }
    let multiline = grammar.multiline_quotes.contains(&quote);
    let mut escaped = false;
    for (offset, c) in rest.char_indices().skip(1) {
        match c {
            _ if escaped => escaped = false,
            ESCAPE => escaped = true,
            LINE_BREAK if !multiline => return Some(offset),
            _ if c == quote => return Some(offset + c.len_utf8()),
            _ => {}
        }
    }
    Some(rest.len())
}

/// `'a'` or `'\n'`: a quote, one (possibly escaped) character and a closing quote.
fn char_literal(rest: &str) -> Option<usize> {
    let mut chars = rest.char_indices().skip(1);
    let (_, first) = chars.next()?;
    if first == ESCAPE {
        let close = rest[1..].char_indices().skip(2).find(|&(_, c)| c == CHAR_QUOTE)?.0 + 1;
        return (close < MAX_ESCAPED_CHAR_LEN).then_some(close + 1);
    }
    let (offset, second) = chars.next()?;
    (second == CHAR_QUOTE && first != CHAR_QUOTE).then_some(offset + 1)
}

/// A number, keyword, type or plain word at `rest`, with its length.
fn word(grammar: &Grammar, rest: &str, c: char, previous: Option<char>) -> (usize, Option<CodeStyle>) {
    if !is_word_char(c) || previous.is_some_and(is_word_char) {
        return (c.len_utf8(), None);
    }
    if c.is_ascii_digit() {
        return (number_len(rest), Some(CodeStyle::Number));
    }
    let len = rest.find(|ch: char| !is_word_char(ch)).unwrap_or(rest.len());
    let text = &rest[..len];
    let style = if grammar.is_keyword(text) {
        Some(CodeStyle::Keyword)
    } else if grammar.is_type(text) {
        Some(CodeStyle::Type)
    } else {
        None
    };
    (len, style)
}

/// Digits, letters and `_` (`0xFF`, `1_000u32`), and a `.` followed by a digit (`3.14`).
fn number_len(rest: &str) -> usize {
    let bytes = rest.as_bytes();
    let mut len = 0;
    while len < bytes.len() {
        let b = bytes[len];
        let decimal_point = b == b'.' && bytes.get(len + 1).is_some_and(u8::is_ascii_digit);
        if !(b.is_ascii_alphanumeric() || b == b'_' || decimal_point) {
            break;
        }
        len += 1;
    }
    len
}
