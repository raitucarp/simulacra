//! MP3 (MPEG audio layer III): past the ID3 tag, the first frame's header gives the version,
//! bit rate and sample rate; a Xing / Info or VBRI header gives the frame count of a
//! variable bit rate file, else the length follows from the constant bit rate.

use super::bytes::{be_u32, ByteSource};

const ID3: &[u8] = b"ID3";
const ID3_HEADER: usize = 10;
const ID3_FOOTER_FLAG: u8 = 0x10;
const XING: [&[u8]; 2] = [b"Xing", b"Info"];
const VBRI: &[u8] = b"VBRI";
/// VBRI sits a fixed 32 bytes after the frame header; its frame count 14 bytes in.
const VBRI_AT: usize = 4 + 32;
const VBRI_FRAMES_AT: usize = 14;
const XING_FRAMES_FLAG: u32 = 1;
const FRAME_HEADER: usize = 4;
/// Kilobits per second by bit rate index (layer III), for MPEG-1 and for MPEG-2 / 2.5.
const MPEG1_KBPS: [u32; 15] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320];
const MPEG2_KBPS: [u32; 15] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160];
/// Sample rates by index, for MPEG-1, MPEG-2 and MPEG-2.5.
const RATES: [[u32; 3]; 3] = [[44_100, 48_000, 32_000], [22_050, 24_000, 16_000], [11_025, 12_000, 8_000]];
const LAYER_III: u8 = 1;
const MONO: u8 = 3;
/// Bytes searched for the first frame after the tag.
const SYNC_SEARCH: usize = 4096;
const HEAD_BYTES: usize = 128 * 1024;
const BITS_PER_BYTE: f64 = 8.0;
const BITS_PER_KILOBIT: f64 = 1000.0;

/// What the first frame header says.
struct Frame {
    mpeg1: bool,
    mono: bool,
    kbps: u32,
    rate: u32,
}

/// Seconds of an MP3 file.
pub(super) fn duration(source: &mut dyn ByteSource) -> Option<f64> {
    let head = source.read_at(0, HEAD_BYTES)?;
    let start = tag_length(&head);
    let at = (start..start + SYNC_SEARCH).find(|at| frame(&head, *at).is_some())?;
    let frame = frame(&head, at)?;
    let samples_per_frame = if frame.mpeg1 { 1152.0 } else { 576.0 };
    match frame_count(&head, at, &frame) {
        Some(frames) => Some(f64::from(frames) * samples_per_frame / f64::from(frame.rate)),
        None => Some(source.len().saturating_sub(at as u64) as f64 * BITS_PER_BYTE / (f64::from(frame.kbps) * BITS_PER_KILOBIT)),
    }
}

/// Bytes of the ID3v2 tag at the start (its size is written 7 bits per byte).
fn tag_length(head: &[u8]) -> usize {
    if !head.starts_with(ID3) || head.len() < ID3_HEADER {
        return 0;
    }
    let size = head[6..10].iter().fold(0usize, |size, byte| size << 7 | usize::from(byte & 0x7F));
    let footer = if head[5] & ID3_FOOTER_FLAG != 0 { ID3_HEADER } else { 0 };
    ID3_HEADER + size + footer
}

/// The layer III frame header at `at`, if one is there.
fn frame(head: &[u8], at: usize) -> Option<Frame> {
    let bytes = head.get(at..at + FRAME_HEADER)?;
    if bytes[0] != 0xFF || bytes[1] & 0xE0 != 0xE0 || (bytes[1] >> 1) & 3 != LAYER_III {
        return None;
    }
    let version = match (bytes[1] >> 3) & 3 {
        3 => 0,
        2 => 1,
        0 => 2,
        _ => return None,
    };
    let kbps = if version == 0 { MPEG1_KBPS } else { MPEG2_KBPS }.get(usize::from(bytes[2] >> 4)).copied().filter(|kbps| *kbps > 0)?;
    let rate = *RATES[version].get(usize::from((bytes[2] >> 2) & 3))?;
    Some(Frame { mpeg1: version == 0, mono: bytes[3] >> 6 == MONO, kbps, rate })
}

/// The frame count a Xing / Info or VBRI header gives.
fn frame_count(head: &[u8], at: usize, frame: &Frame) -> Option<u32> {
    let side_info = match (frame.mpeg1, frame.mono) {
        (true, false) => 32,
        (true, true) | (false, false) => 17,
        (false, true) => 9,
    };
    let xing = at + FRAME_HEADER + side_info;
    if XING.iter().any(|tag| head.get(xing..xing + 4) == Some(*tag)) {
        let flags = be_u32(head, xing + 4)?;
        return (flags & XING_FRAMES_FLAG != 0).then(|| be_u32(head, xing + 8)).flatten();
    }
    let vbri = at + VBRI_AT;
    (head.get(vbri..vbri + 4) == Some(VBRI)).then(|| be_u32(head, vbri + VBRI_FRAMES_AT)).flatten()
}
