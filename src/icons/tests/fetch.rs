//! The pinned package download (S10): integrity, progress, cancel and caps, served locally.

use super::fixture_package;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::icons::sha512_integrity;
use crate::icons::{fetch_package_blocking, FetchError, PackageSource, Progress, PACKAGE_INTEGRITY, PACKAGE_URL, PACKAGE_VERSION};
use std::sync::atomic::AtomicBool;

const TARBALL: &str = "application/octet-stream";

fn source(server: &MockServer, integrity: String) -> PackageSource {
    PackageSource { url: format!("{}/pkg.tgz", server.url), integrity, max_bytes: 1024 * 1024, allow_private: true }
}

#[test]
fn the_pin_is_one_registry_tarball_with_its_sha512() {
    assert!(PACKAGE_URL.starts_with("https://registry.npmjs.org/") && PACKAGE_URL.ends_with(&format!("-{PACKAGE_VERSION}.tgz")));
    assert!(PACKAGE_INTEGRITY.starts_with("sha512-") && PACKAGE_INTEGRITY.len() == "sha512-".len() + 88);
    assert!(!PackageSource::pinned().allow_private);
}

#[test]
fn a_matching_tarball_is_kept_and_progress_is_reported() {
    let package = fixture_package();
    let server = MockServer::start(vec![MockReply::raw(200, TARBALL, package.clone())]);
    let mut seen = Vec::new();
    let bytes = fetch_package_blocking(&source(&server, sha512_integrity(&package)), &AtomicBool::new(false), |p| seen.push(p)).expect("downloaded");
    assert_eq!(bytes, package);
    let last: Progress = *seen.last().expect("progress");
    assert_eq!((last.received, last.total), (package.len() as u64, Some(package.len() as u64)));
}

#[test]
fn a_tarball_that_does_not_match_its_sha512_is_refused() {
    let package = fixture_package();
    let server = MockServer::start(vec![MockReply::raw(200, TARBALL, package)]);
    let wrong = sha512_integrity(b"something else");
    assert_eq!(fetch_package_blocking(&source(&server, wrong), &AtomicBool::new(false), |_| {}), Err(FetchError::Integrity));
}

#[test]
fn cancel_status_size_and_private_hosts_stop_a_download() {
    let package = fixture_package();
    let integrity = sha512_integrity(&package);
    let server = MockServer::start(vec![MockReply::raw(200, TARBALL, package.clone()), MockReply::raw(404, TARBALL, "gone"), MockReply::raw(200, TARBALL, package.clone())]);
    assert_eq!(fetch_package_blocking(&source(&server, integrity.clone()), &AtomicBool::new(true), |_| {}), Err(FetchError::Cancelled));
    assert_eq!(fetch_package_blocking(&source(&server, integrity.clone()), &AtomicBool::new(false), |_| {}), Err(FetchError::Status(404)));
    let tiny = PackageSource { max_bytes: 16, ..source(&server, integrity.clone()) };
    assert_eq!(fetch_package_blocking(&tiny, &AtomicBool::new(false), |_| {}), Err(FetchError::TooLarge));
    let public_only = PackageSource { allow_private: false, ..source(&server, integrity) };
    assert!(matches!(fetch_package_blocking(&public_only, &AtomicBool::new(false), |_| {}), Err(FetchError::Network(_))));
}
