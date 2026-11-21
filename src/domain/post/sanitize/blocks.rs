//! One prose line of agent output: headings, rules, quotes, bullets and table rows.

use super::inline::sanitize_inline;

const HEADING_MARKER: char = '#';
const MAX_HEADING_LEVEL: usize = 6;
const QUOTE_MARKER: char = '>';
/// Characters a horizontal rule or setext underline is made of.
const RULE_CHARS: [char; 4] = ['-', '*', '_', '='];
/// Shortest rule (`---`).
const MIN_RULE_LEN: usize = 3;
/// Bullet markers rewritten to `-`.
const STAR_BULLETS: [&str; 2] = ["* ", "+ "];
const DASH_BULLET: &str = "- ";
const TABLE_PIPE: char = '|';
/// Characters of a table separator row besides pipes (`|:---|---:|`).
const TABLE_RULE_CHARS: [char; 4] = ['-', ':', ' ', '\t'];

/// The line as it goes into the post, or `None` when it is dropped.
pub(super) fn prose_line(line: &str) -> Option<String> {
    let indent = &line[..line.len() - line.trim_start().len()];
    let body = strip_quote(line.trim());
    if is_rule(body) || is_table_rule(body) {
        return None;
    }
    let body = heading_text(body).unwrap_or(body);
    let (bullet, rest) = match STAR_BULLETS.iter().find_map(|marker| body.strip_prefix(marker)) {
        Some(rest) => (DASH_BULLET, rest),
        None => ("", body),
    };
    Some(format!("{indent}{bullet}{}", sanitize_inline(rest)))
}

/// `body` without its leading `>` quote markers.
fn strip_quote(mut body: &str) -> &str {
    while let Some(rest) = body.strip_prefix(QUOTE_MARKER) {
        body = rest.trim_start();
    }
    body
}

/// The text of an ATX heading (`## Title ##`); `None` for anything else, a `#Tag` included.
fn heading_text(body: &str) -> Option<&str> {
    let level = body.chars().take_while(|&c| c == HEADING_MARKER).count();
    let rest = &body[level..];
    let is_heading = (1..=MAX_HEADING_LEVEL).contains(&level) && (rest.is_empty() || rest.starts_with(char::is_whitespace));
    is_heading.then(|| rest.trim().trim_end_matches(HEADING_MARKER).trim_end())
}

/// A horizontal rule or setext underline: one rule character repeated (spaces allowed).
fn is_rule(body: &str) -> bool {
    let marks: Vec<char> = body.chars().filter(|c| !c.is_whitespace()).collect();
    marks.len() >= MIN_RULE_LEN && RULE_CHARS.contains(&marks[0]) && marks.iter().all(|&c| c == marks[0])
}

/// A table separator row (`| --- | :---: |`).
fn is_table_rule(body: &str) -> bool {
    body.contains(TABLE_PIPE) && body.contains('-') && body.chars().all(|c| c == TABLE_PIPE || TABLE_RULE_CHARS.contains(&c))
}
