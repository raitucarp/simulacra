//! Turns matched character positions into byte ranges for highlighted text.

use std::ops::Range;

/// Byte ranges of `text` covering the characters at `positions` (sorted char indices),
/// adjacent characters merged into one range. Positions past the end are ignored.
pub fn highlight_ranges(text: &str, positions: &[usize]) -> Vec<Range<usize>> {
    let mut wanted = positions.iter().copied().peekable();
    text.char_indices().enumerate().fold(Vec::new(), |mut ranges: Vec<Range<usize>>, (index, (byte, c))| {
        while wanted.peek().is_some_and(|&at| at < index) {
            wanted.next();
        }
        if wanted.peek() == Some(&index) {
            let end = byte + c.len_utf8();
            match ranges.last_mut() {
                Some(last) if last.end == byte => last.end = end,
                _ => ranges.push(byte..end),
            }
        }
        ranges
    })
}
