//! Illustration pack guard (WS5): every pack shipped under `assets/packs/` has a valid
//! `pack.json` whose keys are Classic keys (or new art under an allowed prefix, see
//! `tools/illustrations/README.md`), whose files exist and match their pinned SHA-256, and
//! whose images have the size the manifest states; the downloadable list only holds safe
//! entries.

use simulacra::domain::packs::{is_new_art_key, parse_downloadable, sha256_hex, PackManifest, DOWNLOADABLE_JSON, FLAT_PACK_ID, MANIFEST_FILE, NEW_ART_PREFIXES};
use std::fs;
use std::path::{Path, PathBuf};

const PACKS_DIR: &str = "assets/packs";
const CLASSIC_DIR: &str = "assets/illustrations";

/// Every shipped pack folder (a folder holding a `pack.json`).
fn pack_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(PACKS_DIR).expect("assets/packs").flatten().map(|e| e.path()).filter(|p| p.join(MANIFEST_FILE).is_file()).collect();
    dirs.sort();
    dirs
}

fn manifest(dir: &Path) -> PackManifest {
    let json = fs::read_to_string(dir.join(MANIFEST_FILE)).expect("pack.json");
    PackManifest::parse(&json).unwrap_or_else(|e| panic!("{}: {e:?}", dir.display()))
}

#[test]
fn the_built_in_flat_pack_is_shipped() {
    let dirs = pack_dirs();
    assert!(dirs.iter().any(|dir| dir.ends_with(FLAT_PACK_ID)), "{dirs:?}");
    assert_eq!(manifest(&Path::new(PACKS_DIR).join(FLAT_PACK_ID)).id, FLAT_PACK_ID);
}

#[test]
fn every_pack_key_is_a_classic_key_or_allowed_new_art() {
    let mut stray = Vec::new();
    for dir in pack_dirs() {
        for key in manifest(&dir).coverage.keys() {
            let classic = Path::new(CLASSIC_DIR).join(format!("{key}.svg")).is_file();
            if !classic && !is_new_art_key(key) {
                stray.push(format!("{}: {key}", dir.display()));
            }
        }
    }
    assert!(stray.is_empty(), "keys with no Classic template (allowed new-art prefixes: {NEW_ART_PREFIXES:?}):\n{}", stray.join("\n"));
}

#[test]
fn every_pack_file_exists_matches_its_checksum_and_size() {
    let mut problems = Vec::new();
    for dir in pack_dirs() {
        let manifest = manifest(&dir);
        assert_eq!(dir.file_name().and_then(|n| n.to_str()), Some(manifest.id.as_str()), "a pack's folder is named after its id");
        for (key, entry) in &manifest.coverage {
            let Ok(bytes) = fs::read(dir.join(&entry.file)) else {
                problems.push(format!("{key}: missing {}", entry.file));
                continue;
            };
            if sha256_hex(&bytes) != entry.sha256 {
                problems.push(format!("{key}: checksum differs from pack.json"));
            }
            match image::load_from_memory(&bytes) {
                Ok(image) if (image.width(), image.height()) == (entry.width, entry.height) => {}
                Ok(image) => problems.push(format!("{key}: {}×{} but pack.json says {}×{}", image.width(), image.height(), entry.width, entry.height)),
                Err(err) => problems.push(format!("{key}: not decodable ({err})")),
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn the_downloadable_list_parses_and_holds_only_valid_entries() {
    let raw: serde_json::Value = serde_json::from_str(DOWNLOADABLE_JSON).expect("assets/packs/downloadable.json is JSON");
    let listed = raw["packs"].as_array().expect("a packs array").len();
    assert_eq!(parse_downloadable(DOWNLOADABLE_JSON).len(), listed, "every listed pack is valid (https, pinned sha256, size, id)");
}
