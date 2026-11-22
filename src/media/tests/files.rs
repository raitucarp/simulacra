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

#[test]
fn an_image_file_goes_through_the_thumbnail_pipeline() {
    use crate::domain::media::catalog::ImageScan;
    let dir = scratch_media_dir("file-image");
    let path = dir.join("wide.png");
    std::fs::write(&path, png_bytes(crate::media::images::THUMB_MAX_EDGE * 2, 64)).expect("write");
    let file = read_file_ref(&path).expect("read");
    let ImageScan::Known(facts) = crate::media::scan_image(&file.path, &dir) else { panic!("an image file scans") };
    assert!(facts.thumb_path.is_some_and(|thumb| std::path::Path::new(&thumb).is_file()), "a large image file gets a feed thumbnail");
}

#[test]
fn a_video_file_scans_to_its_length_and_a_gone_one_to_missing() {
    use crate::domain::media::files::{FilePreview, FileScan};
    let dir = scratch_media_dir("file-scan-video");
    let boxed = |kind: &[u8], body: &[u8]| [&(body.len() as u32 + 8).to_be_bytes()[..], kind, body].concat();
    let mvhd = boxed(b"mvhd", &[&[0u8; 12][..], &1000u32.to_be_bytes(), &42_000u32.to_be_bytes(), &[0u8; 80]].concat());
    let path = dir.join("talk.mp4");
    std::fs::write(&path, [boxed(b"ftyp", b"isom\0\0\0\0"), boxed(b"mdat", &[1u8; 4096]), boxed(b"moov", &mvhd)].concat()).expect("write");
    let file = read_file_ref(&path).expect("read");
    assert_eq!(file.mime, "video/mp4");
    let scan = crate::media::files::scan_file(&file, &dir);
    assert!(matches!(scan.preview(), Some(FilePreview::Video { seconds: Some(42), .. })), "{scan:?}");
    std::fs::remove_file(&path).expect("remove");
    assert_eq!(crate::media::files::scan_file(&file, &dir), FileScan::Missing);
}

#[test]
fn an_audio_file_scans_to_its_length() {
    use crate::domain::media::files::FilePreview;
    let dir = scratch_media_dir("file-scan-audio");
    let fmt = [&b"fmt "[..], &16u32.to_le_bytes(), &[1, 0, 1, 0], &8_000u32.to_le_bytes(), &16_000u32.to_le_bytes(), &[2, 0, 16, 0]].concat();
    let samples = vec![0u8; 16_000 * 4];
    let data = [&b"data"[..], &(samples.len() as u32).to_le_bytes(), &samples].concat();
    let path = dir.join("memo.wav");
    std::fs::write(&path, [&b"RIFF"[..], &((4 + fmt.len() + data.len()) as u32).to_le_bytes(), b"WAVE", &fmt, &data].concat()).expect("write");
    let file = read_file_ref(&path).expect("read");
    assert_eq!(file.mime, "audio/wav");
    let scan = crate::media::files::scan_file(&file, &dir);
    assert_eq!(scan.preview(), Some(&FilePreview::Audio { seconds: Some(4) }));
}

#[test]
fn a_code_file_scans_to_its_first_lines_in_its_language() {
    use crate::domain::media::files::{FilePreview, TEXT_PREVIEW_LINES};
    let dir = scratch_media_dir("file-scan-text");
    let path = dir.join("lib.rs");
    let source: String = (0..40).map(|n| format!("pub const N{n}: u32 = {n};\n")).collect();
    std::fs::write(&path, &source).expect("write");
    let file = read_file_ref(&path).expect("read");
    let Some(FilePreview::Text { lines, lang, more }) = crate::media::files::scan_file(&file, &dir).preview().cloned() else { panic!("a text preview") };
    assert_eq!((lines.lines().count(), lang.as_deref(), more), (TEXT_PREVIEW_LINES, Some("rs"), true));
    assert!(lines.starts_with("pub const N0: u32 = 0;"));
}
