//! Illustration packs at the edges (WS5): reading and verifying `.simpack` archives, the
//! store of installed packs next to the vault, and downloads from the built-in list. Every
//! function here blocks; the shell runs them on a background thread. The manifest rules are
//! pure and live in [`crate::domain::packs`].

pub mod archive;
pub mod fetch;
pub mod store;
#[cfg(test)]
mod tests;

pub use archive::{read_simpack, verified_files, write_simpack, VerifiedPack};
pub use fetch::{download_pack_blocking, download_source, PackDownloadError};
pub use store::{packs_dir_for, InstalledPack, PackStore, PACKS_DIR_NAME};
