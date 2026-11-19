//! MOBI, AZW and AZW3 (KF8) books, and plain PalmDOC texts: the PalmDB records, the MOBI
//! header and EXTH metadata, PalmDOC decompression, then the text's HTML as blocks.
//! HUFF/CDIC compression and DRM are reported as unsupported.

mod exth;
mod header;
mod palm;
mod palmdoc;

#[cfg(test)]
mod tests;

use super::html::html_blocks;
use super::sections::sections_from_blocks;
use super::{Content, ParseError, ParseResult};
use header::{BookHeader, Compression};

/// PalmDB type of a plain-text PalmDOC (no MOBI header, no HTML).
const PLAIN_PALMDOC: &[u8] = b"TEXtREAd";
const PALM_TYPE_OFFSET: usize = 60;
const FDST_MAGIC: &[u8] = b"FDST";
/// Where an FDST record stores the offset of its `(start, end)` table, and the usual offset.
const FDST_TABLE_POINTER: usize = 4;
const FDST_TABLE_OFFSET: usize = 12;

/// Parses a PalmDB book.
pub fn parse(bytes: &[u8]) -> ParseResult {
    let records = palm::records(bytes)?;
    let record0 = records.first().ok_or_else(|| ParseError::Unreadable("no records".to_string()))?;
    let header = header::parse_header(record0)?;
    if header.encrypted {
        return Err(ParseError::Encrypted);
    }
    match header.compression {
        Compression::Huffman => return Err(ParseError::Unsupported("HUFF/CDIC")),
        Compression::Other(_) => return Err(ParseError::Unsupported("compression")),
        Compression::None | Compression::PalmDoc => {}
    }
    let raw = first_flow(&records, &header, text_bytes(&records, &header));
    let text = decode(&raw, header.utf8);
    if bytes.get(PALM_TYPE_OFFSET..PALM_TYPE_OFFSET + PLAIN_PALMDOC.len()) == Some(PLAIN_PALMDOC) {
        let mut content = super::text::parse_plain(&text);
        content.metadata.title = Some(palm::database_name(bytes));
        return Ok(content);
    }
    let html = html_blocks(&text);
    let mut metadata = header.exth.clone().and_then(|range| record0.get(range)).map(exth::exth_metadata).unwrap_or_default();
    metadata.title = metadata.title.or(header.full_name).or_else(|| Some(palm::database_name(bytes)));
    metadata.language = metadata.language.or(html.language);
    Ok(Content { metadata, sections: sections_from_blocks(html.blocks), notes: Vec::new() })
}

/// The decompressed text records, concatenated and cut to the declared text length.
fn text_bytes(records: &[&[u8]], header: &BookHeader) -> Vec<u8> {
    let mut text: Vec<u8> = records
        .iter()
        .skip(1)
        .take(header.text_records)
        .flat_map(|record| {
            let body = palmdoc::strip_trailing_entries(record, header.extra_flags);
            match header.compression {
                Compression::PalmDoc => palmdoc::decompress(body),
                _ => body.to_vec(),
            }
        })
        .collect();
    if header.text_length > 0 {
        text.truncate(header.text_length);
    }
    text
}

/// KF8 stores CSS and SVG flows after the text; the FDST record says where the text ends.
fn first_flow(records: &[&[u8]], header: &BookHeader, mut text: Vec<u8>) -> Vec<u8> {
    let fdst = header.fdst_record.and_then(|index| records.get(index)).filter(|r| r.starts_with(FDST_MAGIC));
    let table = |record: &[u8]| palm::read_u32(record, FDST_TABLE_POINTER).map_or(FDST_TABLE_OFFSET, |at| at as usize);
    let end = fdst.and_then(|record| palm::read_u32(record, table(record) + 4)).map(|end| end as usize);
    if let Some(end) = end.filter(|&end| end > 0 && end < text.len()) {
        text.truncate(end);
    }
    text
}

/// UTF-8 books decode as UTF-8; everything else as Windows-1252.
fn decode(raw: &[u8], utf8: bool) -> String {
    if utf8 {
        String::from_utf8_lossy(raw).into_owned()
    } else {
        encoding_rs::WINDOWS_1252.decode(raw).0.into_owned()
    }
}
