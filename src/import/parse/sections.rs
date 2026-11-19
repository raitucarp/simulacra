//! Shared helpers: whitespace normalisation and grouping a flat block list into sections.

use crate::import::document::{Block, BlockKind, Section};

/// Collapses every run of whitespace (including line breaks) into one space and trims.
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Normalises each line on its own and keeps single line breaks between non-blank lines.
pub fn normalize_lines(text: &str) -> String {
    let lines = text.lines().map(normalize_whitespace).filter(|line| !line.is_empty());
    lines.collect::<Vec<_>>().join("\n")
}

/// The top heading level present (the smallest number), if any heading exists.
fn top_heading_level(blocks: &[Block]) -> Option<u8> {
    blocks
        .iter()
        .filter_map(|block| match block.kind {
            BlockKind::Heading(level) => Some(level),
            _ => None,
        })
        .min()
}

/// Splits `blocks` into sections at every heading of the top level present. Blocks before the
/// first such heading form an untitled section; without headings everything is one section.
pub fn sections_from_blocks(blocks: Vec<Block>) -> Vec<Section> {
    let Some(top) = top_heading_level(&blocks) else {
        return non_empty(vec![Section::new(None, blocks)]);
    };
    let mut sections = vec![Section::default()];
    for block in blocks {
        if block.kind == BlockKind::Heading(top) {
            sections.push(Section::new(Some(block.text.clone()), vec![block]));
        } else if let Some(current) = sections.last_mut() {
            current.blocks.push(block);
        }
    }
    non_empty(sections)
}

/// Drops sections without any block.
pub fn non_empty(sections: Vec<Section>) -> Vec<Section> {
    sections.into_iter().filter(|section| !section.blocks.is_empty()).collect()
}
