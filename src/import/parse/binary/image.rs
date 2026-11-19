//! Image metadata only: the pixel size read from the header of PNG, GIF, BMP, JPEG and WebP.

use crate::import::detect::Format;
use crate::import::document::ParseNote;
use crate::import::parse::Content;

/// JPEG start-of-frame markers (baseline, progressive, ...), which carry the size.
const JPEG_FRAME_MARKERS: [u8; 13] = [0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC6, 0xC7, 0xC9, 0xCA, 0xCB, 0xCD, 0xCE, 0xCF];
const JPEG_FIRST_SEGMENT: usize = 2;
const WEBP_CHUNK: usize = 12;
const WEBP_14_BIT_MASK: u32 = 0x3FFF;
const WEBP_24_BIT_MASK: u32 = 0xFF_FFFF;

fn be16(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at + 2).map(|b| u32::from(u16::from_be_bytes([b[0], b[1]])))
}

fn le16(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at + 2).map(|b| u32::from(u16::from_le_bytes([b[0], b[1]])))
}

fn be32(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

fn le32(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    let mut at = JPEG_FIRST_SEGMENT;
    while at + 4 <= bytes.len() {
        if bytes[at] != 0xFF {
            return None;
        }
        let marker = bytes[at + 1];
        if JPEG_FRAME_MARKERS.contains(&marker) {
            return Some((be16(bytes, at + 7)?, be16(bytes, at + 5)?));
        }
        at += 2 + be16(bytes, at + 2)? as usize;
    }
    None
}

fn webp_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    let chunk = bytes.get(WEBP_CHUNK..WEBP_CHUNK + 4)?;
    match chunk {
        b"VP8 " => Some((le16(bytes, 26)? & WEBP_14_BIT_MASK, le16(bytes, 28)? & WEBP_14_BIT_MASK)),
        b"VP8L" => {
            let bits = le32(bytes, 21)?;
            Some(((bits & WEBP_14_BIT_MASK) + 1, ((bits >> 14) & WEBP_14_BIT_MASK) + 1))
        }
        b"VP8X" => Some(((le32(bytes, 24)? & WEBP_24_BIT_MASK) + 1, (le32(bytes, 27)? & WEBP_24_BIT_MASK) + 1)),
        _ => None,
    }
}

/// `(width, height)` of an image, when its header can be read.
pub fn image_dimensions(format: Format, bytes: &[u8]) -> Option<(u32, u32)> {
    match format {
        Format::Png => Some((be32(bytes, 16)?, be32(bytes, 20)?)),
        Format::Gif => Some((le16(bytes, 6)?, le16(bytes, 8)?)),
        Format::Bmp => Some((le32(bytes, 18)?, le32(bytes, 22)?.cast_signed().unsigned_abs())),
        Format::Jpeg => jpeg_dimensions(bytes),
        Format::Webp => webp_dimensions(bytes),
        _ => None,
    }
}

/// An image: no text, its size as a note.
pub fn parse_image(format: Format, bytes: &[u8]) -> Content {
    let notes = image_dimensions(format, bytes)
        .map(|(width, height)| ParseNote::Dimensions { width, height })
        .into_iter()
        .collect();
    Content { notes, ..Content::default() }
}
