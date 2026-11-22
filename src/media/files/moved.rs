//! L11: finding a file that was renamed where it was: one of the same size and content hash
//! in the folder the post says it was in.

use crate::domain::media::FileRef;
use std::path::Path;

/// Entries of the old folder looked at, at most, and same-size files hashed.
const MAX_ENTRIES: usize = 2048;
const MAX_HASHED: usize = 8;

/// A file in the folder `file` was in with its size and content: where it went when renamed.
pub fn find_renamed(file: &FileRef) -> Option<String> {
    let folder = Path::new(&file.path).parent()?;
    if file.sha256.is_empty() || file.size == 0 {
        return None;
    }
    let same_size = std::fs::read_dir(folder).ok()?.flatten().take(MAX_ENTRIES).filter(|entry| entry.metadata().is_ok_and(|meta| meta.is_file() && meta.len() == file.size));
    same_size.take(MAX_HASHED).map(|entry| entry.path()).find(|path| super::read::sniff_and_hash(path).is_ok_and(|(_, hash)| hash == file.sha256)).map(|path| path.display().to_string())
}
