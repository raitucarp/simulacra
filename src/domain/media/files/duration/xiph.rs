//! FLAC (the stream info block: sample rate and total samples) and Ogg Vorbis / Opus (the
//! identification header's rate, the last page's granule position).

use super::bytes::{le_u32, le_u64, ByteSource};

const FLAC: &[u8] = b"fLaC";
/// Offset of the first metadata block's body, and of the rate inside stream info.
const STREAM_INFO: usize = 8;
const RATE_AT: usize = 10;
const BLOCK_TYPE_MASK: u8 = 0x7F;
const OGG_PAGE: &[u8] = b"OggS";
/// Page header bytes before the segment table, and where the granule and segment count sit.
const PAGE_HEADER: usize = 27;
const GRANULE_AT: usize = 6;
const SEGMENTS_AT: usize = 26;
const VORBIS_ID: &[u8] = b"\x01vorbis";
const VORBIS_RATE_AT: usize = 12;
const OPUS_ID: &[u8] = b"OpusHead";
const OPUS_PRE_SKIP_AT: usize = 10;
/// Opus granules always count 48 kHz samples.
const OPUS_RATE: u32 = 48_000;
const HEAD_BYTES: usize = 4096;
/// Bytes read from the end to find the last page (a page is at most about 64 KB).
const TAIL_BYTES: u64 = 128 * 1024;

/// Seconds of a FLAC stream.
pub(super) fn flac_duration(source: &mut dyn ByteSource) -> Option<f64> {
    let head = source.read_at(0, HEAD_BYTES)?;
    if head.get(..4)? != FLAC || head.get(4)? & BLOCK_TYPE_MASK != 0 {
        return None;
    }
    let info = head.get(STREAM_INFO + RATE_AT..STREAM_INFO + RATE_AT + 8)?;
    let rate = u32::from(info[0]) << 12 | u32::from(info[1]) << 4 | u32::from(info[2]) >> 4;
    let samples = u64::from(info[3] & 0x0F) << 32 | u64::from(u32::from_be_bytes(info[4..8].try_into().ok()?));
    (rate > 0 && samples > 0).then(|| samples as f64 / f64::from(rate))
}

/// Seconds of an Ogg Vorbis or Opus stream.
pub(super) fn ogg_duration(source: &mut dyn ByteSource) -> Option<f64> {
    let head = source.read_at(0, HEAD_BYTES)?;
    if head.get(..4)? != OGG_PAGE {
        return None;
    }
    let body = head.get(PAGE_HEADER + usize::from(*head.get(SEGMENTS_AT)?)..)?;
    let (rate, skip) = if body.starts_with(VORBIS_ID) {
        (le_u32(body, VORBIS_RATE_AT)?, 0)
    } else if body.starts_with(OPUS_ID) {
        (OPUS_RATE, u64::from(u16::from_le_bytes(body.get(OPUS_PRE_SKIP_AT..OPUS_PRE_SKIP_AT + 2)?.try_into().ok()?)))
    } else {
        return None;
    };
    let from = source.len().saturating_sub(TAIL_BYTES);
    let tail = source.read_at(from, TAIL_BYTES as usize)?;
    let last = tail.windows(OGG_PAGE.len()).rposition(|window| window == OGG_PAGE)?;
    let granule = le_u64(&tail, last + GRANULE_AT)?;
    (rate > 0).then(|| granule.saturating_sub(skip) as f64 / f64::from(rate))
}
