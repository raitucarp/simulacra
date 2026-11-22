//! "Split this line" (T3): a free compose line longer than the per-post limit becomes several
//! lines, cut by [`split_into_thread`] on sentence and word boundaries, each keeping the
//! line's indentation and bullet so the pieces stay where the line was in the tree.

use super::lines::strip_bullet;
use crate::domain::split_into_thread;

/// The indentation and bullet (`  - `) that start `line`, and its text.
fn prefix_and_text(line: &str) -> (String, &str) {
    let trimmed = line.trim_start_matches([' ', '\t']);
    let indent = &line[..line.len() - trimmed.len()];
    match (strip_bullet(trimmed.trim_end()), trimmed.chars().next()) {
        (Some(text), Some(marker)) => (format!("{indent}{marker} "), text),
        _ => (indent.to_string(), trimmed.trim_end()),
    }
}

/// `text` with raw line `line` cut into pieces of at most `limit` characters, one line each;
/// unchanged when there is no such line or it already fits.
pub fn split_line(text: &str, line: usize, limit: usize) -> String {
    let mut lines: Vec<String> = text.split('\n').map(str::to_string).collect();
    let Some(raw) = lines.get(line).cloned() else { return text.to_string() };
    let (body, ending) = raw.strip_suffix('\r').map_or((raw.as_str(), ""), |body| (body, "\r"));
    let (prefix, words) = prefix_and_text(body);
    let pieces = split_into_thread(words, limit);
    if pieces.len() < 2 {
        return text.to_string();
    }
    let replaced: Vec<String> = pieces.into_iter().map(|piece| format!("{prefix}{piece}{ending}")).collect();
    lines.splice(line..=line, replaced);
    lines.join("\n")
}
