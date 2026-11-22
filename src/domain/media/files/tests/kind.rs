//! L3: a file's type from its bytes first, then its extension.

use super::super::{detect_mime, file_family, UNKNOWN_MIME};
use crate::import::FormatFamily;

#[test]
fn magic_bytes_win_over_the_extension() {
    assert_eq!(detect_mime("photo.txt", b"\x89PNG\r\n\x1a\n...."), "image/png");
    assert_eq!(detect_mime("paper", b"%PDF-1.7\n"), "application/pdf");
    assert_eq!(detect_mime("song.bin", b"ID3\x04\x00\x00\x00\x00\x00\x00"), "audio/mpeg");
}

#[test]
fn iso_media_brands_tell_audio_and_quicktime_apart() {
    let ftyp = |brand: &[u8]| [&[0u8, 0, 0, 24][..], &b"ftyp"[..], brand, &[0u8; 12][..]].concat();
    assert_eq!(detect_mime("a.mp4", &ftyp(b"isom")), "video/mp4");
    assert_eq!(detect_mime("a.m4a", &ftyp(b"M4A ")), "audio/mp4");
    assert_eq!(detect_mime("a.mov", &ftyp(b"qt  ")), "video/quicktime");
    assert_eq!(detect_mime("clip.webm", b"\x1A\x45\xDF\xA3\x9f"), "video/webm");
    assert_eq!(detect_mime("clip.mkv", b"\x1A\x45\xDF\xA3\x9f"), "video/x-matroska");
}

#[test]
fn without_a_signature_the_extension_and_then_the_content_decide() {
    assert_eq!(detect_mime("voice.opus", &[0x00, 0x01, 0xFE]), "audio/ogg");
    assert_eq!(detect_mime("notes.md", b"# Title\nbody"), "text/markdown");
    assert_eq!(detect_mime("main.rs", b"fn main() {}\n"), "text/plain");
    assert_eq!(detect_mime("blob.dat", &[0x00, 0x9F, 0x01, 0xFE, 0x00]), UNKNOWN_MIME);
    assert_eq!(detect_mime("empty", b""), UNKNOWN_MIME);
}

#[test]
fn families_follow_the_mime_type() {
    assert_eq!(file_family("application/pdf"), FormatFamily::Pdf);
    assert_eq!(file_family("application/epub+zip"), FormatFamily::Book);
    assert_eq!(file_family("video/webm"), FormatFamily::Video);
    assert_eq!(file_family("audio/mp4"), FormatFamily::Audio);
    assert_eq!(file_family("text/plain; charset=utf-8"), FormatFamily::Text);
    assert_eq!(file_family("image/svg+xml"), FormatFamily::Image);
    assert_eq!(file_family(""), FormatFamily::Unknown);
    assert_eq!(file_family(UNKNOWN_MIME), FormatFamily::Unknown);
}
