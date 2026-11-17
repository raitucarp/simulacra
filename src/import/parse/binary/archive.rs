//! Archives: the entry list of zip and tar files, and the original name inside a gzip.

use crate::import::detect::Format;
use crate::import::document::{Block, BlockKind, ParseNote, Section};
use crate::import::parse::container::ZipBundle;
use crate::import::parse::{Content, ParseResult};

const TAR_BLOCK: usize = 512;
const TAR_NAME_BYTES: usize = 100;
const TAR_SIZE_OFFSET: usize = 124;
const TAR_SIZE_BYTES: usize = 12;
const TAR_TYPE_OFFSET: usize = 156;
const TAR_PREFIX_OFFSET: usize = 345;
const TAR_PREFIX_BYTES: usize = 155;
const GZIP_HEADER_BYTES: usize = 10;
const GZIP_FLAG_EXTRA: u8 = 0x04;
const GZIP_FLAG_NAME: u8 = 0x08;
/// Most entries listed from one archive.
const MAX_ENTRIES: usize = 5000;

fn field(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim().to_string()
}

/// `(path, size)` of every regular file in a tar archive.
pub fn tar_entries(bytes: &[u8]) -> Vec<(String, u64)> {
    let mut entries = Vec::new();
    let mut at = 0;
    while let Some(header) = bytes.get(at..at + TAR_BLOCK).filter(|h| h.iter().any(|&b| b != 0)) {
        let name = field(&header[..TAR_NAME_BYTES]);
        let prefix = field(&header[TAR_PREFIX_OFFSET..TAR_PREFIX_OFFSET + TAR_PREFIX_BYTES]);
        let size = u64::from_str_radix(&field(&header[TAR_SIZE_OFFSET..TAR_SIZE_OFFSET + TAR_SIZE_BYTES]), 8).unwrap_or(0);
        if matches!(header[TAR_TYPE_OFFSET], b'0' | 0) && entries.len() < MAX_ENTRIES {
            entries.push((if prefix.is_empty() { name } else { format!("{prefix}/{name}") }, size));
        }
        at += TAR_BLOCK + (size as usize).div_ceil(TAR_BLOCK) * TAR_BLOCK;
    }
    entries
}

/// The original file name stored in a gzip header.
pub fn gzip_name(bytes: &[u8]) -> Option<String> {
    let flags = *bytes.get(3)?;
    if flags & GZIP_FLAG_NAME == 0 {
        return None;
    }
    let mut at = GZIP_HEADER_BYTES;
    if flags & GZIP_FLAG_EXTRA != 0 {
        at += 2 + usize::from(u16::from_le_bytes([*bytes.get(at)?, *bytes.get(at + 1)?]));
    }
    Some(field(bytes.get(at..)?)).filter(|name| !name.is_empty())
}

/// Lists an archive's entries, one list-item block each.
pub fn parse_archive(format: Format, bytes: &[u8]) -> ParseResult {
    let entries: Vec<(String, u64)> = match format {
        Format::Zip => ZipBundle::open(bytes)?.entries().into_iter().map(|entry| (entry.name, entry.size)).collect(),
        Format::Tar => tar_entries(bytes),
        _ => gzip_name(bytes).map(|name| (name, 0)).into_iter().collect(),
    };
    let blocks: Vec<Block> = entries.iter().take(MAX_ENTRIES).map(|(name, _)| Block::new(BlockKind::ListItem, name.clone())).collect();
    let sections = if blocks.is_empty() { Vec::new() } else { vec![Section::new(None, blocks)] };
    Ok(Content { sections, notes: vec![ParseNote::ArchiveEntries(entries.len())], ..Content::default() })
}
