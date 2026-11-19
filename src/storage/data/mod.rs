//! The file edges of data ownership (sb F2): writing the export zip, and writing, reading
//! and staging encrypted snapshots. The decisions live in `domain::data`.

mod bundle;
mod snapshot;

pub use bundle::{write_bundle, BundleReport};
pub use snapshot::{create_snapshot, open_snapshot, pre_restore_path, staged_restore_path, StageFailure, StagedRestore};

use crate::error::AppError;

/// Maps a file-system error into the application's error.
fn io_err(err: impl ToString) -> AppError {
    AppError::DatabaseError(err.to_string())
}

#[cfg(test)]
mod tests;
