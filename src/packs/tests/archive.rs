//! `.simpack` archives: the sample pack verifies, and a tampered, incomplete, mislabelled or
//! built-in archive is refused.

use super::SAMPLE_SIMPACK;
use crate::domain::packs::{sha256_hex, PackError, PackFile, PackManifest};
use crate::packs::{read_simpack, write_simpack};
use std::collections::BTreeMap;

fn manifest(id: &str, files: &BTreeMap<String, Vec<u8>>) -> PackManifest {
    let coverage = files
        .iter()
        .map(|(file, data)| (file.trim_end_matches(".webp").to_string(), PackFile { file: file.clone(), sha256: sha256_hex(data), width: 8, height: 8 }))
        .collect();
    PackManifest { id: id.into(), name: "Test".into(), version: "1.0.0".into(), coverage, ..PackManifest::default() }
}

fn files() -> BTreeMap<String, Vec<u8>> {
    BTreeMap::from([("spot/ideas.webp".to_string(), b"ideas".to_vec()), ("guide/cheer.webp".to_string(), b"cheer".to_vec())])
}

#[test]
fn the_sample_simpack_verifies_every_file() {
    let pack = read_simpack(SAMPLE_SIMPACK).expect("the tooling's archive is valid");
    assert_eq!(pack.manifest.id, "sample");
    assert!(pack.manifest.covers("spot/ideas") && pack.manifest.covers("portrait/face/round"));
    for entry in pack.manifest.coverage.values() {
        assert_eq!(sha256_hex(&pack.files[&entry.file]), entry.sha256);
    }
}

#[test]
fn an_archive_written_here_round_trips() {
    let files = files();
    let manifest = manifest("roundtrip", &files);
    let pack = read_simpack(&write_simpack(&manifest, &files).expect("zip")).expect("valid");
    assert_eq!((pack.manifest, pack.files), (manifest, files));
}

#[test]
fn a_file_that_does_not_match_its_checksum_is_refused() {
    let mut files = files();
    let manifest = manifest("tampered", &files);
    files.insert("spot/ideas.webp".into(), b"edited".to_vec());
    let archive = write_simpack(&manifest, &files).expect("zip");
    assert_eq!(read_simpack(&archive), Err(PackError::Checksum("spot/ideas.webp".into())));
}

#[test]
fn missing_files_bad_manifests_and_built_in_ids_are_refused() {
    let mut files = files();
    let manifest = manifest("partial", &files);
    files.remove("guide/cheer.webp");
    assert_eq!(read_simpack(&write_simpack(&manifest, &files).unwrap()), Err(PackError::Missing("guide/cheer.webp".into())));
    let flat = PackManifest { id: "flat".into(), ..manifest.clone() };
    assert_eq!(read_simpack(&write_simpack(&flat, &self::files()).unwrap()), Err(PackError::BuiltIn("flat".into())));
    let classic = PackManifest { id: "classic".into(), ..manifest };
    assert!(matches!(read_simpack(&write_simpack(&classic, &self::files()).unwrap()), Err(PackError::Invalid(..))));
    assert!(matches!(read_simpack(b"not a zip"), Err(PackError::Archive(_))));
}
