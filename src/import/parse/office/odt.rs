//! ODF `content.xml`: `text:h` headings (by outline level), `text:p` paragraphs (list items
//! inside `text:list-item`), with `text:s`, `text:tab` and `text:line-break` spacing.

use crate::import::document::{Block, BlockKind};
use crate::import::markup::{decode_entities, tokenize, Tag, Token};
use crate::import::parse::sections::normalize_lines;

const MAX_HEADING_LEVEL: u8 = 6;

/// Text read so far and the open element's kind.
#[derive(Default)]
struct Reader {
    out: Vec<Block>,
    current: Option<(BlockKind, String)>,
    list_depth: usize,
}

impl Reader {
    fn start(&mut self, tag: &Tag) {
        self.flush();
        let kind = if tag.is("h") {
            let level = tag.attr("outline-level").and_then(|level| level.parse::<u8>().ok()).unwrap_or(1);
            BlockKind::Heading(level.clamp(1, MAX_HEADING_LEVEL))
        } else if self.list_depth > 0 {
            BlockKind::ListItem
        } else {
            BlockKind::Paragraph
        };
        self.current = (!tag.self_closing).then(|| (kind, String::new()));
    }

    fn push(&mut self, text: &str) {
        if let Some((_, buffer)) = self.current.as_mut() {
            buffer.push_str(text);
        }
    }

    fn flush(&mut self) {
        if let Some((kind, text)) = self.current.take() {
            let text = normalize_lines(&text);
            if !text.is_empty() {
                self.out.push(Block::new(kind, text));
            }
        }
    }
}

/// Every non-empty heading and paragraph of the document body, in order.
pub fn blocks(content_xml: &str) -> Vec<Block> {
    let mut reader = Reader::default();
    for token in tokenize(content_xml) {
        match token {
            Token::Open(tag) if tag.is("p") || tag.is("h") => reader.start(&tag),
            Token::Open(tag) if tag.is("list-item") && !tag.self_closing => reader.list_depth += 1,
            // Runs of spaces collapse when the block is normalised, so one space stands for any.
            Token::Open(tag) if tag.is("s") || tag.is("tab") => reader.push(" "),
            Token::Open(tag) if tag.is("line-break") => reader.push("\n"),
            Token::Text(text) => reader.push(&decode_entities(text)),
            token if token.closes("p") || token.closes("h") => reader.flush(),
            token if token.closes("list-item") => reader.list_depth = reader.list_depth.saturating_sub(1),
            _ => {}
        }
    }
    reader.flush();
    reader.out
}
