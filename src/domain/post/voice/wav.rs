//! N11: WAV files of recorded voice notes: 16-bit PCM samples written with the canonical
//! 44-byte header, and the length of a WAV file read back from its header.

/// The rate voice notes are recorded at (speech needs no more, and the files stay small).
pub const RECORD_SAMPLE_RATE: u32 = 16_000;
/// Recorded voice notes are mono.
pub const RECORD_CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;
const BYTES_PER_SAMPLE: u32 = 2;
const PCM_FORMAT: u16 = 1;
const FMT_CHUNK_BYTES: u32 = 16;
/// The header bytes counted by the RIFF size besides the data (`WAVE`, the `fmt ` chunk and the `data` chunk head).
const RIFF_OVERHEAD: u32 = 36;
const CHUNK_HEAD: usize = 8;
const RIFF_HEAD: usize = 12;
const MS_PER_SECOND: u64 = 1000;

/// `samples` (interleaved for `channels`) at `rate`, as a WAV file.
pub fn wav_bytes(samples: &[i16], rate: u32, channels: u16) -> Vec<u8> {
    let data_len = samples.len() as u32 * BYTES_PER_SAMPLE;
    let block_align = channels * BITS_PER_SAMPLE / 8;
    let mut out = Vec::with_capacity(RIFF_OVERHEAD as usize + CHUNK_HEAD + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(RIFF_OVERHEAD + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&FMT_CHUNK_BYTES.to_le_bytes());
    out.extend_from_slice(&PCM_FORMAT.to_le_bytes());
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * u32::from(block_align)).to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for sample in samples {
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

fn u32_at(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// How long the WAV file `bytes` plays, from its `fmt ` and `data` chunks (`None`: not a WAV
/// file this can read).
pub fn wav_duration_ms(bytes: &[u8]) -> Option<u64> {
    if bytes.get(0..4)? != b"RIFF" || bytes.get(8..12)? != b"WAVE" {
        return None;
    }
    let (mut at, mut byte_rate, mut data) = (RIFF_HEAD, None, None);
    while at + CHUNK_HEAD <= bytes.len() {
        let size = u32_at(bytes, at + 4)?;
        match &bytes[at..at + 4] {
            b"fmt " => byte_rate = u32_at(bytes, at + CHUNK_HEAD + 8),
            b"data" => data = Some(size),
            _ => {}
        }
        at += CHUNK_HEAD + size as usize + (size as usize % 2);
    }
    let (rate, data) = (u64::from(byte_rate.filter(|rate| *rate > 0)?), u64::from(data?));
    Some(data * MS_PER_SECOND / rate)
}
