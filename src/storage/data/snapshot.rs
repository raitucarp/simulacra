//! Snapshot files: sealing a consistent database copy, and opening a snapshot into a staged
//! file beside the vault that is verified before anything is replaced.

use super::io_err;
use crate::domain::data::snapshot::{open, seal, KdfParams, SnapshotError};
use crate::error::{AppError, Result};
use crate::storage::db::{verify_database_file, DatabaseSummary};
use std::path::{Path, PathBuf};
use zeroize::Zeroizing;

/// Suffix of the decrypted database waiting for the user's confirmation.
const STAGED_SUFFIX: &str = ".restore";
/// Suffix of the copy of the database a restore replaced.
const PRE_RESTORE_SUFFIX: &str = ".pre-restore";
const STAMP_FORMAT: &str = "%Y%m%d-%H%M%S";
/// Suffix of a snapshot while it is being written.
const PARTIAL_SUFFIX: &str = ".partial";

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}

/// Where a verified snapshot waits for confirmation (beside the database).
pub fn staged_restore_path(db_path: &Path) -> PathBuf {
    with_suffix(db_path, STAGED_SUFFIX)
}

/// Where the database is kept before a restore replaces it (a time-stamped name when an
/// earlier copy is already there).
pub fn pre_restore_path(db_path: &Path) -> PathBuf {
    let plain = with_suffix(db_path, PRE_RESTORE_SUFFIX);
    if !plain.exists() {
        return plain;
    }
    with_suffix(db_path, &format!(".{}{PRE_RESTORE_SUFFIX}", crate::time::local_now().strftime(STAMP_FORMAT)))
}

fn snapshot_error(err: SnapshotError) -> AppError {
    AppError::DatabaseError(err.key().to_string())
}

/// Seals the database copy at `copy` into `out` with `passphrase`, then deletes the copy.
/// Returns the snapshot's size in bytes.
pub fn create_snapshot(copy: &Path, out: &Path, passphrase: &Zeroizing<String>, created_at: i64) -> Result<u64> {
    let plain = std::fs::read(copy).map(Zeroizing::new);
    let _ = std::fs::remove_file(copy);
    let sealed = seal(&plain.map_err(io_err)?, passphrase, KdfParams::DEFAULT, created_at).map_err(snapshot_error)?;
    let partial = with_suffix(out, PARTIAL_SUFFIX);
    std::fs::write(&partial, &sealed).and_then(|()| std::fs::rename(&partial, out)).map_err(|err| {
        let _ = std::fs::remove_file(&partial);
        io_err(err)
    })?;
    Ok(sealed.len() as u64)
}

/// A snapshot opened, decrypted and checked, waiting in `path` for the user's go-ahead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedRestore {
    pub path: PathBuf,
    /// When the snapshot was made (Unix seconds, from its authenticated header).
    pub created_at: i64,
    pub summary: DatabaseSummary,
}

/// Why a snapshot could not be staged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageFailure {
    Snapshot(SnapshotError),
    File(String),
    /// It decrypted, but is not a sound Simulacra database.
    NotAVault(String),
}

/// Opens the snapshot `file` with `passphrase` into `staged` and verifies the database.
/// Nothing beside `staged` is touched; a failed check removes it again.
pub fn open_snapshot(file: &Path, passphrase: &Zeroizing<String>, staged: &Path) -> std::result::Result<StagedRestore, StageFailure> {
    let bytes = std::fs::read(file).map_err(|err| StageFailure::File(err.to_string()))?;
    let opened = open(&bytes, passphrase).map_err(StageFailure::Snapshot)?;
    let _ = std::fs::remove_file(staged);
    std::fs::write(staged, opened.plaintext.as_slice()).map_err(|err| StageFailure::File(err.to_string()))?;
    match verify_database_file(staged) {
        Ok(summary) => Ok(StagedRestore { path: staged.to_path_buf(), created_at: opened.header.created_at, summary }),
        Err(err) => {
            let _ = std::fs::remove_file(staged);
            Err(StageFailure::NotAVault(err.to_string()))
        }
    }
}
