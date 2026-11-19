//! `word/document.xml`: every `w:p` is a block; its `w:pStyle` makes it a heading or a
//! quote, `w:numPr` a list item; `w:t` runs carry the text, `w:tab` and `w:br` the spacing.

use crate::import::document::{Block, BlockKind};
use crate::import::markup::{decode_entities, tokenize, Token};
use crate::import::parse::sections::normalize_lines;

const HEADING_STYLE_PREFIX: &str = "heading";
const TITLE_STYLE: &str = "title";
const QUOTE_STYLES: [&str; 2] = ["quote", "intensequote"];
const MAX_HEADING_LEVEL: u8 = 6;

/// The block kind a paragraph style id names (`Heading2`, `heading 2`, `Title`, `Quote`).
pub fn style_kind(style: &str, is_list: bool) -> BlockKind {
    let style = style.to_ascii_lowercase().replace(' ', "");
    if style == TITLE_STYLE {
        return BlockKind::Heading(1);
    }
    if let Some(level) = style.strip_prefix(HEADING_STYLE_PREFIX).and_then(|n| n.parse::<u8>().ok()) {
        return BlockKind::Heading(level.clamp(1, MAX_HEADING_LEVEL));
    }
    if QUOTE_STYLES.contains(&style.as_str()) {
        BlockKind::Quote
    } else if is_list {
        BlockKind::ListItem
    } else {
        BlockKind::Paragraph
    }
}

/// The paragraph being read.
#[derive(Default)]
struct Paragraph {
    text: String,
    style: String,
    is_list: bool,
}

/// Every non-empty paragraph of the document body, in order.
pub fn blocks(document_xml: &str) -> Vec<Block> {
    let mut out = Vec::new();
    let mut paragraph = Paragraph::default();
    let mut in_text = false;
    for token in tokenize(document_xml) {
        match token {
            Token::Open(tag) if tag.is("p") && !tag.self_closing => paragraph = Paragraph::default(),
            Token::Open(tag) if tag.is("pStyle") => paragraph.style = tag.attr("val").unwrap_or_default(),
            Token::Open(tag) if tag.is("numPr") => paragraph.is_list = true,
            Token::Open(tag) if tag.is("t") => in_text = !tag.self_closing,
            Token::Open(tag) if tag.is("tab") => paragraph.text.push(' '),
            Token::Open(tag) if tag.is("br") || tag.is("cr") => paragraph.text.push('\n'),
            Token::Text(text) if in_text => paragraph.text.push_str(&decode_entities(text)),
            token if token.closes("t") => in_text = false,
            token if token.closes("p") => {
                let text = normalize_lines(&std::mem::take(&mut paragraph.text));
                if !text.is_empty() {
                    out.push(Block::new(style_kind(&paragraph.style, paragraph.is_list), text));
                }
            }
            _ => {}
        }
    }
    out
}
