//! Zip containers (EPUB, DOCX, ODT, plain zip archives), read with a per-entry size cap so a
//! compressed bomb cannot exhaust memory.

use super::ParseError;
use std::io::{Cursor, Read};
use zip::ZipArchive;

/// Largest single entry ever decompressed (bytes).
const MAX_ENTRY_BYTES: u64 = 64 * 1024 * 1024;

/// An opened zip container over borrowed bytes.
pub struct ZipBundle<'a> {
    archive: ZipArchive<Cursor<&'a [u8]>>,
}

/// One listed entry: its path and uncompressed size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZipEntry {
    pub name: String,
    pub size: u64,
}

impl<'a> ZipBundle<'a> {
    pub fn open(bytes: &'a [u8]) -> Result<Self, ParseError> {
        let archive = ZipArchive::new(Cursor::new(bytes)).map_err(|err| ParseError::Unreadable(err.to_string()))?;
        Ok(Self { archive })
    }

    /// The bytes of the entry at `path` (exact, then case-insensitive), capped in size.
    pub fn read(&mut self, path: &str) -> Option<Vec<u8>> {
        let name = self.resolve(path)?;
        let entry = self.archive.by_name(&name).ok()?;
        if entry.encrypted() {
            return None;
        }
        let mut bytes = Vec::new();
        entry.take(MAX_ENTRY_BYTES).read_to_end(&mut bytes).ok()?;
        Some(bytes)
    }

    /// The entry at `path` as (lossy) UTF-8 text.
    pub fn read_text(&mut self, path: &str) -> Option<String> {
        self.read(path).map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
    }

    fn resolve(&self, path: &str) -> Option<String> {
        let path = path.trim_start_matches('/');
        if self.archive.index_for_name(path).is_some() {
            return Some(path.to_string());
        }
        self.archive.file_names().flatten().find(|name| name.eq_ignore_ascii_case(path)).map(|name| name.into_owned())
    }

    /// Every file entry (directories excluded), in archive order.
    pub fn entries(&mut self) -> Vec<ZipEntry> {
        (0..self.archive.len())
            .filter_map(|index| {
                let entry = self.archive.by_index_raw(index).ok()?;
                let name = entry.name().ok()?.into_owned();
                (!entry.is_dir()).then(|| ZipEntry { name, size: entry.size() })
            })
            .collect()
    }
}

/// Resolves `href` (relative, possibly percent-encoded, maybe with a `#fragment`) against the
/// folder of `base` inside the container.
pub fn resolve_path(base: &str, href: &str) -> String {
    let href = href.split('#').next().unwrap_or_default();
    let folder = base.rsplit_once('/').map_or("", |(folder, _)| folder);
    let joined = if folder.is_empty() || href.starts_with('/') { href.to_string() } else { format!("{folder}/{href}") };
    let mut parts: Vec<&str> = Vec::new();
    for part in joined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    percent_decode(&parts.join("/"))
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let hex = bytes.get(index + 1..index + 3).and_then(|h| std::str::from_utf8(h).ok());
        match (bytes[index], hex.and_then(|h| u8::from_str_radix(h, 16).ok())) {
            (b'%', Some(value)) => {
                out.push(value);
                index += 3;
            }
            (byte, _) => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}
