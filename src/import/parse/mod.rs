//! Pure parsers, one module per format family: bytes in, [`ParsedDocument`] out.
//!
//! [`parse_document`] never fails. A file that cannot be read as its format (corrupt,
//! encrypted, an unsupported compression) is read as a binary instead, with a note saying why,
//! so every file yields at least its facts and printable strings.

mod binary;
mod container;
mod epub;
mod html;
mod mobi;
mod office;
mod pdf;
mod sections;
mod text;

pub use binary::sha256_hex;
pub use sections::{normalize_whitespace, sections_from_blocks};

use super::detect::{Format, FormatFamily};
use super::document::{DocumentMetadata, FileFacts, ParseNote, ParsedDocument, Section};

/// What a format parser extracts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Content {
    pub metadata: DocumentMetadata,
    pub sections: Vec<Section>,
    pub notes: Vec<ParseNote>,
}

/// Why a file could not be read as its format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Unreadable(String),
    Encrypted,
    Unsupported(&'static str),
}

impl ParseError {
    fn into_note(self) -> ParseNote {
        match self {
            Self::Unreadable(reason) => ParseNote::Unreadable(reason),
            Self::Encrypted => ParseNote::Encrypted,
            Self::Unsupported(feature) => ParseNote::Unsupported(feature),
        }
    }
}

/// Shorthand for parser results.
pub type ParseResult = Result<Content, ParseError>;

/// Parses `bytes` (detected as `format`) from a file called `file_name`.
pub fn parse_document(format: Format, bytes: &[u8], file_name: &str) -> ParsedDocument {
    let facts = FileFacts { size_bytes: bytes.len() as u64, sha256: sha256_hex(bytes), mime: format.mime() };
    let (format, mut content) = match parse_as(format, bytes) {
        Ok(content) => (format, content),
        Err(err) => {
            let mut content = binary::parse_unknown(bytes);
            content.notes.insert(0, err.into_note());
            (Format::Unknown, content)
        }
    };
    if content.metadata.title.as_deref().is_none_or(|title| title.trim().is_empty()) {
        content.metadata.title = Some(title_from_file_name(file_name));
    }
    let facts = FileFacts { mime: if format == Format::Unknown { facts.mime } else { format.mime() }, ..facts };
    ParsedDocument { format, metadata: content.metadata, facts, sections: content.sections, notes: content.notes }
}

fn parse_as(format: Format, bytes: &[u8]) -> ParseResult {
    match format {
        Format::Epub => epub::parse(bytes),
        Format::Mobi => mobi::parse(bytes),
        Format::Pdf => pdf::parse(bytes),
        Format::Docx => office::parse_docx(bytes),
        Format::Odt => office::parse_odt(bytes),
        Format::Html => Ok(html::parse(&text::decode_text(bytes))),
        Format::Markdown => Ok(text::parse_markdown(&text::decode_text(bytes))),
        Format::PlainText => Ok(text::parse_plain(&text::decode_text(bytes))),
        Format::Rtf => Ok(text::parse_rtf(&text::decode_text(bytes))),
        Format::Csv => Ok(text::parse_delimited(&text::decode_text(bytes), ',')),
        Format::Tsv => Ok(text::parse_delimited(&text::decode_text(bytes), '\t')),
        Format::Json => text::parse_json(bytes),
        Format::Zip | Format::Tar | Format::Gzip => binary::parse_archive(format, bytes),
        other if other.family() == FormatFamily::Image => Ok(binary::parse_image(format, bytes)),
        other if matches!(other.family(), FormatFamily::Audio | FormatFamily::Video) => Ok(Content::default()),
        _ => Ok(binary::parse_unknown(bytes)),
    }
}

/// `Moby_Dick-or the.Whale.epub` → `Moby Dick or the.Whale`.
pub fn title_from_file_name(file_name: &str) -> String {
    let file = file_name.rsplit(['/', '\\']).next().unwrap_or(file_name);
    let stem = file.rsplit_once('.').map_or(file, |(stem, _)| stem);
    let spaced: String = stem.chars().map(|c| if c == '_' || c == '-' { ' ' } else { c }).collect();
    let title = normalize_whitespace(&spaced);
    if title.is_empty() { file.to_string() } else { title }
}

#[cfg(test)]
mod tests;
