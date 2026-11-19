//! Markdown: ATX headings, block quotes, list items, fenced code and paragraphs; links and
//! images keep their text, emphasis markers are dropped. Front matter is skipped.

use crate::import::document::{Block, BlockKind};
use crate::import::parse::sections::{normalize_whitespace, sections_from_blocks};
use crate::import::parse::Content;

const FENCE: &str = "```";
const FRONT_MATTER: &str = "---";
const LIST_MARKERS: [&str; 3] = ["- ", "* ", "+ "];
/// Emphasis and code markers dropped from text (`_` is kept: it is common inside words).
const EMPHASIS: [char; 2] = ['*', '`'];
const MAX_HEADING_LEVEL: usize = 6;

/// One line's role.
fn classify(line: &str) -> (BlockKind, &str) {
    let hashes = line.chars().take_while(|&c| c == '#').count();
    if (1..=MAX_HEADING_LEVEL).contains(&hashes) && line[hashes..].starts_with(' ') {
        return (BlockKind::Heading(hashes as u8), line[hashes..].trim());
    }
    if let Some(quoted) = line.strip_prefix('>') {
        return (BlockKind::Quote, quoted.trim());
    }
    if let Some(item) = LIST_MARKERS.iter().find_map(|marker| line.strip_prefix(marker)) {
        return (BlockKind::ListItem, item.trim());
    }
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    match line[digits..].strip_prefix(". ") {
        Some(item) if digits > 0 => (BlockKind::ListItem, item.trim()),
        _ => (BlockKind::Paragraph, line),
    }
}

/// `[text](url)` and `![alt](src)` become their text; emphasis and code markers are dropped.
pub fn strip_inline(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let close = rest[open..].find("](").map(|at| open + at);
        let end = close.and_then(|close| rest[close..].find(')').map(|at| close + at));
        let (Some(close), Some(end)) = (close, end) else { break };
        out.push_str(rest[..open].trim_end_matches('!'));
        out.push_str(&rest[open + 1..close]);
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out.chars().filter(|c| !EMPHASIS.contains(c)).collect()
}

/// Lines after an opening `---` front-matter block.
fn skip_front_matter(text: &str) -> &str {
    let Some(rest) = text.strip_prefix(FRONT_MATTER) else { return text };
    rest.find("\n---").map_or(text, |end| rest[end + 4..].trim_start_matches(['-', '\r', '\n']))
}

/// Parses Markdown; sections start at its top headings.
pub fn parse_markdown(text: &str) -> Content {
    let mut blocks: Vec<Block> = Vec::new();
    let mut paragraph: Vec<&str> = Vec::new();
    let mut code: Option<Vec<&str>> = None;
    let flush = |paragraph: &mut Vec<&str>, blocks: &mut Vec<Block>| {
        let text = normalize_whitespace(&strip_inline(&paragraph.join(" ")));
        paragraph.clear();
        blocks.extend((!text.is_empty()).then(|| Block::paragraph(text)));
    };
    for line in skip_front_matter(text).lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(FENCE) {
            flush(&mut paragraph, &mut blocks);
            match code.take() {
                Some(lines) => blocks.push(Block::new(BlockKind::Preformatted, lines.join("\n"))),
                None => code = Some(Vec::new()),
            }
            continue;
        }
        if let Some(lines) = code.as_mut() {
            lines.push(line);
            continue;
        }
        match classify(trimmed) {
            _ if trimmed.is_empty() => flush(&mut paragraph, &mut blocks),
            (BlockKind::Paragraph, text) => paragraph.push(text),
            (kind, text) => {
                flush(&mut paragraph, &mut blocks);
                blocks.push(Block::new(kind, normalize_whitespace(&strip_inline(text))));
            }
        }
    }
    flush(&mut paragraph, &mut blocks);
    let title = blocks.iter().find(|block| block.kind == BlockKind::Heading(1)).map(|block| block.text.clone());
    let mut content = Content { sections: sections_from_blocks(blocks), ..Content::default() };
    content.metadata.title = title;
    content
}
