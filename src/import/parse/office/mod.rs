//! Word-processor files: DOCX (`word/document.xml`, `docProps/core.xml`) and ODT
//! (`content.xml`, `meta.xml`), read as zip plus XML into headings, paragraphs and list items.

mod docx;
mod odt;

#[cfg(test)]
mod tests;

use super::container::ZipBundle;
use super::sections::sections_from_blocks;
use super::{Content, ParseError, ParseResult};
use crate::import::document::{Block, DocumentMetadata};
use crate::import::markup::{element_texts, first_element_text};

/// Parses a DOCX package.
pub fn parse_docx(bytes: &[u8]) -> ParseResult {
    let mut zip = ZipBundle::open(bytes)?;
    let body = zip.read_text("word/document.xml").ok_or_else(|| missing("word/document.xml"))?;
    let metadata = zip.read_text("docProps/core.xml").map(|core| core_metadata(&core)).unwrap_or_default();
    Ok(content(metadata, docx::blocks(&body)))
}

/// Parses an ODT package.
pub fn parse_odt(bytes: &[u8]) -> ParseResult {
    let mut zip = ZipBundle::open(bytes)?;
    let body = zip.read_text("content.xml").ok_or_else(|| missing("content.xml"))?;
    let mut metadata = zip.read_text("meta.xml").map(|meta| core_metadata(&meta)).unwrap_or_default();
    if metadata.creators.is_empty() {
        metadata.creators = zip.read_text("meta.xml").map(|meta| element_texts(&meta, "initial-creator")).unwrap_or_default();
    }
    Ok(content(metadata, odt::blocks(&body)))
}

fn content(metadata: DocumentMetadata, blocks: Vec<Block>) -> Content {
    Content { metadata, sections: sections_from_blocks(blocks), notes: Vec::new() }
}

fn missing(what: &str) -> ParseError {
    ParseError::Unreadable(format!("missing {what}"))
}

/// Dublin Core fields shared by `docProps/core.xml` and ODF `meta.xml`.
fn core_metadata(xml: &str) -> DocumentMetadata {
    DocumentMetadata {
        title: first_element_text(xml, "title"),
        creators: element_texts(xml, "creator"),
        language: first_element_text(xml, "language"),
        date: first_element_text(xml, "created").or_else(|| first_element_text(xml, "creation-date")),
        identifiers: element_texts(xml, "identifier"),
        description: first_element_text(xml, "description").or_else(|| first_element_text(xml, "subject")),
        ..DocumentMetadata::default()
    }
}
