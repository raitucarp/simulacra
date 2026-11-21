//! Toggling a mark on a selection over several lines: each line on its own, or a fenced
//! block for code.

use super::wrap::{inside_len, trimmed, wrapped};
use super::{FormatEdit, FormatMark};
use std::ops::Range;

pub(super) const LINE_BREAK: char = '\n';
/// Fence line opening and closing a code block.
const FENCE: &str = "```";

/// Toggles `mark` over a selection that holds at least one line break.
pub(super) fn toggle_lines(text: &str, selection: Range<usize>, mark: FormatMark) -> FormatEdit {
    if mark == FormatMark::Code {
        return toggle_fence(text, selection);
    }
    let spans: Vec<Range<usize>> = line_spans(text, &selection).into_iter().map(|line| trimmed(text, line)).filter(|span| !span.is_empty()).collect();
    let unwrap = !spans.is_empty() && spans.iter().all(|span| inside_len(&text[span.clone()], mark).is_some());
    let mut out = String::new();
    let mut cursor = selection.start;
    for span in spans {
        out.push_str(&text[cursor..span.start]);
        let line = &text[span.clone()];
        match inside_len(line, mark) {
            Some(len) if unwrap => out.push_str(&line[len..line.len() - len]),
            Some(_) => out.push_str(line),
            None => out.push_str(&wrapped(line, mark)),
        }
        cursor = span.end;
    }
    out.push_str(&text[cursor..selection.end]);
    let end = selection.start + out.len();
    FormatEdit { range: selection.clone(), text: out, selection: selection.start..end }
}

/// The part of each line of `text` that `selection` covers.
fn line_spans(text: &str, selection: &Range<usize>) -> Vec<Range<usize>> {
    let mut spans = Vec::new();
    let mut start = selection.start;
    for line in text[selection.clone()].split(LINE_BREAK) {
        spans.push(start..start + line.len());
        start += line.len() + LINE_BREAK.len_utf8();
    }
    spans
}

/// Whole lines touched by `selection`: from the start of its first line to the end of its last.
fn whole_lines(text: &str, selection: &Range<usize>) -> Range<usize> {
    let start = text[..selection.start].rfind(LINE_BREAK).map_or(0, |at| at + 1);
    let end = text[selection.end..].find(LINE_BREAK).map_or(text.len(), |at| selection.end + at);
    start..end
}

/// Wraps the touched lines in a fenced block, or removes the fences when they are its first
/// and last lines.
fn toggle_fence(text: &str, selection: Range<usize>) -> FormatEdit {
    let range = whole_lines(text, &selection);
    let block = &text[range.clone()];
    let lines: Vec<&str> = block.split(LINE_BREAK).collect();
    let fenced = lines.len() >= 2 && lines[0].trim_start().starts_with(FENCE) && lines[lines.len() - 1].trim() == FENCE;
    let new = if fenced {
        lines[1..lines.len() - 1].join("\n")
    } else {
        format!("{FENCE}{LINE_BREAK}{block}{LINE_BREAK}{FENCE}")
    };
    let selection = if fenced {
        range.start..range.start + new.len()
    } else {
        let code_start = range.start + FENCE.len() + LINE_BREAK.len_utf8();
        code_start..code_start + block.len()
    };
    FormatEdit { range, text: new, selection }
}
