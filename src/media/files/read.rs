//! L3: reading what a post keeps about a local file: its size, modification time, type (by
//! its leading bytes, then its extension) and content hash. Blocking; run off the UI thread.

use crate::domain::media::files::{detect_mime, SNIFF_BYTES};
use crate::domain::media::FileRef;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;

/// Bytes read per step while hashing.
const READ_CHUNK: usize = 64 * 1024;

/// The reference a post keeps to the file at `path` (made absolute).
pub fn read_file_ref(path: &Path) -> Result<FileRef, String> {
    let path = std::path::absolute(path).map_err(|err| err.to_string())?;
    let meta = std::fs::metadata(&path).map_err(|err| err.to_string())?;
    if !meta.is_file() {
        return Err(format!("{} is not a file", path.display()));
    }
    let (head, sha256) = sniff_and_hash(&path)?;
    let name = path.to_string_lossy().into_owned();
    let modified = meta.modified().ok().and_then(|time| jiff::Timestamp::try_from(time).ok());
    Ok(FileRef { mime: detect_mime(&name, &head), path: name, size: meta.len(), modified, sha256 })
}

/// The first [`SNIFF_BYTES`] of the file and the hex SHA-256 of all of it.
pub(super) fn sniff_and_hash(path: &Path) -> Result<(Vec<u8>, String), String> {
    let mut file = std::fs::File::open(path).map_err(|err| err.to_string())?;
    let (mut hasher, mut head, mut buffer) = (Sha256::new(), Vec::new(), vec![0u8; READ_CHUNK]);
    loop {
        let read = file.read(&mut buffer).map_err(|err| err.to_string())?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        let room = SNIFF_BYTES.saturating_sub(head.len());
        head.extend_from_slice(&buffer[..read.min(room)]);
    }
    Ok((head, hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect()))
}
