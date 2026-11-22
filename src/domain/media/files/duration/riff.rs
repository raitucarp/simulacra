//! RIFF containers: a WAV's `fmt ` chunk gives the byte rate and its `data` chunk the length;
//! an AVI's main header (`avih`) gives the frame time and the frame count.

use super::bytes::{find, le_u32, ByteSource};

const RIFF: &[u8] = b"RIFF";
const WAVE: &[u8] = b"WAVE";
const AVI: &[u8] = b"AVI ";
const FMT: &[u8] = b"fmt ";
const DATA: &[u8] = b"data";
const AVIH: &[u8] = b"avih";
/// Offset of the first chunk after `RIFF <size> <form>`, and the size of a chunk header.
const FIRST_CHUNK: usize = 12;
const CHUNK_HEADER: usize = 8;
/// Offsets inside `fmt ` (byte rate) and `avih` (frame time, frame count).
const BYTE_RATE_AT: usize = 8;
const FRAME_TIME_AT: usize = 0;
const FRAME_COUNT_AT: usize = 16;
/// `data` sizes a streaming writer leaves unset.
const UNSET_SIZES: [u32; 2] = [0, u32::MAX];
const MICROS_PER_SECOND: f64 = 1e6;
const HEAD_BYTES: usize = 64 * 1024;

/// Seconds of a WAV or AVI file.
pub(super) fn duration(source: &mut dyn ByteSource) -> Option<f64> {
    let head = source.read_at(0, HEAD_BYTES)?;
    if head.get(..4)? != RIFF {
        return None;
    }
    match head.get(8..12)? {
        form if form == WAVE => wav(&head, source.len()),
        form if form == AVI => avi(&head),
        _ => None,
    }
}

fn wav(head: &[u8], file_len: u64) -> Option<f64> {
    let (fmt, _) = chunk(head, FMT)?;
    let byte_rate = le_u32(head, fmt + BYTE_RATE_AT).filter(|rate| *rate > 0)?;
    let (data, size) = chunk(head, DATA)?;
    let size = if UNSET_SIZES.contains(&size) { file_len.saturating_sub(data as u64) } else { u64::from(size) };
    Some(size as f64 / f64::from(byte_rate))
}

fn avi(head: &[u8]) -> Option<f64> {
    let header = find(head, AVIH, FIRST_CHUNK)? + CHUNK_HEADER;
    let frame_micros = le_u32(head, header + FRAME_TIME_AT)?;
    let frames = le_u32(head, header + FRAME_COUNT_AT)?;
    Some(f64::from(frame_micros) * f64::from(frames) / MICROS_PER_SECOND)
}

/// The data offset and size of the first top-level chunk `id`.
fn chunk(head: &[u8], id: &[u8]) -> Option<(usize, u32)> {
    let mut at = FIRST_CHUNK;
    while at + CHUNK_HEADER <= head.len() {
        let size = le_u32(head, at + 4)?;
        if head.get(at..at + 4)? == id {
            return Some((at + CHUNK_HEADER, size));
        }
        let padded = usize::try_from(size).ok()?.checked_add(size as usize & 1)?;
        at = at.checked_add(CHUNK_HEADER + padded)?;
    }
    None
}
