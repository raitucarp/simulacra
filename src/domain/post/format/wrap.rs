//! Toggling a mark on a selection within one line.

use super::{FormatEdit, FormatMark};
use std::ops::Range;

const STAR: char = '*';
const BACKTICK: char = '`';
/// Padding inside a code span whose text starts or ends with a backtick.
const CODE_PADDING: char = ' ';

/// Toggles `mark` on `selection` (no line break inside).
pub(super) fn toggle_span(text: &str, selection: Range<usize>, mark: FormatMark) -> FormatEdit {
    let span = trimmed(text, selection);
    if let Some(len) = outside_len(text, &span, mark) {
        let range = span.start - len..span.end + len;
        return FormatEdit { range, text: text[span.clone()].to_string(), selection: span.start - len..span.end - len };
    }
    if let Some(len) = inside_len(&text[span.clone()], mark) {
        let inner = &text[span.start + len..span.end - len];
        return FormatEdit { range: span.clone(), text: inner.to_string(), selection: span.start..span.start + inner.len() };
    }
    let wrapped = wrapped(&text[span.clone()], mark);
    let open = (wrapped.len() - span.len()) / 2;
    FormatEdit { selection: span.start + open..span.end + open, range: span, text: wrapped }
}

/// `selection` without its leading and trailing whitespace (empty at its start when blank).
pub(super) fn trimmed(text: &str, selection: Range<usize>) -> Range<usize> {
    let inner = &text[selection.clone()];
    let start = selection.start + (inner.len() - inner.trim_start().len());
    let end = selection.end - (inner.len() - inner.trim_end().len());
    if start >= end {
        selection.start..selection.start
    } else {
        start..end
    }
}

/// `inner` between the delimiters of `mark`.
pub(super) fn wrapped(inner: &str, mark: FormatMark) -> String {
    if mark != FormatMark::Code {
        let delimiter = mark.delimiter();
        return format!("{delimiter}{inner}{delimiter}");
    }
    let fence = BACKTICK.to_string().repeat(longest_run(inner, BACKTICK) + 1);
    let pad = if inner.starts_with(BACKTICK) || inner.ends_with(BACKTICK) { CODE_PADDING.to_string() } else { String::new() };
    format!("{fence}{pad}{inner}{pad}{fence}")
}

/// Delimiter length when `span` is wrapped by `mark` just outside it.
fn outside_len(text: &str, span: &Range<usize>, mark: FormatMark) -> Option<usize> {
    let before = &text[..span.start];
    let after = &text[span.end..];
    delimiter_len(mark, |c| (trailing_run(before, c), leading_run(after, c)))
}

/// Delimiter length when `span` itself starts and ends with the delimiters of `mark`.
pub(super) fn inside_len(span: &str, mark: FormatMark) -> Option<usize> {
    let len = delimiter_len(mark, |c| (leading_run(span, c), trailing_run(span, c)))?;
    (span.len() >= 2 * len).then_some(len)
}

/// How many delimiter characters `mark` takes off each side, given the runs on each side.
fn delimiter_len(mark: FormatMark, runs: impl Fn(char) -> (usize, usize)) -> Option<usize> {
    match mark {
        FormatMark::Italic => {
            let (a, b) = runs(STAR);
            (a % 2 == 1 && b % 2 == 1).then_some(1)
        }
        FormatMark::Bold => {
            let (a, b) = runs(STAR);
            (a >= 2 && b >= 2).then_some(2)
        }
        FormatMark::Code => {
            let (a, b) = runs(BACKTICK);
            (a == b && a > 0).then_some(a)
        }
        FormatMark::Underline | FormatMark::Spoiler => {
            let delimiter = mark.delimiter();
            let (a, b) = runs(delimiter.chars().next().unwrap_or_default());
            (a >= delimiter.len() && b >= delimiter.len()).then_some(delimiter.len())
        }
    }
}

fn leading_run(text: &str, c: char) -> usize {
    text.chars().take_while(|&x| x == c).count()
}

fn trailing_run(text: &str, c: char) -> usize {
    text.chars().rev().take_while(|&x| x == c).count()
}

fn longest_run(text: &str, c: char) -> usize {
    text.split(|x| x != c).map(str::len).max().unwrap_or(0)
}
