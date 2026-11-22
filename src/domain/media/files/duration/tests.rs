//! L7, L8: playing times from hand-built container headers (no media fixtures needed).

use super::{media_duration, Bytes};

fn boxed(kind: &[u8], body: &[u8]) -> Vec<u8> {
    [&(body.len() as u32 + 8).to_be_bytes()[..], kind, body].concat()
}

/// A version 0 movie header: time scale and duration after version, flags and two dates.
fn mvhd(timescale: u32, duration: u32) -> Vec<u8> {
    boxed(b"mvhd", &[&[0u8; 12][..], &timescale.to_be_bytes(), &duration.to_be_bytes(), &[0u8; 80]].concat())
}

fn duration_of(mime: &str, bytes: &[u8]) -> Option<u32> {
    media_duration(mime, &mut Bytes(bytes))
}

#[test]
fn mp4_reads_the_movie_header_even_after_the_media_data() {
    let ftyp = boxed(b"ftyp", b"isom\0\0\0\0");
    let mdat = boxed(b"mdat", &[7u8; 5000]);
    let moov = boxed(b"moov", &mvhd(600, 600 * 95));
    assert_eq!(duration_of("video/mp4", &[ftyp.clone(), mdat, moov.clone()].concat()), Some(95));
    assert_eq!(duration_of("video/quicktime", &[ftyp.clone(), moov].concat()), Some(95));
    assert_eq!(duration_of("video/mp4", &ftyp), None, "no movie header");
}

#[test]
fn matroska_reads_the_segment_info() {
    // EBML header, then a Segment of unknown size holding Info: scale 1 ms, duration 61.5 s.
    let info_body = [&[0x2A, 0xD7, 0xB1, 0x83, 0x0F, 0x42, 0x40][..], &[0x44, 0x89, 0x88], &61_500f64.to_be_bytes()].concat();
    let info = [&[0x15, 0x49, 0xA9, 0x66, 0x80 | info_body.len() as u8][..], &info_body].concat();
    let segment = [&[0x18, 0x53, 0x80, 0x67, 0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF][..], &info].concat();
    let file = [&[0x1A, 0x45, 0xDF, 0xA3, 0x80][..], &segment].concat();
    assert_eq!(duration_of("video/webm", &file), Some(62));
}

#[test]
fn riff_reads_the_avi_header() {
    let avih = [&b"avih"[..], &56u32.to_le_bytes(), &40_000u32.to_le_bytes(), &[0u8; 12], &250u32.to_le_bytes(), &[0u8; 36]].concat();
    let avi = [&b"RIFF"[..], &0u32.to_le_bytes(), b"AVI LIST", &0u32.to_le_bytes(), b"hdrl", &avih].concat();
    assert_eq!(duration_of("video/x-msvideo", &avi), Some(10));
}

#[test]
fn unknown_or_broken_files_have_no_duration() {
    assert_eq!(duration_of("video/mp4", b""), None);
    assert_eq!(duration_of("application/pdf", b"%PDF-1.7"), None);
}
