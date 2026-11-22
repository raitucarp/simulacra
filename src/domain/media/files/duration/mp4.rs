//! ISO base media (MP4, M4A, QuickTime): the movie header box (`moov` › `mvhd`) holds the
//! time scale and the duration. `moov` may sit after the media data, so boxes are walked by
//! offset rather than read from the head.

use super::bytes::{be_u32, be_u64, ByteSource};

/// Bytes of a box header (size and type), and of its 64-bit size when the size is 1.
const BOX_HEADER: u64 = 8;
const LARGE_SIZE: u64 = 8;
/// Top-level boxes walked at most (a well-formed file has a handful).
const MAX_BOXES: usize = 64;
/// Largest `moov` read whole.
const MAX_MOOV_BYTES: u64 = 16 * 1024 * 1024;

/// Seconds of the movie.
pub(super) fn duration(source: &mut dyn ByteSource) -> Option<f64> {
    let end = source.len();
    let (start, size) = find_box(source, 0, end, b"moov")?;
    let moov = source.read_at(start, usize::try_from(size.min(MAX_MOOV_BYTES)).ok()?)?;
    let mvhd = child(&moov, b"mvhd")?;
    let (timescale, length) = match mvhd.first()? {
        1 => (be_u32(mvhd, 20)?, be_u64(mvhd, 24)?),
        _ => (be_u32(mvhd, 12)?, u64::from(be_u32(mvhd, 16)?)),
    };
    (timescale > 0).then(|| length as f64 / f64::from(timescale))
}

/// The body offset and size of the first `kind` box between `from` and `end`.
fn find_box(source: &mut dyn ByteSource, mut at: u64, end: u64, kind: &[u8; 4]) -> Option<(u64, u64)> {
    for _ in 0..MAX_BOXES {
        let header = source.read_at(at, (BOX_HEADER + LARGE_SIZE) as usize)?;
        let (size, header_len) = match u64::from(be_u32(&header, 0)?) {
            1 => (be_u64(&header, 8)?, BOX_HEADER + LARGE_SIZE),
            0 => (end.checked_sub(at)?, BOX_HEADER),
            size => (size, BOX_HEADER),
        };
        if size < header_len {
            return None;
        }
        if header.get(4..8)? == kind {
            return Some((at + header_len, size - header_len));
        }
        at = at.checked_add(size).filter(|next| *next < end)?;
    }
    None
}

/// The body of the first `kind` box directly inside `body`.
fn child<'a>(body: &'a [u8], kind: &[u8; 4]) -> Option<&'a [u8]> {
    let mut at = 0usize;
    while at + BOX_HEADER as usize <= body.len() {
        let size = be_u32(body, at)? as usize;
        if size < BOX_HEADER as usize {
            return None;
        }
        if body.get(at + 4..at + 8)? == kind {
            return body.get(at + 8..(at + size).min(body.len()));
        }
        at = at.checked_add(size)?;
    }
    None
}
