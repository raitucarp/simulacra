//! The stored text as the reader shows it: blocks (headings, paragraphs, quotations), the
//! outline (table of contents) of its headings, and an estimated reading time.

#[cfg(test)]
mod tests;

/// Markdown heading marker, quotation marker and the deepest heading level kept.
const HEADING_MARKER: char = '#';
const QUOTE_MARKER: char = '>';
const MAX_HEADING_LEVEL: u8 = 4;
/// Average silent reading speed used for the time estimate (words per minute).
const WORDS_PER_MINUTE: usize = 230;
/// Fewest headings for which an outline is worth showing.
pub const MIN_OUTLINE_ENTRIES: usize = 2;

/// What a block of text is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    /// A heading of level 1 (largest) to 4.
    Heading(u8),
    Paragraph,
    Quote,
}

/// One block of the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBlock {
    pub kind: BlockKind,
    pub text: String,
}

/// One heading of the outline and the index of its block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlineEntry {
    pub title: String,
    pub level: u8,
    pub block: usize,
}

/// A line as a heading (`## Title` → level 2), when it is one.
fn heading(line: &str) -> Option<TextBlock> {
    let hashes = line.chars().take_while(|c| *c == HEADING_MARKER).count();
    let rest = line[hashes..].trim();
    (hashes > 0 && line[hashes..].starts_with(' ') && !rest.is_empty()).then(|| TextBlock {
        kind: BlockKind::Heading((hashes as u8).min(MAX_HEADING_LEVEL)),
        text: rest.to_string(),
    })
}

/// The pending prose lines as one paragraph (or quotation when every line is quoted).
fn flush_prose(prose: &mut Vec<&str>, out: &mut Vec<TextBlock>) {
    if prose.is_empty() {
        return;
    }
    let quoted = prose.iter().all(|line| line.starts_with(QUOTE_MARKER));
    let text = prose.iter().map(|line| line.trim_start_matches(QUOTE_MARKER).trim()).collect::<Vec<_>>().join(" ");
    out.push(TextBlock { kind: if quoted { BlockKind::Quote } else { BlockKind::Paragraph }, text });
    prose.clear();
}

/// A run of non-blank lines as blocks: headings on their own, the lines between them joined.
fn chunk_blocks(lines: &[&str]) -> Vec<TextBlock> {
    let mut out = Vec::new();
    let mut prose: Vec<&str> = Vec::new();
    for line in lines {
        match heading(line) {
            Some(block) => {
                flush_prose(&mut prose, &mut out);
                out.push(block);
            }
            None => prose.push(line),
        }
    }
    flush_prose(&mut prose, &mut out);
    out
}

/// The blocks of `content` (blank lines separate paragraphs).
pub fn blocks(content: &str) -> Vec<TextBlock> {
    let lines: Vec<&str> = content.lines().map(str::trim).collect();
    lines.split(|line| line.is_empty()).filter(|run| !run.is_empty()).flat_map(chunk_blocks).collect()
}

/// The headings of `blocks`, in order.
pub fn outline(blocks: &[TextBlock]) -> Vec<OutlineEntry> {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(block, b)| match b.kind {
            BlockKind::Heading(level) => Some(OutlineEntry { title: b.text.clone(), level, block }),
            _ => None,
        })
        .collect()
}

/// Minutes needed to read `words` words (at least one for any text).
pub fn reading_minutes(words: usize) -> usize {
    if words == 0 {
        0
    } else {
        words.div_ceil(WORDS_PER_MINUTE)
    }
}
