//! Files without readable text: unknown binaries (printable strings), images (pixel size),
//! archives (their entry list), and the SHA-256 checksum every file gets.

mod archive;
mod image;

#[cfg(test)]
mod tests;

pub use archive::parse_archive;
pub use image::parse_image;
#[cfg(test)]
use image::image_dimensions;

use super::Content;
use crate::import::document::{Block, BlockKind, ParseNote, Section};
use sha2::{Digest, Sha256};

/// Shortest run of printable characters kept as a string.
pub const MIN_STRING_CHARS: usize = 8;
/// Most strings kept from one file.
pub const MAX_STRINGS: usize = 2000;
/// Strings grouped into one block.
const STRINGS_PER_BLOCK: usize = 8;
/// A kept string holds a run of at least this many lower-case letters (a word), which noise
/// that merely happens to be printable rarely does.
const MIN_WORD_LETTERS: usize = 4;

/// Lower-case hex SHA-256 of `bytes`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

fn is_printable(byte: u8) -> bool {
    byte == b' ' || byte.is_ascii_graphic()
}

/// Whether `run` contains a word of [`MIN_WORD_LETTERS`] lower-case letters.
fn has_word(run: &str) -> bool {
    run.split(|c: char| !c.is_ascii_lowercase()).any(|word| word.len() >= MIN_WORD_LETTERS)
}

/// Runs of at least `min_chars` printable ASCII characters that contain a word, at most
/// `limit` of them.
pub fn printable_strings(bytes: &[u8], min_chars: usize, limit: usize) -> Vec<String> {
    bytes
        .split(|&byte| !is_printable(byte))
        .filter(|run| run.len() >= min_chars)
        .map(|run| String::from_utf8_lossy(run).trim().to_string())
        .filter(|run| run.chars().count() >= min_chars && has_word(run))
        .take(limit)
        .collect()
}

/// An unrecognised binary: its printable strings, grouped into preformatted blocks.
pub fn parse_unknown(bytes: &[u8]) -> Content {
    let strings = printable_strings(bytes, MIN_STRING_CHARS, MAX_STRINGS);
    let blocks: Vec<Block> = strings
        .chunks(STRINGS_PER_BLOCK)
        .map(|chunk| Block::new(BlockKind::Preformatted, chunk.join("\n")))
        .collect();
    let notes = vec![ParseNote::ExtractedStrings(strings.len())];
    let sections = if blocks.is_empty() { Vec::new() } else { vec![Section::new(None, blocks)] };
    Content { sections, notes, ..Content::default() }
}
