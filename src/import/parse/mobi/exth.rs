//! EXTH: the MOBI metadata block (author, publisher, dates, identifiers, language, title).

use super::palm::read_u32;
use crate::import::document::DocumentMetadata;

const EXTH_MAGIC: &[u8] = b"EXTH";
const RECORD_HEADER_BYTES: usize = 8;
const AUTHOR: u32 = 100;
const PUBLISHER: u32 = 101;
const DESCRIPTION: u32 = 103;
const ISBN: u32 = 104;
const PUBLISHED: u32 = 106;
const SOURCE: u32 = 112;
const ASIN: u32 = 113;
const UPDATED_TITLE: u32 = 503;
const LANGUAGE: u32 = 524;

/// `(type, data)` of every EXTH record in `block`.
pub fn exth_records(block: &[u8]) -> Vec<(u32, &[u8])> {
    if block.get(..EXTH_MAGIC.len()) != Some(EXTH_MAGIC) {
        return Vec::new();
    }
    let count = read_u32(block, 8).unwrap_or(0) as usize;
    let mut at = 12;
    let mut out = Vec::new();
    for _ in 0..count {
        let (Some(kind), Some(length)) = (read_u32(block, at), read_u32(block, at + 4)) else { break };
        let length = length as usize;
        let Some(data) = block.get(at + RECORD_HEADER_BYTES..at + length.max(RECORD_HEADER_BYTES)) else { break };
        out.push((kind, data));
        at += length.max(RECORD_HEADER_BYTES);
    }
    out
}

/// Metadata from the EXTH records (text decoded as UTF-8).
pub fn exth_metadata(block: &[u8]) -> DocumentMetadata {
    let mut meta = DocumentMetadata::default();
    for (kind, data) in exth_records(block) {
        let text = String::from_utf8_lossy(data).trim().to_string();
        if text.is_empty() {
            continue;
        }
        match kind {
            AUTHOR => meta.creators.push(text),
            PUBLISHER => meta.publisher = Some(text),
            DESCRIPTION => meta.description = Some(text),
            ISBN | SOURCE | ASIN => meta.identifiers.push(text),
            PUBLISHED => meta.date = Some(text),
            UPDATED_TITLE => meta.title = Some(text),
            LANGUAGE => meta.language = Some(text),
            _ => {}
        }
    }
    meta
}
