//! Lexical syntax highlighting of fenced code blocks: comments, strings, numbers, keywords
//! and type names, from small per-language tables (see
//! `docs/decisions/B5-syntax-highlighter.md` for why this is not a parser library).
//!
//! Output is byte ranges into the code with a [`CodeStyle`]; the UI maps each style to a
//! theme token. An unknown or missing language yields no highlights (plain monospace).

mod grammar;
mod languages;
mod lexer;

use std::ops::Range;

/// What a highlighted run of code is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodeStyle {
    Keyword,
    Type,
    String,
    Number,
    Comment,
}

/// One highlighted byte range of the code.
pub type CodeHighlight = (Range<usize>, CodeStyle);

/// Line break between the rows of a code block.
const LINE_BREAK: char = '\n';

/// Whether `lang` (a lower-cased fence tag) names a language the highlighter knows.
pub fn is_known_language(lang: &str) -> bool {
    languages::grammar_for(lang).is_some()
}

/// Highlights of `code` written in `lang`, ordered and non-overlapping.
pub fn highlight(lang: Option<&str>, code: &str) -> Vec<CodeHighlight> {
    match lang.and_then(languages::grammar_for) {
        Some(grammar) => lexer::scan(grammar, code),
        None => Vec::new(),
    }
}

/// The highlights of each line of `code` (split at `\n`), relative to the line start.
pub fn highlight_lines(lang: Option<&str>, code: &str) -> Vec<Vec<CodeHighlight>> {
    let spans = highlight(lang, code);
    let mut start = 0;
    code.split(LINE_BREAK)
        .map(|line| {
            let line_range = start..start + line.len();
            start = line_range.end + LINE_BREAK.len_utf8();
            clip(&spans, &line_range)
        })
        .collect()
}

/// The parts of `spans` inside `line`, shifted to start at the line.
fn clip(spans: &[CodeHighlight], line: &Range<usize>) -> Vec<CodeHighlight> {
    spans
        .iter()
        .filter_map(|(range, style)| {
            let start = range.start.max(line.start);
            let end = range.end.min(line.end);
            (start < end).then(|| (start - line.start..end - line.start, *style))
        })
        .collect()
}

#[cfg(test)]
mod tests;
