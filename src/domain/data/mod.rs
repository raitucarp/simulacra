//! Data ownership (sb F2): version history, collections and bookmarks, the export bundle and
//! encrypted snapshots. Everything here is pure; files, the database and the OS random
//! generator stay at the edges (`storage::data`, `storage::snapshot`, the shell).

pub mod export;
pub mod history;
pub mod saved;
pub mod snapshot;
