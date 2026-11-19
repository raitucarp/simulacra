//! PalmDOC (LZ77-style) decompression and the trailing entries MOBI appends to text records.

/// Back-reference distance bits and length bits of a two-byte PalmDOC pair.
const DISTANCE_SHIFT: u16 = 3;
const DISTANCE_MASK: u16 = 0x7FF;
const LENGTH_MASK: u16 = 0x7;
const MIN_COPY: usize = 3;
/// Varint bits per byte of a trailing-entry size, and the most bits read.
const VARINT_BITS: u32 = 7;
const VARINT_MAX_BITS: u32 = 28;
const MULTIBYTE_FLAG: u16 = 1;
const MULTIBYTE_MASK: u8 = 0x3;

/// Decompresses one PalmDOC record. Malformed back references are skipped, never panic.
pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() * 2);
    let mut index = 0;
    while index < input.len() {
        let byte = input[index];
        index += 1;
        match byte {
            0x01..=0x08 => {
                let end = (index + usize::from(byte)).min(input.len());
                out.extend_from_slice(&input[index..end]);
                index = end;
            }
            0x00 | 0x09..=0x7F => out.push(byte),
            0x80..=0xBF => {
                let Some(&next) = input.get(index) else { break };
                index += 1;
                let pair = u16::from_be_bytes([byte, next]);
                let distance = usize::from((pair >> DISTANCE_SHIFT) & DISTANCE_MASK);
                let length = usize::from(pair & LENGTH_MASK) + MIN_COPY;
                copy_back(&mut out, distance, length);
            }
            0xC0..=0xFF => {
                out.push(b' ');
                out.push(byte ^ 0x80);
            }
        }
    }
    out
}

/// Appends `length` bytes copied from `distance` back (overlapping copies repeat bytes).
fn copy_back(out: &mut Vec<u8>, distance: usize, length: usize) {
    if distance == 0 || distance > out.len() {
        return;
    }
    let start = out.len() - distance;
    for offset in 0..length {
        let byte = out[start + offset];
        out.push(byte);
    }
}

/// Size of the backward-encoded varint ending at `end` in `record`.
fn trailing_entry_size(record: &[u8], end: usize) -> usize {
    let (mut result, mut bits, mut at) = (0usize, 0u32, end);
    while at > 0 {
        let byte = record[at - 1];
        result |= usize::from(byte & 0x7F) << bits;
        bits += VARINT_BITS;
        at -= 1;
        if byte & 0x80 != 0 || bits >= VARINT_MAX_BITS {
            break;
        }
    }
    result
}

/// `record` without the trailing entries announced by the header's extra-data flags.
pub fn strip_trailing_entries(record: &[u8], flags: u16) -> &[u8] {
    let mut size = record.len();
    let mut entries = flags >> 1;
    while entries != 0 {
        if entries & 1 != 0 {
            size = size.saturating_sub(trailing_entry_size(record, size));
        }
        entries >>= 1;
    }
    if flags & MULTIBYTE_FLAG != 0 && size > 0 {
        size = size.saturating_sub(usize::from(record[size - 1] & MULTIBYTE_MASK) + 1);
    }
    &record[..size]
}
