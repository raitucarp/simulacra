//! Record 0: the PalmDOC header and, after it, the MOBI (or KF8) header.

use super::palm::{read_u16, read_u32};
use crate::import::parse::ParseError;

const PALMDOC_HEADER_BYTES: usize = 16;
const MOBI_MAGIC: &[u8] = b"MOBI";
const OFFSET_HEADER_LENGTH: usize = 20;
const OFFSET_ENCODING: usize = 28;
const OFFSET_VERSION: usize = 36;
const OFFSET_FULL_NAME: usize = 84;
const OFFSET_FULL_NAME_LENGTH: usize = 88;
const OFFSET_EXTH_FLAGS: usize = 128;
const OFFSET_FDST_INDEX: usize = 192;
const OFFSET_EXTRA_FLAGS: usize = 242;
/// Header length from which the extra-data flags field exists.
const EXTRA_FLAGS_MIN_HEADER: u32 = 0xE4;
const EXTH_PRESENT_BIT: u32 = 0x40;
const KF8_VERSION: u32 = 8;
const NO_RECORD: u32 = u32::MAX;
const UTF8_CODE_PAGE: u32 = 65001;

/// How the text records are compressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    None,
    PalmDoc,
    /// HUFF/CDIC, which this reader does not decode.
    Huffman,
    Other(u16),
}

/// What the importer reads from record 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookHeader {
    pub compression: Compression,
    pub text_length: usize,
    pub text_records: usize,
    pub encrypted: bool,
    pub utf8: bool,
    pub full_name: Option<String>,
    /// Bit 0: multibyte overlap bytes; higher bits: trailing entries per text record.
    pub extra_flags: u16,
    /// Bytes of the EXTH block, when present.
    pub exth: Option<std::ops::Range<usize>>,
    /// Record index of the KF8 flow table.
    pub fdst_record: Option<usize>,
}

/// Parses record 0.
pub fn parse_header(record0: &[u8]) -> Result<BookHeader, ParseError> {
    let short = || ParseError::Unreadable("MOBI header".to_string());
    let compression = match read_u16(record0, 0).ok_or_else(short)? {
        1 => Compression::None,
        2 => Compression::PalmDoc,
        17480 => Compression::Huffman,
        other => Compression::Other(other),
    };
    let has_mobi = record0.get(PALMDOC_HEADER_BYTES..PALMDOC_HEADER_BYTES + MOBI_MAGIC.len()) == Some(MOBI_MAGIC);
    let field = |at: usize| if has_mobi { read_u32(record0, at) } else { None };
    let header_length = field(OFFSET_HEADER_LENGTH).unwrap_or(0);
    let version = field(OFFSET_VERSION).unwrap_or(0);
    let exth = (field(OFFSET_EXTH_FLAGS).unwrap_or(0) & EXTH_PRESENT_BIT != 0)
        .then(|| PALMDOC_HEADER_BYTES + header_length as usize..record0.len());
    Ok(BookHeader {
        compression,
        text_length: read_u32(record0, 4).ok_or_else(short)? as usize,
        text_records: usize::from(read_u16(record0, 8).ok_or_else(short)?),
        encrypted: read_u16(record0, 12).ok_or_else(short)? != 0,
        utf8: field(OFFSET_ENCODING) == Some(UTF8_CODE_PAGE),
        full_name: full_name(record0, has_mobi),
        extra_flags: if header_length >= EXTRA_FLAGS_MIN_HEADER { read_u16(record0, OFFSET_EXTRA_FLAGS).unwrap_or(0) } else { 0 },
        exth,
        fdst_record: field(OFFSET_FDST_INDEX).filter(|&i| version >= KF8_VERSION && i != NO_RECORD && i != 0).map(|i| i as usize),
    })
}

fn full_name(record0: &[u8], has_mobi: bool) -> Option<String> {
    if !has_mobi {
        return None;
    }
    let start = read_u32(record0, OFFSET_FULL_NAME)? as usize;
    let length = read_u32(record0, OFFSET_FULL_NAME_LENGTH)? as usize;
    let bytes = record0.get(start..start + length)?;
    Some(String::from_utf8_lossy(bytes).trim().to_string()).filter(|name| !name.is_empty())
}
