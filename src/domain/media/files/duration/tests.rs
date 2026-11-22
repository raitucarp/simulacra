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
    assert_eq!(duration_of("audio/mp4", &[ftyp.clone(), moov].concat()), Some(95));
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
fn riff_reads_wav_and_avi_headers() {
    let fmt = [&b"fmt "[..], &16u32.to_le_bytes(), &[1, 0, 2, 0], &44_100u32.to_le_bytes(), &176_400u32.to_le_bytes(), &[4, 0, 16, 0]].concat();
    let data = [&b"data"[..], &(176_400u32 * 3).to_le_bytes()].concat();
    let wav = [&b"RIFF"[..], &0u32.to_le_bytes(), b"WAVE", &fmt, &data].concat();
    assert_eq!(duration_of("audio/wav", &wav), Some(3));
    let avih = [&b"avih"[..], &56u32.to_le_bytes(), &40_000u32.to_le_bytes(), &[0u8; 12], &250u32.to_le_bytes(), &[0u8; 36]].concat();
    let avi = [&b"RIFF"[..], &0u32.to_le_bytes(), b"AVI LIST", &0u32.to_le_bytes(), b"hdrl", &avih].concat();
    assert_eq!(duration_of("video/x-msvideo", &avi), Some(10));
}

#[test]
fn flac_and_ogg_read_their_stream_headers() {
    // 48 kHz, 2 channels, 16 bits, 48 000 * 7 samples.
    let samples: u64 = 48_000 * 7;
    let packed = [(48_000u32 >> 12) as u8, (48_000u32 >> 4) as u8, ((48_000u32 & 0xF) << 4) as u8 | (1 << 1), (15 << 4) | (samples >> 32) as u8];
    let flac = [&b"fLaC"[..], &[0x80, 0, 0, 34], &[0u8; 10], &packed, &(samples as u32).to_be_bytes(), &[0u8; 16]].concat();
    assert_eq!(duration_of("audio/flac", &flac), Some(7));
    let page = |granule: u64, body: &[u8]| [&b"OggS"[..], &[0, 2], &granule.to_le_bytes(), &[0u8; 12], &[1, body.len() as u8], body].concat();
    let ident = [&b"\x01vorbis"[..], &[0u8; 4], &[2], &44_100u32.to_le_bytes(), &[0u8; 14]].concat();
    let ogg = [page(0, &ident), page(44_100 * 12, b"audio")].concat();
    assert_eq!(duration_of("audio/ogg", &ogg), Some(12));
}

#[test]
fn mp3_uses_the_xing_frame_count_or_the_bit_rate() {
    // MPEG-1 layer III, 128 kbps, 44.1 kHz, stereo.
    let header = [0xFF, 0xFB, 0x90, 0x00];
    let xing = [&header[..], &[0u8; 32], b"Xing", &1u32.to_be_bytes(), &1000u32.to_be_bytes()].concat();
    let id3 = [&b"ID3"[..], &[4, 0, 0, 0, 0, 0, 10], &[0u8; 10]].concat();
    assert_eq!(duration_of("audio/mpeg", &[id3, xing].concat()), Some(26), "1000 frames of 1152 samples");
    let cbr = [&header[..], &vec![0u8; 16_000 * 5 - 4]].concat();
    assert_eq!(duration_of("audio/mpeg", &cbr), Some(5), "16 000 bytes a second at 128 kbps");
}

#[test]
fn unknown_or_broken_files_have_no_duration() {
    assert_eq!(duration_of("video/mp4", b""), None);
    assert_eq!(duration_of("audio/flac", b"fLaC"), None);
    assert_eq!(duration_of("application/pdf", b"%PDF-1.7"), None);
    assert_eq!(duration_of("audio/mpeg", &[0u8; 64]), None);
}
