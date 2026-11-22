//! The package's integrity form and the tarball reader (S10).

use crate::domain::icons::{matches_integrity, read_files, sha512_integrity, ArchiveError};

#[test]
fn integrity_is_npm_sha512_form() {
    // SHA-512 of the empty input, base64.
    let empty = "sha512-z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==";
    assert_eq!(sha512_integrity(b""), empty);
    assert!(matches_integrity(b"", empty));
    assert!(!matches_integrity(b"x", empty));
    assert!(!matches_integrity(b"", &empty.replace("sha512-", "sha1-")), "only SHA-512 is accepted");
}

#[test]
fn damaged_or_truncated_archives_are_refused() {
    assert_eq!(read_files(b"not a tarball", |_| true), Err(ArchiveError::Corrupt));
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    std::io::Write::write_all(&mut gz, &[1u8; 100]).unwrap();
    assert_eq!(read_files(&gz.finish().unwrap(), |_| true), Err(ArchiveError::Corrupt), "shorter than one header");
}
