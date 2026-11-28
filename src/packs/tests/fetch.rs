//! Pack downloads through the icon library's fetch, pinned to the archive's SHA-256, served
//! by the local mock server.

use super::SAMPLE_SIMPACK;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::packs::{sha256_hex, DownloadablePack, PackError, PackRule};
use crate::icons::FetchError;
use crate::packs::{download_pack_blocking, download_source, PackDownloadError};
use std::sync::atomic::AtomicBool;

const ARCHIVE: &str = "application/zip";

fn listed(server: &MockServer, id: &str, sha256: String) -> DownloadablePack {
    DownloadablePack {
        id: id.into(),
        name: "Sample".into(),
        description: String::new(),
        version: "1.0.0".into(),
        url: format!("{}/sample.simpack", server.url),
        sha256,
        bytes: SAMPLE_SIMPACK.len() as u64,
    }
}

#[test]
fn a_pinned_archive_downloads_and_verifies() {
    let server = MockServer::start(vec![MockReply::raw(200, ARCHIVE, SAMPLE_SIMPACK.to_vec())]);
    let pack = listed(&server, "sample", sha256_hex(SAMPLE_SIMPACK));
    let mut progress = Vec::new();
    let verified = download_pack_blocking(&pack, true, &AtomicBool::new(false), |p| progress.push(p)).expect("downloaded");
    assert_eq!(verified.manifest.id, "sample");
    assert_eq!(progress.last().map(|p| p.received), Some(SAMPLE_SIMPACK.len() as u64));
}

#[test]
fn an_archive_that_does_not_match_its_pin_is_thrown_away() {
    let server = MockServer::start(vec![MockReply::raw(200, ARCHIVE, SAMPLE_SIMPACK.to_vec())]);
    let pack = listed(&server, "sample", sha256_hex(b"another archive"));
    assert_eq!(download_pack_blocking(&pack, true, &AtomicBool::new(false), |_| {}).err(), Some(PackDownloadError::Fetch(FetchError::Integrity)));
}

#[test]
fn a_listing_that_names_another_pack_or_no_checksum_is_refused() {
    let server = MockServer::start(vec![MockReply::raw(200, ARCHIVE, SAMPLE_SIMPACK.to_vec())]);
    let renamed = listed(&server, "other", sha256_hex(SAMPLE_SIMPACK));
    let refused = download_pack_blocking(&renamed, true, &AtomicBool::new(false), |_| {}).err();
    assert_eq!(refused, Some(PackDownloadError::Pack(PackError::Invalid(PackRule::Id, "sample".into()))));
    let unpinned = listed(&server, "sample", "nope".into());
    assert!(download_source(&unpinned, true).is_err());
    let public_only = download_source(&listed(&server, "sample", sha256_hex(SAMPLE_SIMPACK)), false).expect("source");
    assert!(!public_only.allow_private && public_only.integrity.starts_with("sha256-"));
}
