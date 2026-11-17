//! Sentence splitting for chunking: a sentence ends at terminal punctuation followed by
//! whitespace (or the end), or at a line break.

/// Punctuation that ends a sentence (Latin, ellipsis, CJK full-width).
const TERMINALS: [char; 7] = ['.', '!', '?', '…', '。', '！', '？'];
/// Full-width terminals end a sentence even without a following space (CJK text has none).
const FULL_WIDTH_TERMINALS: [char; 3] = ['。', '！', '？'];
/// Closing marks that stay with the sentence they end (`"Why?"`, `(sic.)`).
const CLOSERS: [char; 5] = ['"', '\'', ')', '”', '’'];

/// The trimmed, non-empty sentences of `text`, in order.
pub fn split_sentences(text: &str) -> Vec<&str> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut sentences = Vec::new();
    let mut start = 0;
    for (position, &(at, c)) in chars.iter().enumerate() {
        let end = if c == '\n' {
            Some(at)
        } else if TERMINALS.contains(&c) && (FULL_WIDTH_TERMINALS.contains(&c) || ends_here(&chars, position)) {
            Some(closing_end(text, &chars, position))
        } else {
            None
        };
        if let Some(end) = end.filter(|&end| end >= start) {
            sentences.push(&text[start..end]);
            start = end;
        }
    }
    sentences.push(&text[start..]);
    sentences.into_iter().map(str::trim).filter(|s| !s.is_empty()).collect()
}

/// Whether the terminal at `position` ends a sentence: the next non-closer is whitespace or
/// the end of the text (so `3.14` and `e.g.x` do not split).
fn ends_here(chars: &[(usize, char)], position: usize) -> bool {
    chars[position + 1..]
        .iter()
        .map(|&(_, c)| c)
        .find(|c| !CLOSERS.contains(c) && !TERMINALS.contains(c))
        .is_none_or(char::is_whitespace)
}

/// Byte offset just past the terminal run and its closing marks.
fn closing_end(text: &str, chars: &[(usize, char)], position: usize) -> usize {
    chars[position + 1..]
        .iter()
        .find(|&&(_, c)| !CLOSERS.contains(&c) && !TERMINALS.contains(&c))
        .map_or(text.len(), |&(at, _)| at)
}
