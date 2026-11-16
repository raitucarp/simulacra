//! Character-safe node labels.

/// Longest label, in characters, before truncation.
pub(super) const LABEL_MAX_CHARS: usize = 42;
/// Marker appended to a truncated label.
const ELLIPSIS: char = '…';

/// Collapses whitespace and truncates `text` to `max_chars` characters (ellipsis included),
/// never splitting a multi-byte character.
pub fn excerpt(text: &str, max_chars: usize) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= max_chars {
        return collapsed;
    }
    let keep = max_chars.saturating_sub(1);
    let mut out: String = collapsed.chars().take(keep).collect();
    out.push(ELLIPSIS);
    out
}
