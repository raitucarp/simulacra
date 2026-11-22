//! Icon library IO (S10): the download against a local server, fed a tarball built here
//! from tiny hand-written fixtures. No test touches the network or the real app data folder.

mod fetch;

use flate2::write::GzEncoder;
use std::io::Write;

/// The fixture module of the line set `fi`, and the package licence.
pub(super) const LINE_SET_MODULE: &str = include_str!("fixtures/line_set.mjs");
pub(super) const PACKAGE_LICENCE: &str = include_str!("fixtures/LICENSE");

const BLOCK: usize = 512;

/// One `ustar` header for a regular file `path` of `size` bytes.
fn header(path: &str, size: usize) -> [u8; BLOCK] {
    let mut block = [0u8; BLOCK];
    block[..path.len()].copy_from_slice(path.as_bytes());
    block[100..108].copy_from_slice(b"0000644\0");
    block[124..136].copy_from_slice(format!("{size:011o}\0").as_bytes());
    block[136..148].copy_from_slice(b"00000000000\0");
    block[156] = b'0';
    block[257..263].copy_from_slice(b"ustar\0");
    block[263..265].copy_from_slice(b"00");
    block[148..156].copy_from_slice(b"        ");
    let sum: u32 = block.iter().map(|b| *b as u32).sum();
    block[148..156].copy_from_slice(format!("{sum:06o}\0 ").as_bytes());
    block
}

/// A gzip-compressed tar of `files`.
pub(super) fn tarball(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut tar = Vec::new();
    for (path, data) in files {
        tar.extend_from_slice(&header(path, data.len()));
        tar.extend_from_slice(data);
        tar.resize(tar.len().div_ceil(BLOCK) * BLOCK, 0);
    }
    tar.resize(tar.len() + 2 * BLOCK, 0);
    let mut gz = GzEncoder::new(Vec::new(), flate2::Compression::fast());
    gz.write_all(&tar).expect("compress");
    gz.finish().expect("finish")
}

/// The fixture package: the line set's module, a second set and the licence.
pub(super) fn fixture_package() -> Vec<u8> {
    tarball(&[
        ("package/LICENSE", PACKAGE_LICENCE.as_bytes()),
        ("package/fi/index.mjs", LINE_SET_MODULE.as_bytes()),
        ("package/lu/index.mjs", LINE_SET_MODULE.replace("Fi", "Lu").as_bytes()),
    ])
}

