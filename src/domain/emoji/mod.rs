//! Emoji data for the picker (R3): names, groups and skin tones of every fully-qualified
//! emoji, generated from Unicode's `emoji-test.txt` into [`table`].

pub mod generate;
mod groups;
mod table;
#[cfg(test)]
mod tests;

pub use groups::{EmojiGroup, SkinTone};
pub use table::{EMOJI, UNICODE_EMOJI_VERSION};

/// The Unicode emoji version an emoji arrived in (`E13.0` is `{ major: 13, minor: 0 }`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmojiVersion {
    pub major: u8,
    pub minor: u8,
}

/// One emoji: its text, Unicode name, group, version and skin-tone variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Emoji {
    pub glyph: &'static str,
    pub name: &'static str,
    pub group: EmojiGroup,
    pub since: EmojiVersion,
    /// The five single skin-tone variants in [`SkinTone::ALL`] order, or none.
    pub tones: &'static [&'static str],
}

impl Emoji {
    /// The emoji in `tone` (itself when it has no skin tones or `tone` is `None`).
    pub fn in_tone(&self, tone: Option<SkinTone>) -> &'static str {
        tone.and_then(|tone| self.tones.get(tone.index()).copied()).unwrap_or(self.glyph)
    }

    /// Whether the emoji comes in skin tones.
    pub fn has_tones(&self) -> bool {
        !self.tones.is_empty()
    }
}

/// Builds a table entry (kept short: the generated table has one call per emoji).
const fn e(glyph: &'static str, name: &'static str, group: EmojiGroup, major: u8, minor: u8, tones: &'static [&'static str]) -> Emoji {
    Emoji { glyph, name, group, since: EmojiVersion { major, minor }, tones }
}

/// The table entry whose glyph, or one of whose skin-tone variants, is `text`.
pub fn find_emoji(text: &str) -> Option<&'static Emoji> {
    EMOJI.iter().find(|emoji| emoji.glyph == text || emoji.tones.contains(&text))
}
