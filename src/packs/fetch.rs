//! Downloading a pack from the built-in list: the icon library's download (chunked, capped,
//! cancellable, no cookies or referrer) pinned to the archive's SHA-256, then the archive's
//! own per-file check.

use super::archive::{read_simpack, VerifiedPack};
use crate::domain::packs::{sha256_integrity, DownloadablePack, PackError, PackRule};
use crate::icons::{fetch_package_blocking, FetchError, PackageSource, Progress};
use std::sync::atomic::AtomicBool;

/// Why a pack download failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackDownloadError {
    /// The transfer failed or the archive did not match its pinned SHA-256.
    Fetch(FetchError),
    /// The archive arrived intact but is not an installable pack.
    Pack(PackError),
}

/// What to fetch for `pack`; local addresses only when `allow_private` (tests).
pub fn download_source(pack: &DownloadablePack, allow_private: bool) -> Result<PackageSource, PackError> {
    let integrity = sha256_integrity(&pack.sha256).ok_or_else(|| PackError::Invalid(PackRule::Checksum, pack.id.clone()))?;
    Ok(PackageSource { url: pack.url.clone(), integrity, max_bytes: pack.bytes, allow_private })
}

/// Downloads and verifies `pack` on the calling (background) thread.
pub fn download_pack_blocking(
    pack: &DownloadablePack,
    allow_private: bool,
    cancel: &AtomicBool,
    on_progress: impl FnMut(Progress),
) -> Result<VerifiedPack, PackDownloadError> {
    let source = download_source(pack, allow_private).map_err(PackDownloadError::Pack)?;
    let bytes = fetch_package_blocking(&source, cancel, on_progress).map_err(PackDownloadError::Fetch)?;
    let verified = read_simpack(&bytes).map_err(PackDownloadError::Pack)?;
    if verified.manifest.id != pack.id {
        return Err(PackDownloadError::Pack(PackError::Invalid(PackRule::Id, verified.manifest.id)));
    }
    Ok(verified)
}
