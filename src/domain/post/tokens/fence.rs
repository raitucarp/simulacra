//! Fenced code blocks: a line of three or more backticks with an optional language tag opens
//! one; the next line of at least as many backticks (and nothing else) closes it.
//!
//! An opener without a closer is ordinary prose. The line breaks next to a block belong to it.

/// Fence character.
const BACKTICK: char = '`';
/// Shortest backtick run that opens a fence.
const MIN_FENCE_RUN: usize = 3;
const LINE_BREAK: char = '\n';
const CARRIAGE_RETURN: char = '\r';
/// Characters allowed in a language tag besides alphanumerics (`c++`, `c#`, `objective-c`).
const TAG_PUNCTUATION: [char; 5] = ['+', '#', '-', '_', '.'];

/// A run of prose or one fenced code block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Segment<'a> {
    Prose(&'a str),
    Code { lang: Option<String>, code: String },
}

/// Splits `content` into prose and fenced code blocks, in order.
pub(super) fn segments(content: &str) -> Vec<Segment<'_>> {
    let lines = line_spans(content);
    let mut out = Vec::new();
    let mut prose_start = 0;
    let mut k = 0;
    while k < lines.len() {
        let block = opener(lines[k].1)
            .and_then(|(lang, run)| (k + 1..lines.len()).find(|&j| is_closer(lines[j].1, run)).map(|close| (lang, close)));
        let Some((lang, close)) = block else {
            k += 1;
            continue;
        };
        push_prose(strip_line_end(&content[prose_start..lines[k].0]), &mut out);
        let code: Vec<&str> = lines[k + 1..close].iter().map(|(_, line)| strip_line_end(line)).collect();
        out.push(Segment::Code { lang, code: code.join("\n") });
        prose_start = lines[close].0 + lines[close].1.len();
        k = close + 1;
    }
    push_prose(&content[prose_start..], &mut out);
    out
}

fn push_prose<'a>(text: &'a str, out: &mut Vec<Segment<'a>>) {
    if !text.is_empty() {
        out.push(Segment::Prose(text));
    }
}

/// Every line with its byte offset; each line keeps its line break.
fn line_spans(content: &str) -> Vec<(usize, &str)> {
    let mut offset = 0;
    content
        .split_inclusive(LINE_BREAK)
        .map(|line| {
            let start = offset;
            offset += line.len();
            (start, line)
        })
        .collect()
}

/// `text` without one trailing line break (`\n` or `\r\n`).
fn strip_line_end(text: &str) -> &str {
    let text = text.strip_suffix(LINE_BREAK).unwrap_or(text);
    text.strip_suffix(CARRIAGE_RETURN).unwrap_or(text)
}

/// The language tag and backtick count of an opening fence line.
fn opener(line: &str) -> Option<(Option<String>, usize)> {
    let fence = strip_line_end(line).trim_start();
    let run = fence.chars().take_while(|&c| c == BACKTICK).count();
    let info = fence[run..].trim();
    (run >= MIN_FENCE_RUN && !info.contains(BACKTICK)).then(|| (language_tag(info), run))
}

/// Whether `line` closes a fence opened with `run` backticks.
fn is_closer(line: &str, run: usize) -> bool {
    let fence = strip_line_end(line).trim();
    fence.len() >= run && fence.chars().all(|c| c == BACKTICK)
}

/// The lower-cased first word of the info string; `None` (plain) when missing or malformed.
pub(super) fn language_tag(info: &str) -> Option<String> {
    let word = info.split_whitespace().next()?;
    let valid = word.chars().all(|c| c.is_alphanumeric() || TAG_PUNCTUATION.contains(&c));
    valid.then(|| word.to_lowercase())
}
