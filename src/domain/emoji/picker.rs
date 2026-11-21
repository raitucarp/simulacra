//! What the emoji picker offers (R5): emoji up to the version every platform's font draws
//! (decision R4), searched by name, by group, with the recently used ones and a skin tone.

use super::{Emoji, EmojiGroup, EmojiVersion, SkinTone, EMOJI};
use serde::{Deserialize, Serialize};

/// The newest emoji version the picker offers (decision R4: what Windows 11, macOS and Noto
/// Color Emoji all draw).
pub const PICKER_MAX_EMOJI_VERSION: EmojiVersion = EmojiVersion { major: 15, minor: 0 };
/// Recently used emoji kept, newest first.
pub const RECENT_LIMIT: usize = 24;
/// Search results shown at most.
pub const SEARCH_LIMIT: usize = 120;

/// Every emoji the picker offers, in table order.
pub fn pickable() -> impl Iterator<Item = &'static Emoji> {
    EMOJI.iter().filter(|emoji| emoji.since <= PICKER_MAX_EMOJI_VERSION)
}

/// The offered emoji of `group`.
pub fn group_emoji(group: EmojiGroup) -> Vec<&'static Emoji> {
    pickable().filter(|emoji| emoji.group == group).collect()
}

/// Whether the picker offers `text` (a glyph or one of its skin tones).
pub fn is_pickable(text: &str) -> bool {
    pickable().any(|emoji| emoji.glyph == text || emoji.tones.contains(&text))
}

/// Offered emoji whose name has a word starting with every term of `query` (case does not
/// matter), best first: the name itself, then names whose first word is the query, then
/// names starting with it, then the rest, each in table order. An empty query finds nothing.
pub fn search_emoji(query: &str) -> Vec<&'static Emoji> {
    let query = query.trim().to_lowercase();
    let terms: Vec<&str> = query.split_whitespace().collect();
    if terms.is_empty() {
        return Vec::new();
    }
    let mut found: Vec<&Emoji> = pickable().filter(|emoji| matches_terms(emoji.name, &terms)).collect();
    found.sort_by_key(|emoji| rank(&emoji.name.to_lowercase(), &query));
    found.truncate(SEARCH_LIMIT);
    found
}

/// How well `name` matches `query` (lower is better).
fn rank(name: &str, query: &str) -> u8 {
    match name.strip_prefix(query) {
        Some("") => 0,
        Some(rest) if rest.starts_with(|c: char| !c.is_alphanumeric()) => 1,
        Some(_) => 2,
        None => 3,
    }
}

fn matches_terms(name: &str, terms: &[&str]) -> bool {
    let name = name.to_lowercase();
    let words: Vec<&str> = name.split(|c: char| !c.is_alphanumeric()).filter(|word| !word.is_empty()).collect();
    terms.iter().all(|term| words.iter().any(|word| word.starts_with(term)))
}

/// The picker's memory: recently used emoji (newest first) and the chosen skin tone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PickerMemory {
    pub recent: Vec<String>,
    /// The tone applied to emoji that have tones (`None`: the default yellow).
    pub tone: Option<SkinTone>,
}

impl PickerMemory {
    /// Remembers `emoji` as the most recent, keeping at most [`RECENT_LIMIT`].
    pub fn used(&mut self, emoji: &str) {
        self.recent.retain(|recent| recent != emoji);
        self.recent.insert(0, emoji.to_string());
        self.recent.truncate(RECENT_LIMIT);
    }

    /// The memory as read back: unknown or repeated entries dropped, at most [`RECENT_LIMIT`].
    pub fn validated(mut self) -> Self {
        let mut seen = std::collections::HashSet::new();
        self.recent.retain(|emoji| super::find_emoji(emoji).is_some() && seen.insert(emoji.clone()));
        self.recent.truncate(RECENT_LIMIT);
        self
    }

    /// `emoji` in the chosen tone.
    pub fn toned(&self, emoji: &Emoji) -> &'static str {
        emoji.in_tone(self.tone)
    }
}
