//! The PalmDB container: a 78-byte header, then a table of record offsets.

use crate::import::parse::ParseError;

/// Offset of the record count in the PalmDB header.
const RECORD_COUNT_OFFSET: usize = 76;
/// First record-table entry.
const RECORD_TABLE_OFFSET: usize = 78;
/// Bytes per record-table entry (offset, attributes, unique id).
const RECORD_ENTRY_BYTES: usize = 8;
/// Length of the database name field.
const NAME_BYTES: usize = 32;

pub fn read_u16(bytes: &[u8], at: usize) -> Option<u16> {
    bytes.get(at..at + 2).map(|b| u16::from_be_bytes([b[0], b[1]]))
}

pub fn read_u32(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

fn corrupt(what: &str) -> ParseError {
    ParseError::Unreadable(format!("PalmDB {what}"))
}

/// The database name (NUL-terminated in the header).
pub fn database_name(bytes: &[u8]) -> String {
    let name = &bytes[..bytes.len().min(NAME_BYTES)];
    let end = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    String::from_utf8_lossy(&name[..end]).into_owned()
}

/// Every record, in order, as a slice of `bytes`.
pub fn records(bytes: &[u8]) -> Result<Vec<&[u8]>, ParseError> {
    let count = usize::from(read_u16(bytes, RECORD_COUNT_OFFSET).ok_or_else(|| corrupt("header"))?);
    let offsets: Vec<usize> = (0..count)
        .map(|index| read_u32(bytes, RECORD_TABLE_OFFSET + index * RECORD_ENTRY_BYTES).map(|o| o as usize))
        .collect::<Option<_>>()
        .ok_or_else(|| corrupt("record table"))?;
    offsets
        .iter()
        .enumerate()
        .map(|(index, &start)| {
            let end = offsets.get(index + 1).copied().unwrap_or(bytes.len());
            bytes.get(start..end.max(start)).ok_or_else(|| corrupt("record offset"))
        })
        .collect()
}
