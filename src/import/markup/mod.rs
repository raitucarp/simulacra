//! A tolerant, zero-copy markup tokenizer shared by every HTML and XML reader (XHTML, MOBI
//! HTML, OPF, NCX, DOCX, ODT). It never fails: unclosed tags become text, comments,
//! declarations and processing instructions are skipped, and `<script>` / `<style>` bodies
//! are swallowed whole.

mod entities;
mod query;
mod tag;

#[cfg(test)]
mod tests;

pub use entities::decode_entities;
pub use query::{element_texts, first_element_text, tags_named};
pub use tag::{local_name, Tag};

/// Elements whose content is raw text that never holds markup.
const RAW_TEXT_ELEMENTS: [&str; 2] = ["script", "style"];

/// One piece of markup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    /// An opening (or self-closing) tag.
    Open(Tag<'a>),
    /// A closing tag, by its raw name (`w:p`, `h2`).
    Close(&'a str),
    /// Raw text between tags (entities not yet decoded).
    Text(&'a str),
}

impl Token<'_> {
    /// Whether this is a closing tag of `name` (local name, case-insensitive).
    pub fn closes(&self, name: &str) -> bool {
        matches!(self, Token::Close(raw) if local_name(raw).eq_ignore_ascii_case(name))
    }
}

/// Iterator over the tokens of a markup string.
pub struct Tokens<'a> {
    src: &'a str,
    at: usize,
}

/// Tokenizes `src`.
pub fn tokenize(src: &str) -> Tokens<'_> {
    Tokens { src, at: 0 }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        while self.at < self.src.len() {
            let rest = &self.src[self.at..];
            if !rest.starts_with('<') {
                let end = rest.find('<').unwrap_or(rest.len());
                self.at += end;
                return Some(Token::Text(&rest[..end]));
            }
            if let Some(token) = self.markup(rest) {
                return Some(token);
            }
        }
        None
    }
}

impl<'a> Tokens<'a> {
    /// Reads the markup at the start of `rest` (which begins with `<`); `None` when it was
    /// skipped (a comment, declaration or processing instruction).
    fn markup(&mut self, rest: &'a str) -> Option<Token<'a>> {
        if let Some(body) = rest.strip_prefix("<![CDATA[") {
            let end = body.find("]]>").unwrap_or(body.len());
            self.at += "<![CDATA[".len() + end + "]]>".len().min(body.len() - end);
            return Some(Token::Text(&body[..end]));
        }
        for (open, close) in [("<!--", "-->"), ("<!", ">"), ("<?", "?>")] {
            if let Some(after) = rest.strip_prefix(open) {
                self.at += after.find(close).map_or(rest.len(), |end| open.len() + end + close.len());
                return None;
            }
        }
        let starts_tag = rest[1..].starts_with(|c: char| c.is_alphabetic() || c == '/');
        let Some(end) = tag_end(rest).filter(|_| starts_tag) else {
            self.at += 1;
            return Some(Token::Text(&rest[..1]));
        };
        self.at += end + 1;
        let inner = &rest[1..end];
        if let Some(name) = inner.strip_prefix('/') {
            return Some(Token::Close(name.trim()));
        }
        let tag = Tag::parse(inner);
        if !tag.self_closing && RAW_TEXT_ELEMENTS.iter().any(|raw| tag.is(raw)) {
            self.skip_raw_text(tag.name);
        }
        Some(Token::Open(tag))
    }

    /// Skips past the closing tag of a raw-text element.
    fn skip_raw_text(&mut self, name: &str) {
        let rest = &self.src[self.at..];
        let closing = format!("</{}", name.to_ascii_lowercase());
        let end = rest.to_ascii_lowercase().find(&closing).unwrap_or(rest.len());
        self.at += end;
    }
}

/// Byte index of the `>` closing the tag that starts `rest`, skipping quoted attribute values.
fn tag_end(rest: &str) -> Option<usize> {
    let mut quote: Option<char> = None;
    for (index, c) in rest.char_indices().skip(1) {
        match (quote, c) {
            (Some(open), c) if c == open => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '>') => return Some(index),
            (None, '<') => return None,
            _ => {}
        }
    }
    None
}
