//! Reading the files the icon library needs out of the package tarball (a gzip-compressed
//! `ustar` archive, S10/S11). Only regular files whose path the caller asks for are kept, each
//! capped in size; nothing is ever written or run from the archive.

use std::collections::HashMap;
use std::io::Read;

/// One tar block.
const BLOCK: usize = 512;
/// Largest single file kept (the biggest icon module is about 7 MB).
pub const MAX_ENTRY_BYTES: u64 = 16 * 1024 * 1024;
/// Header fields: name, size and type flag offsets, and the `ustar` prefix.
const NAME: (usize, usize) = (0, 100);
const SIZE: (usize, usize) = (124, 136);
const TYPE_FLAG: usize = 156;
const PREFIX: (usize, usize) = (345, 500);
/// Type flags of a regular file.
const REGULAR: [u8; 2] = [b'0', 0];

/// Why an archive could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveError {
    /// The gzip stream or a tar header is damaged, or the archive ends early.
    Corrupt,
    /// A wanted file is larger than [`MAX_ENTRY_BYTES`].
    TooLarge(String),
}

/// The files of the gzip-compressed tar `bytes` for which `wanted(path)` holds, by path.
pub fn read_files(bytes: &[u8], wanted: impl Fn(&str) -> bool) -> Result<HashMap<String, Vec<u8>>, ArchiveError> {
    let mut tar = flate2::read::GzDecoder::new(bytes);
    let mut files = HashMap::new();
    let mut header = [0u8; BLOCK];
    loop {
        tar.read_exact(&mut header).map_err(|_| ArchiveError::Corrupt)?;
        if header.iter().all(|b| *b == 0) {
            return Ok(files);
        }
        let size = octal(&header[SIZE.0..SIZE.1]).ok_or(ArchiveError::Corrupt)?;
        let path = entry_path(&header);
        let padded = size.div_ceil(BLOCK as u64) * BLOCK as u64;
        if REGULAR.contains(&header[TYPE_FLAG]) && wanted(&path) {
            if size > MAX_ENTRY_BYTES {
                return Err(ArchiveError::TooLarge(path));
            }
            let mut data = vec![0u8; padded as usize];
            tar.read_exact(&mut data).map_err(|_| ArchiveError::Corrupt)?;
            data.truncate(size as usize);
            files.insert(path, data);
        } else {
            let skipped = std::io::copy(&mut (&mut tar).take(padded), &mut std::io::sink()).map_err(|_| ArchiveError::Corrupt)?;
            if skipped != padded {
                return Err(ArchiveError::Corrupt);
            }
        }
    }
}

/// The entry's path: the `ustar` prefix, a slash and the name (or the name alone).
fn entry_path(header: &[u8; BLOCK]) -> String {
    let name = text(&header[NAME.0..NAME.1]);
    let prefix = text(&header[PREFIX.0..PREFIX.1]);
    if prefix.is_empty() { name } else { format!("{prefix}/{name}") }
}

/// A NUL-terminated header field as text.
fn text(field: &[u8]) -> String {
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    String::from_utf8_lossy(&field[..end]).into_owned()
}

/// A NUL- or space-terminated octal number.
fn octal(field: &[u8]) -> Option<u64> {
    let digits: String = field.iter().map(|b| *b as char).filter(|c| !matches!(c, '\0' | ' ')).collect();
    if digits.is_empty() { Some(0) } else { u64::from_str_radix(&digits, 8).ok() }
}
