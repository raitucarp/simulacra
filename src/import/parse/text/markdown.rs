//! Markdown: ATX headings, block quotes, list items, fenced code and paragraphs; front matter
//! and horizontal rules are skipped.
//!
//! The marks posts use survive (`**bold**`, `*italic*`, `__underline__`, `` `code` ``,
//! `||spoiler||`, and fenced blocks with their language tag), so a thread exported as
//! Markdown imports back with its formatting. Links and images keep their text;
//! strikethrough, which posts do not have, keeps its text without the tildes. Headings
//! (never posts) are plain text.

use crate::domain::post::{tokenize_post_body, visible_text};
use crate::import::document::{Block, BlockKind};
use crate::import::parse::sections::{normalize_whitespace, sections_from_blocks};
use crate::import::parse::Content;

const FENCE: &str = "```";
const FRONT_MATTER: &str = "---";
const LIST_MARKERS: [&str; 3] = ["- ", "* ", "+ "];
/// Strikethrough delimiter, dropped from text.
const STRIKE: &str = "~~";
const MAX_HEADING_LEVEL: usize = 6;
/// Characters a horizontal rule is made of, and its shortest length.
const RULE_CHARS: [char; 3] = ['-', '*', '_'];
const MIN_RULE_LEN: usize = 3;
const LINE_BREAK: &str = "\n";

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

/// `[text](url)` and `![alt](src)` become their text and strikethrough loses its tildes;
/// every other mark is kept as written.
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
    out.replace(STRIKE, "")
}

/// A horizontal rule (`---`, `***`, `___`, spaces allowed).
fn is_rule(line: &str) -> bool {
    let marks: Vec<char> = line.chars().filter(|c| !c.is_whitespace()).collect();
    marks.len() >= MIN_RULE_LEN && RULE_CHARS.contains(&marks[0]) && marks.iter().all(|&c| c == marks[0])
}

/// Lines after an opening `---` front-matter block.
fn skip_front_matter(text: &str) -> &str {
    let Some(rest) = text.strip_prefix(FRONT_MATTER) else { return text };
    rest.find("\n---").map_or(text, |end| rest[end + 4..].trim_start_matches(['-', '\r', '\n']))
}

/// Blocks collected so far, the paragraph being joined and the fenced block being read.
#[derive(Default)]
struct Reader<'a> {
    blocks: Vec<Block>,
    paragraph: Vec<&'a str>,
    /// The open fence's opening line and the code lines read so far.
    code: Option<(&'a str, Vec<&'a str>)>,
}

impl<'a> Reader<'a> {
    fn flush(&mut self) {
        let text = normalize_whitespace(&strip_inline(&self.paragraph.join(" ")));
        self.paragraph.clear();
        self.blocks.extend((!text.is_empty()).then(|| Block::paragraph(text)));
    }

    /// A fence line: opens a block, or closes the open one as a fenced `Preformatted` block
    /// (fences and language tag kept, so it reads back as a code block).
    fn fence(&mut self, opener: &'a str) {
        self.flush();
        match self.code.take() {
            Some((open, lines)) => {
                let text = [vec![open], lines, vec![FENCE]].concat().join(LINE_BREAK);
                self.blocks.push(Block::new(BlockKind::Preformatted, text));
            }
            None => self.code = Some((opener, Vec::new())),
        }
    }

    fn line(&mut self, line: &'a str) {
        let trimmed = line.trim();
        if trimmed.starts_with(FENCE) {
            return self.fence(trimmed);
        }
        if let Some((_, lines)) = self.code.as_mut() {
            return lines.push(line);
        }
        match classify(trimmed) {
            _ if trimmed.is_empty() || is_rule(trimmed) => self.flush(),
            (BlockKind::Paragraph, text) => self.paragraph.push(text),
            (kind, text) => {
                self.flush();
                let text = normalize_whitespace(&strip_inline(text));
                let text = if matches!(kind, BlockKind::Heading(_)) { visible_text(&tokenize_post_body(&text)) } else { text };
                self.blocks.push(Block::new(kind, text));
            }
        }
    }
}

/// Parses Markdown; sections start at its top headings.
pub fn parse_markdown(text: &str) -> Content {
    let mut reader = Reader::default();
    for line in skip_front_matter(text).lines() {
        reader.line(line);
    }
    reader.flush();
    let blocks = reader.blocks;
    let title = blocks.iter().find(|block| block.kind == BlockKind::Heading(1)).map(|block| block.text.clone());
    let mut content = Content { sections: sections_from_blocks(blocks), ..Content::default() };
    content.metadata.title = title;
    content
}
