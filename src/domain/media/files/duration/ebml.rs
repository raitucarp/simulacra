//! Matroska and WebM: the segment's `Info` element holds the timecode scale (nanoseconds per
//! tick) and the duration in ticks. It sits near the start, before the clusters.

use super::bytes::ByteSource;
use std::ops::Range;

const SEGMENT: u64 = 0x1853_8067;
const INFO: u64 = 0x1549_A966;
const TIMECODE_SCALE: u64 = 0x2A_D7B1;
const DURATION: u64 = 0x4489;
/// Ticks are milliseconds unless the file says otherwise.
const DEFAULT_SCALE_NANOS: u64 = 1_000_000;
const NANOS_PER_SECOND: f64 = 1e9;
/// Bytes read from the start to find `Info`.
const HEAD_BYTES: usize = 256 * 1024;
/// Longest variable-length integer.
const MAX_VINT_BYTES: u32 = 8;

/// Seconds of the segment.
pub(super) fn duration(source: &mut dyn ByteSource) -> Option<f64> {
    let head = source.read_at(0, HEAD_BYTES)?;
    let segment = element(&head, 0..head.len(), SEGMENT)?;
    let info = element(&head, segment, INFO)?;
    let scale = element(&head, info.clone(), TIMECODE_SCALE).map_or(DEFAULT_SCALE_NANOS, |range| unsigned(&head[range]));
    let ticks = float(&head[element(&head, info, DURATION)?])?;
    Some(ticks * scale as f64 / NANOS_PER_SECOND)
}

/// The data range of the first element `id` among the elements laid out in `range`.
fn element(bytes: &[u8], range: Range<usize>, id: u64) -> Option<Range<usize>> {
    let mut at = range.start;
    while at < range.end {
        let (found, id_len) = vint(bytes, at, false)?;
        let (size, size_len) = vint(bytes, at + id_len, true)?;
        let data = at + id_len + size_len;
        let unknown = size == (1u64 << (7 * size_len as u32)) - 1;
        let end = if unknown { range.end } else { data.saturating_add(usize::try_from(size).ok()?).min(range.end) };
        if found == id {
            return Some(data..end);
        }
        if unknown {
            return None;
        }
        at = end;
    }
    None
}

/// The variable-length integer at `at` and its length; `masked` drops the length marker (sizes).
fn vint(bytes: &[u8], at: usize, masked: bool) -> Option<(u64, usize)> {
    let first = *bytes.get(at)?;
    let len = first.leading_zeros() + 1;
    if len > MAX_VINT_BYTES {
        return None;
    }
    let lead = if masked { u64::from(first) & (0xFF >> len) } else { u64::from(first) };
    let rest = bytes.get(at + 1..at + len as usize)?;
    Some((rest.iter().fold(lead, |value, byte| value << 8 | u64::from(*byte)), len as usize))
}

fn unsigned(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0, |value, byte| value << 8 | u64::from(*byte))
}

fn float(bytes: &[u8]) -> Option<f64> {
    match bytes.len() {
        4 => Some(f64::from(f32::from_be_bytes(bytes.try_into().ok()?))),
        8 => Some(f64::from_be_bytes(bytes.try_into().ok()?)),
        _ => None,
    }
}
