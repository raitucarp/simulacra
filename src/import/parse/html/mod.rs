//! HTML and XHTML to blocks: headings, paragraphs, block quotes, list items and preformatted
//! text, with `<br>` kept as a line break. Shared by EPUB chapters and MOBI text.

mod builder;

#[cfg(test)]
mod tests;

use super::sections::sections_from_blocks;
use super::Content;
use crate::import::document::Block;
use crate::import::markup::{tokenize, Token};
use builder::BlockBuilder;

/// The blocks of one HTML document plus its `<title>` and `lang`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HtmlContent {
    pub title: Option<String>,
    pub language: Option<String>,
    pub blocks: Vec<Block>,
}

/// Reads the blocks of an HTML (or XHTML, or MOBI) document.
pub fn html_blocks(src: &str) -> HtmlContent {
    let mut builder = BlockBuilder::default();
    let mut language = None;
    for token in tokenize(src) {
        match token {
            Token::Open(tag) => {
                if tag.is("html") && language.is_none() {
                    language = tag.attr("xml:lang").or_else(|| tag.attr("lang"));
                }
                builder.open(&tag);
            }
            Token::Close(name) => builder.close(name),
            Token::Text(text) => builder.text(text),
        }
    }
    let (title, blocks) = builder.finish();
    HtmlContent { title, language, blocks }
}

/// A standalone HTML file: sections at its top headings.
pub fn parse(src: &str) -> Content {
    let html = html_blocks(src);
    let mut content = Content { sections: sections_from_blocks(html.blocks), ..Content::default() };
    content.metadata.title = html.title;
    content.metadata.language = html.language;
    content
}
