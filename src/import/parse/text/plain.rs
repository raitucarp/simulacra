//! Plain text: blank lines separate paragraphs; hard-wrapped lines inside a paragraph are
//! joined, except short lines (verse, addresses) which keep their breaks.

use crate::import::document::Block;
use crate::import::parse::sections::{normalize_whitespace, sections_from_blocks};
use crate::import::parse::Content;

/// A paragraph whose lines are all shorter than this keeps its line breaks.
const VERSE_LINE_MAX_CHARS: usize = 48;

/// The paragraphs of `text`: runs of non-blank lines.
pub fn paragraphs(text: &str) -> Vec<Vec<&str>> {
    let mut out: Vec<Vec<&str>> = vec![Vec::new()];
    for line in text.lines().map(str::trim) {
        match (line.is_empty(), out.last_mut()) {
            (true, Some(current)) if !current.is_empty() => out.push(Vec::new()),
            (false, Some(current)) => current.push(line),
            _ => {}
        }
    }
    out.into_iter().filter(|lines| !lines.is_empty()).collect()
}

/// Joins a paragraph's lines: verse keeps its breaks, prose is joined with spaces.
pub fn join_paragraph(lines: &[&str]) -> String {
    let is_verse = lines.len() > 1 && lines.iter().all(|line| line.chars().count() < VERSE_LINE_MAX_CHARS);
    let joiner = if is_verse { "\n" } else { " " };
    lines.iter().map(|line| normalize_whitespace(line)).collect::<Vec<_>>().join(joiner)
}

/// Parses plain text into one section of paragraphs.
pub fn parse_plain(text: &str) -> Content {
    let blocks = paragraphs(text).iter().map(|lines| Block::paragraph(join_paragraph(lines))).collect();
    Content { sections: sections_from_blocks(blocks), ..Content::default() }
}
