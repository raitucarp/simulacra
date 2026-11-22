//! The downloadable icon library (decision #19): its sets are fetched on demand from
//! Settings › Symbols, and the projects behind them are named only on the Licences screen.
//!
//! Pure parts; the network and the files are in [`crate::icons`]:
//! - [`integrity`]: the tarball's SHA-512 in npm's `sha512-…` form (S10).
//! - [`archive`]: the wanted files out of the gzip-compressed tarball (S10).

pub mod archive;
pub mod integrity;
#[cfg(test)]
mod tests;

pub use archive::{read_files, ArchiveError, MAX_ENTRY_BYTES};
pub use integrity::{matches_integrity, sha512_integrity};
