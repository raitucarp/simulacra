//! Reading one line of free compose text: its indentation level and its text.

/// Spaces that make up one indentation level.
const SPACES_PER_LEVEL: usize = 2;
/// Characters that start a bullet when followed by whitespace.
const BULLET_MARKERS: [char; 2] = ['-', '*'];

/// The indentation level of `line`: each tab is one level, each two spaces one level, and a
/// leftover single space rounds down. Only leading spaces and tabs count.
pub fn indent_level(line: &str) -> usize {
    let mut columns = 0;
    for c in line.chars() {
        match c {
            ' ' => columns += 1,
            '\t' => columns += SPACES_PER_LEVEL,
            _ => break,
        }
    }
    columns / SPACES_PER_LEVEL
}

/// The text after a bullet marker, when the trimmed `line` starts with one followed by
/// whitespace (or is a bare marker, giving empty text).
pub fn strip_bullet(line: &str) -> Option<&str> {
    let mut chars = line.chars();
    let marker = chars.next()?;
    if !BULLET_MARKERS.contains(&marker) {
        return None;
    }
    let rest = chars.as_str();
    let spaced = rest.chars().next().is_none_or(char::is_whitespace);
    spaced.then(|| rest.trim_start())
}

/// The level and text of a draft line, or `None` for blank lines and empty bullets.
pub(super) fn item(raw: &str) -> Option<(usize, &str)> {
    let line = raw.strip_suffix('\r').unwrap_or(raw);
    let level = indent_level(line);
    let trimmed = line.trim();
    let text = strip_bullet(trimmed).unwrap_or(trimmed).trim();
    (!text.is_empty()).then_some((level, text))
}
