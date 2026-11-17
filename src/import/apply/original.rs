//! Keeping the original file: copied under `<media>/imports/`, named by its checksum so the
//! same file is stored once, and never larger than the import size cap.

use crate::error::{AppError, Result};
use std::path::{Path, PathBuf};

/// Folder under the media folder that holds kept originals.
pub const IMPORTS_DIR_NAME: &str = "imports";
/// Bytes per megabyte of the size preference.
pub const BYTES_PER_MB: u64 = 1024 * 1024;
/// Characters of the checksum used in the stored file name.
const NAME_HASH_CHARS: usize = 16;

/// The cap in bytes for a size preference in megabytes.
pub fn max_bytes(max_file_mb: u32) -> u64 {
    u64::from(max_file_mb) * BYTES_PER_MB
}

/// Where the original with `sha256` and `extension` is kept (pure).
pub fn original_path(media_dir: &Path, sha256: &str, extension: Option<&str>) -> PathBuf {
    let stem: String = sha256.chars().take(NAME_HASH_CHARS).collect();
    let name = match extension.filter(|ext| ext.chars().all(char::is_alphanumeric) && !ext.is_empty()) {
        Some(ext) => format!("{stem}.{ext}"),
        None => stem,
    };
    media_dir.join(IMPORTS_DIR_NAME).join(name)
}

/// Writes `bytes` to `path` unless it is over `max_bytes` or already there.
pub fn keep_original(path: &Path, bytes: &[u8], max_bytes: u64) -> Result<PathBuf> {
    if bytes.len() as u64 > max_bytes {
        return Err(AppError::DatabaseError(format!("{} bytes over the {max_bytes} byte cap", bytes.len())));
    }
    if !path.is_file() {
        let folder = path.parent().ok_or_else(|| AppError::DatabaseError(path.display().to_string()))?;
        std::fs::create_dir_all(folder).map_err(|err| AppError::DatabaseError(err.to_string()))?;
        std::fs::write(path, bytes).map_err(|err| AppError::DatabaseError(err.to_string()))?;
    }
    Ok(path.to_path_buf())
}
