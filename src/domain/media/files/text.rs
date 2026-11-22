//! L9: the first lines of a text or code file, and the language that colours them (the code
//! block highlighter's, chosen by the file's extension).

use super::scan::FilePreview;
use crate::domain::post::highlight::is_known_language;

/// Bytes read from the start of a text file.
pub const TEXT_PREVIEW_BYTES: usize = 8 * 1024;
/// Lines shown, and characters kept of each.
pub const TEXT_PREVIEW_LINES: usize = 8;
const MAX_LINE_CHARS: usize = 160;
const BYTE_ORDER_MARK: char = '\u{feff}';
const TAB: &str = "\t";
const TAB_AS_SPACES: &str = "    ";

/// The preview of a text file of `total` bytes whose first bytes are `head`.
pub fn text_preview(head: &[u8], total: u64, extension: Option<&str>) -> FilePreview {
    let valid = match std::str::from_utf8(head) {
        Ok(text) => text,
        Err(err) => std::str::from_utf8(&head[..err.valid_up_to()]).unwrap_or_default(),
    };
    let text = valid.trim_start_matches(BYTE_ORDER_MARK);
    let mut lines = text.lines();
    let shown: Vec<String> = lines.by_ref().take(TEXT_PREVIEW_LINES).map(clip_line).collect();
    let more = lines.next().is_some() || (head.len() as u64) < total;
    let lang = extension.map(str::to_ascii_lowercase).filter(|ext| is_known_language(ext));
    FilePreview::Text { lines: shown.join("\n"), lang, more }
}

/// A line with tabs widened and cut to [`MAX_LINE_CHARS`].
fn clip_line(line: &str) -> String {
    let line = line.trim_end_matches('\r').replace(TAB, TAB_AS_SPACES);
    match line.char_indices().nth(MAX_LINE_CHARS) {
        Some((cut, _)) => line[..cut].to_string(),
        None => line,
    }
}
