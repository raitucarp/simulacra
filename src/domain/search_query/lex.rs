//! Splitting a query into raw parts: whitespace separates them except inside quotes.

/// Straight and curly double quotes all open and close a quoted run.
pub(super) const QUOTES: [char; 3] = ['"', '\u{201C}', '\u{201D}'];

/// The raw parts of `input`, quotes kept.
pub(super) fn lex(input: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = None;
    let mut quoted = false;
    for (i, c) in input.char_indices() {
        if QUOTES.contains(&c) {
            quoted = !quoted;
            start.get_or_insert(i);
        } else if !c.is_whitespace() || quoted {
            start.get_or_insert(i);
        } else if let Some(begin) = start.take() {
            out.push(&input[begin..i]);
        }
    }
    if let Some(begin) = start {
        out.push(&input[begin..]);
    }
    out
}

/// `text` without one surrounding pair of quotes (either end may be missing).
pub(super) fn unquote(text: &str) -> &str {
    let text = text.strip_prefix(QUOTES).unwrap_or(text);
    text.strip_suffix(QUOTES).unwrap_or(text)
}

/// Whether `text` starts with a quote.
pub(super) fn is_quoted(text: &str) -> bool {
    text.starts_with(QUOTES)
}
