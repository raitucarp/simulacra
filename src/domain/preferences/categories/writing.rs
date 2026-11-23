//! Reading & Writing: post length, thread splitting, autosave, spellcheck, the default
//! identity and what wikilinks and hashtags do.

use super::super::bounds::Bounds;
use super::super::choices::{ArchiveAfter, HashtagClick, PostIdentity, WikilinkClick};
use crate::domain::post::MAX_POST_CHARS;
use serde::{Deserialize, Serialize};

/// Characters per post.
pub const POST_LENGTH: Bounds = Bounds::new(140, 1000, 20);
/// N4: posts the feed shows a day (`0`: no budget, the feed never ends).
pub const FEED_BUDGET: Bounds = Bounds::new(0, 500, 10);
/// Seconds between draft autosaves.
pub const AUTOSAVE_SECONDS: Bounds = Bounds::new(5, 300, 5);
const DEFAULT_AUTOSAVE_SECONDS: u32 = 30;

/// Reading and writing preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct WritingPrefs {
    /// Characters per post. Consumed by a later wave (posts are still capped at 280).
    pub post_length_limit: u32,
    /// Split an over-long draft into a thread instead of refusing it. Consumed by a later wave.
    pub auto_split_threads: bool,
    /// Seconds between draft autosaves. Consumed by a later wave.
    pub autosave_seconds: u32,
    /// Underline misspellings where the platform offers a spellchecker. Consumed by a later wave.
    pub spellcheck: bool,
    /// Who new posts go out as. Consumed by a later wave.
    pub default_identity: PostIdentity,
    /// Consumed by a later wave.
    pub wikilink_click: WikilinkClick,
    /// Consumed by a later wave.
    pub hashtag_click: HashtagClick,
    /// sb E1: a quiet atomic-note hint under composer drafts (one claim, with context).
    pub atomic_hints: bool,
    /// sb F3: a quiet readability meter under composer drafts.
    pub readability_meter: bool,
    /// Y6: how long after they go out the user's posts move to the archive.
    pub auto_archive: ArchiveAfter,
    /// S6: bullet-journal shorthand at the start of a post sets its symbols (off by default).
    pub symbol_shorthand: bool,
    /// M6: a persona's feeling also logs a mood check-in in the private journal (off by default).
    pub feeling_checkins: bool,
    /// N4: posts the feed shows a day before it ends with a closing card (`0`: no budget).
    pub feed_budget: u32,
}

impl Default for WritingPrefs {
    fn default() -> Self {
        Self {
            post_length_limit: MAX_POST_CHARS as u32,
            auto_split_threads: true,
            autosave_seconds: DEFAULT_AUTOSAVE_SECONDS,
            spellcheck: true,
            default_identity: PostIdentity::default(),
            wikilink_click: WikilinkClick::default(),
            hashtag_click: HashtagClick::default(),
            atomic_hints: true,
            readability_meter: true,
            auto_archive: ArchiveAfter::default(),
            symbol_shorthand: false,
            feeling_checkins: false,
            feed_budget: 0,
        }
    }
}

impl WritingPrefs {
    /// Clamps the numeric fields into range.
    pub fn validated(self) -> Self {
        Self {
            post_length_limit: POST_LENGTH.clamp_u32(self.post_length_limit),
            autosave_seconds: AUTOSAVE_SECONDS.clamp_u32(self.autosave_seconds),
            feed_budget: FEED_BUDGET.clamp_u32(self.feed_budget),
            ..self
        }
    }
}
