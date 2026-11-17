//! Accumulates HTML text into blocks as block-level tags open and close.

use crate::import::document::{Block, BlockKind};
use crate::import::markup::{decode_entities, local_name, Tag};
use crate::import::parse::sections::{normalize_lines, normalize_whitespace};

/// Elements that start and end a block.
const BLOCK_ELEMENTS: [&str; 34] = [
    "p", "div", "section", "article", "aside", "header", "footer", "main", "body", "h1", "h2", "h3", "h4",
    "h5", "h6", "blockquote", "li", "dt", "dd", "ul", "ol", "dl", "pre", "table", "tr", "td", "th",
    "figure", "figcaption", "hgroup", "address", "nav", "caption", "center",
];
/// Elements whose content is never prose.
const HIDDEN_ELEMENTS: [&str; 5] = ["svg", "math", "noscript", "template", "object"];
/// Elements that end a block without starting one.
const BREAK_ELEMENTS: [&str; 2] = ["hr", "pagebreak"];

fn heading_level(name: &str) -> Option<u8> {
    match name.as_bytes() {
        [b'h', level @ b'1'..=b'6'] => Some(level - b'0'),
        _ => None,
    }
}

fn is_in(list: &[&str], name: &str) -> bool {
    list.iter().any(|item| item.eq_ignore_ascii_case(name))
}

/// The in-progress block and the element context around it.
#[derive(Debug, Default)]
pub struct BlockBuilder {
    blocks: Vec<Block>,
    text: String,
    heading: Option<u8>,
    quote_depth: usize,
    list_depth: usize,
    pre_depth: usize,
    head_depth: usize,
    hidden_depth: usize,
    in_title: bool,
    title: String,
}

impl BlockBuilder {
    pub fn open(&mut self, tag: &Tag) {
        let name = tag.local().to_ascii_lowercase();
        let opens = !tag.self_closing;
        match name.as_str() {
            hidden if is_in(&HIDDEN_ELEMENTS, hidden) => self.hidden_depth += usize::from(opens),
            "head" => self.head_depth += usize::from(opens),
            "title" if self.head_depth > 0 => self.in_title = opens,
            "br" => self.text.push('\n'),
            breaks if is_in(&BREAK_ELEMENTS, breaks) => self.flush(),
            block if is_in(&BLOCK_ELEMENTS, block) => {
                self.flush();
                if opens {
                    self.enter(block, true);
                }
            }
            _ => {}
        }
    }

    pub fn close(&mut self, raw_name: &str) {
        let name = local_name(raw_name.trim()).to_ascii_lowercase();
        match name.as_str() {
            hidden if is_in(&HIDDEN_ELEMENTS, hidden) => self.hidden_depth = self.hidden_depth.saturating_sub(1),
            "head" => self.head_depth = self.head_depth.saturating_sub(1),
            "title" => self.in_title = false,
            block if is_in(&BLOCK_ELEMENTS, block) => {
                self.flush();
                self.enter(block, false);
            }
            _ => {}
        }
    }

    /// Updates the context for entering (`true`) or leaving a block element.
    fn enter(&mut self, name: &str, entering: bool) {
        let step = |depth: &mut usize| *depth = if entering { *depth + 1 } else { depth.saturating_sub(1) };
        match name {
            "blockquote" => step(&mut self.quote_depth),
            "li" | "dt" | "dd" => step(&mut self.list_depth),
            "pre" => step(&mut self.pre_depth),
            other => {
                if let Some(level) = heading_level(other) {
                    self.heading = entering.then_some(level);
                }
            }
        }
    }

    pub fn text(&mut self, raw: &str) {
        if self.hidden_depth > 0 {
            return;
        }
        let decoded = decode_entities(raw);
        if self.in_title {
            self.title.push_str(&decoded);
        } else if self.head_depth == 0 && self.pre_depth > 0 {
            self.text.push_str(&decoded);
        } else if self.head_depth == 0 {
            // Source line breaks are layout, not content: only `<br>` makes a line break.
            self.text.extend(decoded.chars().map(|c| if c.is_whitespace() { ' ' } else { c }));
        }
    }

    fn kind(&self) -> BlockKind {
        match self.heading {
            Some(level) => BlockKind::Heading(level),
            None if self.pre_depth > 0 => BlockKind::Preformatted,
            None if self.list_depth > 0 => BlockKind::ListItem,
            None if self.quote_depth > 0 => BlockKind::Quote,
            None => BlockKind::Paragraph,
        }
    }

    fn flush(&mut self) {
        let raw = std::mem::take(&mut self.text);
        let kind = self.kind();
        let text = match kind {
            BlockKind::Preformatted => raw.trim_matches('\n').trim_end().to_string(),
            BlockKind::Heading(_) => normalize_whitespace(&raw),
            _ => normalize_lines(&raw),
        };
        if !text.is_empty() {
            self.blocks.push(Block::new(kind, text));
        }
    }

    /// The document title (if any) and every block.
    pub fn finish(mut self) -> (Option<String>, Vec<Block>) {
        self.flush();
        let title = normalize_whitespace(&self.title);
        ((!title.is_empty()).then_some(title), self.blocks)
    }
}
