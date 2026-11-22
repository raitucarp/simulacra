//! L3: reading a local file's reference in a scratch folder (never the app's data folder).

use super::{png_bytes, scratch_media_dir};
use crate::import::parse::sha256_hex;
use crate::media::files::read_file_ref;

#[test]
fn a_file_ref_records_size_date_type_and_hash() {
    let dir = scratch_media_dir("file-ref");
    let bytes = png_bytes(8, 8);
    let path = dir.join("diagram.txt");
    std::fs::write(&path, &bytes).expect("write");
    let file = read_file_ref(&path).expect("read");
    assert_eq!(file.mime, "image/png", "the bytes say PNG whatever the name says");
    assert_eq!((file.size, file.sha256.as_str()), (bytes.len() as u64, sha256_hex(&bytes).as_str()));
    assert!(file.modified.is_some_and(|at| at <= jiff::Timestamp::now()));
    assert!(std::path::Path::new(&file.path).is_absolute() && file.name() == "diagram.txt");
}

#[test]
fn large_files_are_hashed_whole_and_typed_by_their_head() {
    let dir = scratch_media_dir("file-ref-large");
    let mut bytes = b"fn main() {}\n".repeat(20_000);
    bytes.extend_from_slice(b"// the end\n");
    let path = dir.join("main.rs");
    std::fs::write(&path, &bytes).expect("write");
    let file = read_file_ref(&path).expect("read");
    assert_eq!((file.mime.as_str(), file.sha256), ("text/plain", sha256_hex(&bytes)));
}

#[test]
fn missing_files_and_folders_are_refused() {
    let dir = scratch_media_dir("file-ref-missing");
    assert!(read_file_ref(&dir.join("gone.pdf")).is_err());
    assert!(read_file_ref(&dir).is_err());
}

#[test]
fn files_that_are_not_there_are_never_handed_to_the_system() {
    let dir = scratch_media_dir("file-open-missing");
    let gone = dir.join("moved-away.pdf");
    assert!(crate::media::files::open_file(&gone).is_err());
    assert!(crate::media::files::show_in_folder(&gone).is_err());
}
