//! The emoji table generator (R3): reads Unicode's `emoji-test.txt` and writes `table.rs`.
//!
//! Run by the `emoji-table` tool (`cargo run --features codegen --bin emoji-table -- <path to
//! emoji-test.txt>`); nothing here touches the network. Only fully-qualified emoji are kept,
//! the Component group is left out, and every emoji with all five single skin tones lists
//! them (sequences mixing tones are left out: the picker applies one tone to everything).

mod parse;
mod render;
#[cfg(test)]
mod tests;

pub use parse::{parse_emoji_test, ParseError};
pub use render::render_table;

/// One emoji as the generator collects it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiRecord {
    pub glyph: String,
    pub name: String,
    /// Index into [`super::EmojiGroup::ALL`].
    pub group: usize,
    pub subgroup: String,
    pub since: (u8, u8),
    /// The five skin-tone variants, lightest first (empty when the emoji has none).
    pub tones: Vec<String>,
}

/// What the generator read: the Unicode emoji version and every emoji, in CLDR order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiData {
    pub version: String,
    pub emoji: Vec<EmojiRecord>,
}
