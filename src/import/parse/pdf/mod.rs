//! PDF: the text layer page by page (one section per page), paragraphs rebuilt from lines,
//! and the document information dictionary as metadata. Scanned pages without a text layer
//! are counted in a note; encrypted files are opened with the empty password when allowed.

mod paragraphs;
mod text_layer;

#[cfg(test)]
mod tests;

pub use paragraphs::rebuild_paragraphs;

use super::{Content, ParseError, ParseResult};
use crate::import::document::{Block, DocumentMetadata, ParseNote, Section};
use lopdf::{decode_text_string, Document};

/// Parses a PDF.
pub fn parse(bytes: &[u8]) -> ParseResult {
    let mut doc = Document::load_mem(bytes).map_err(|err| ParseError::Unreadable(err.to_string()))?;
    if doc.is_encrypted() && doc.decrypt("").is_err() {
        return Err(ParseError::Encrypted);
    }
    let pages: Vec<Section> = doc
        .get_pages()
        .values()
        .map(|&page_id| {
            let blocks = rebuild_paragraphs(&text_layer::page_lines(&doc, page_id));
            Section::new(None, blocks.into_iter().map(Block::paragraph).collect())
        })
        .collect();
    let blank = pages.iter().filter(|page| page.blocks.is_empty()).count();
    let notes = (blank > 0).then_some(ParseNote::PagesWithoutText(blank)).into_iter().collect();
    Ok(Content { metadata: info(&doc), sections: pages.into_iter().filter(|p| !p.blocks.is_empty()).collect(), notes })
}

/// The `/Info` dictionary's title, author, subject and creation date.
fn info(doc: &Document) -> DocumentMetadata {
    let dictionary = doc
        .trailer
        .get(b"Info")
        .ok()
        .and_then(|info| info.as_reference().ok())
        .and_then(|id| doc.get_dictionary(id).ok());
    let field = |key: &[u8]| {
        let text = dictionary?.get(key).ok().and_then(|value| decode_text_string(value).ok())?;
        let text = text.trim().to_string();
        (!text.is_empty()).then_some(text)
    };
    DocumentMetadata {
        title: field(b"Title"),
        creators: field(b"Author").into_iter().collect(),
        date: field(b"CreationDate").map(|date| pdf_date(&date)),
        description: field(b"Subject"),
        ..DocumentMetadata::default()
    }
}

/// `D:20240131120000Z` → `2024-01-31`; other text is returned as written.
fn pdf_date(raw: &str) -> String {
    let digits: String = raw.trim_start_matches("D:").chars().take_while(char::is_ascii_digit).collect();
    match digits.len() {
        8.. => format!("{}-{}-{}", &digits[..4], &digits[4..6], &digits[6..8]),
        4..=7 => digits[..4].to_string(),
        _ => raw.to_string(),
    }
}
