//! Downloading the pinned icon package from the npm registry (S10): one tarball, at a fixed
//! version, read in chunks (progress after each, cancellable between them), capped in size
//! and kept only when its SHA-512 matches the pinned integrity. Nothing about the user is
//! sent: no cookies, no referrer, a generic user agent.

use crate::agent::tools::http::checked_url;
use crate::agent::tools::FetchPolicy;
use crate::domain::icons::matches_integrity;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// The pinned package version, its tarball and npm's integrity string for it.
pub const PACKAGE_VERSION: &str = "5.7.0";
pub const PACKAGE_URL: &str = "https://registry.npmjs.org/react-icons/-/react-icons-5.7.0.tgz";
pub const PACKAGE_INTEGRITY: &str = "sha512-LBLy340Rzqy6+/yVhZKT3B/QpP1BZaesGqasf09HPOBzRarcDIFH0WwXlXQfE7q7ipxK4MSiC5DIBWURCny6fw==";
/// The tarball's size (what Settings › Symbols says a download costs).
pub const PACKAGE_BYTES: u64 = 22_766_600;
/// Bytes read at most before a download is given up as too large.
pub const MAX_PACKAGE_BYTES: u64 = 64 * 1024 * 1024;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// Longest wait for the next chunk.
const READ_TIMEOUT: Duration = Duration::from_secs(30);
const USER_AGENT: &str = "Simulacra/1.0";

/// What to download.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSource {
    pub url: String,
    pub integrity: String,
    pub max_bytes: u64,
    /// Local addresses are refused unless allowed (tests serve from `127.0.0.1`).
    pub allow_private: bool,
}

impl PackageSource {
    /// The pinned package.
    pub fn pinned() -> Self {
        Self { url: PACKAGE_URL.into(), integrity: PACKAGE_INTEGRITY.into(), max_bytes: MAX_PACKAGE_BYTES, allow_private: false }
    }
}

/// How far a download is: bytes received and, when the server says, the total.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Progress {
    pub received: u64,
    pub total: Option<u64>,
}

/// Why a download failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    /// The URL, the connection or the transfer failed.
    Network(String),
    /// The server answered with this status.
    Status(u16),
    /// More than the cap arrived.
    TooLarge,
    /// The tarball does not match the pinned SHA-512.
    Integrity,
    Cancelled,
}

/// Downloads and verifies `source`, reporting progress; stops early once `cancel` is set.
pub async fn fetch_package(source: &PackageSource, cancel: &AtomicBool, mut on_progress: impl FnMut(Progress)) -> Result<Vec<u8>, FetchError> {
    let policy = FetchPolicy { allow_private_hosts: source.allow_private, ..FetchPolicy::default() };
    let url = checked_url(&source.url, &policy).map_err(FetchError::Network)?;
    let client = reqwest::Client::builder().connect_timeout(CONNECT_TIMEOUT).read_timeout(READ_TIMEOUT).referer(false).user_agent(USER_AGENT).build().map_err(|e| FetchError::Network(e.to_string()))?;
    let mut response = client.get(url).send().await.map_err(|e| FetchError::Network(e.to_string()))?;
    if !response.status().is_success() {
        return Err(FetchError::Status(response.status().as_u16()));
    }
    let total = response.content_length();
    if total.is_some_and(|total| total > source.max_bytes) {
        return Err(FetchError::TooLarge);
    }
    let mut body = Vec::with_capacity(total.unwrap_or_default().min(source.max_bytes) as usize);
    while let Some(chunk) = response.chunk().await.map_err(|e| FetchError::Network(e.to_string()))? {
        if cancel.load(Ordering::Relaxed) {
            return Err(FetchError::Cancelled);
        }
        body.extend_from_slice(&chunk);
        if body.len() as u64 > source.max_bytes {
            return Err(FetchError::TooLarge);
        }
        on_progress(Progress { received: body.len() as u64, total });
    }
    if !matches_integrity(&body, &source.integrity) {
        return Err(FetchError::Integrity);
    }
    Ok(body)
}

/// [`fetch_package`] on a runtime of its own, for a background thread.
pub fn fetch_package_blocking(source: &PackageSource, cancel: &AtomicBool, on_progress: impl FnMut(Progress)) -> Result<Vec<u8>, FetchError> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| FetchError::Network(e.to_string()))?;
    runtime.block_on(fetch_package(source, cancel, on_progress))
}
