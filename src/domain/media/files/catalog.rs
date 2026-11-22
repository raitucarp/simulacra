//! The file side of the media cache: the scan of each attached file by path, and the paths
//! being scanned. Scans are not stored in the vault: a file can change at any time, so each
//! session looks again.

use super::scan::FileScan;
use crate::domain::media::MediaCatalog;
use std::collections::{HashMap, HashSet};

/// Scanned files and the scans running.
#[derive(Debug, Clone, Default)]
pub struct FileShelf {
    scans: HashMap<String, FileScan>,
    scanning: HashSet<String>,
}

impl MediaCatalog {
    /// The scan of the file at `path`, once done.
    pub fn file(&self, path: &str) -> Option<&FileScan> {
        self.files.scans.get(path)
    }

    /// Up to `limit` of `paths` never scanned and not being scanned, without duplicates.
    pub fn files_due<'a>(&self, paths: impl IntoIterator<Item = &'a str>, limit: usize) -> Vec<String> {
        let mut due: Vec<String> = Vec::new();
        for path in paths.into_iter().filter(|path| !self.files.scanning.contains(*path) && !self.files.scans.contains_key(*path)) {
            if due.len() == limit {
                break;
            }
            if !due.iter().any(|seen| seen == path) {
                due.push(path.to_string());
            }
        }
        due
    }

    /// How many file scans are running.
    pub fn files_in_flight(&self) -> usize {
        self.files.scanning.len()
    }

    /// The catalog with `paths` marked as being scanned.
    #[must_use]
    pub fn files_started(mut self, paths: &[String]) -> Self {
        self.files.scanning.extend(paths.iter().cloned());
        self
    }

    /// The catalog with a finished scan of the file at `path` stored.
    #[must_use]
    pub fn with_file(mut self, path: String, scan: FileScan) -> Self {
        self.files.scanning.remove(&path);
        self.files.scans.insert(path, scan);
        self.revision += 1;
        self
    }

    /// The catalog with the scan of `path` forgotten, so the file is scanned again (L11: after
    /// a relink).
    #[must_use]
    pub fn without_file(mut self, path: &str) -> Self {
        self.files.scans.remove(path);
        self.revision += 1;
        self
    }
}
