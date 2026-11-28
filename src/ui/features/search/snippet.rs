//! The passage a result shows (Q4): the post's text on one plain line, cut to a window that opens
//! a little before its first match, with every matching word marked.

use crate::domain::document::query::MIN_TERM_CHARS;
use crate::domain::search_query::PostQuery;
use crate::storage::db::search_terms;
use std::ops::Range;

/// Characters a snippet shows at most.
pub const SNIPPET_CHARS: usize = 220;
/// Characters kept before the first match.
const LEAD_CHARS: usize = 48;
const ELLIPSIS: &str = "…";

/// A shortened passage and the byte ranges of its matching words.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snippet {
    pub text: String,
    pub highlights: Vec<Range<usize>>,
}

/// The terms a passage marks: the query's words (as prefixes) and its phrases' terms.
fn wanted(query: &PostQuery) -> (Vec<String>, Vec<String>) {
    let words = query.words.iter().flat_map(|word| search_terms(word).collect::<Vec<_>>()).filter(|term| term.chars().count() >= MIN_TERM_CHARS).collect();
    let phrases = query.phrases.iter().flat_map(|phrase| search_terms(phrase).collect::<Vec<_>>()).collect();
    (words, phrases)
}

/// Byte ranges of the letter-and-digit runs of `text`.
fn runs(text: &str) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut start = None;
    for (at, c) in text.char_indices() {
        match (c.is_alphanumeric(), start) {
            (true, None) => start = Some(at),
            (false, Some(begin)) => {
                out.push(begin..at);
                start = None;
            }
            _ => {}
        }
    }
    out.extend(start.map(|begin| begin..text.len()));
    out
}

/// The passage of `content` for `query` (see the module rules), read as plain text: no
/// markup and no `[[post_…]]` ids.
pub fn snippet(content: &str, query: &PostQuery) -> Snippet {
    let line = crate::domain::post::readable_line(content);
    let (words, phrases) = wanted(query);
    let marks: Vec<Range<usize>> = runs(&line)
        .into_iter()
        .filter(|run| {
            let term = line[run.clone()].to_lowercase();
            words.iter().any(|word| term.starts_with(word.as_str())) || phrases.contains(&term)
        })
        .collect();
    let start = marks.first().map_or(0, |first| window_start(&line, first.start));
    let end = line[start..].char_indices().nth(SNIPPET_CHARS).map_or(line.len(), |(at, _)| start + at);
    let lead = if start > 0 { ELLIPSIS } else { "" };
    let tail = if end < line.len() { ELLIPSIS } else { "" };
    let shift = |at: usize| at - start + lead.len();
    let highlights = marks.into_iter().filter(|mark| mark.start >= start && mark.end <= end).map(|mark| shift(mark.start)..shift(mark.end)).collect();
    Snippet { text: format!("{lead}{}{tail}", &line[start..end]), highlights }
}

/// Where a window opening [`LEAD_CHARS`] before byte `first` starts: at a word's start.
fn window_start(line: &str, first: usize) -> usize {
    let Some((cut, _)) = line[..first].char_indices().rev().nth(LEAD_CHARS - 1) else { return 0 };
    line[cut..first].find(' ').map_or(cut, |space| cut + space + 1)
}
