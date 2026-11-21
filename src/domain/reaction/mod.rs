//! Emoji reactions and claps on posts (epic R).
//!
//! Likes stay as they are (decision #20); reactions are kept beside them. One row per post,
//! profile and emoji (`post_reactions`): an emoji reaction counts once, while claps (the
//! [`CLAP`] emoji) count every time a profile claps.

mod book;
mod emoji_text;
pub mod gesture; // R7: the clap button's click, Alt-click and hold
mod post_reactions;
mod totals;
#[cfg(test)]
mod tests;

pub use book::ReactionBook;
pub use emoji_text::{normalize_emoji, MAX_EMOJI_CHARS};
pub use post_reactions::PostReactions;
pub use totals::ReactionChip;

use super::ids::{PostId, ProfileId};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// The emoji a clap is stored as: one row per profile, its count the number of claps.
pub const CLAP: &str = "\u{1F44F}";

/// One profile's reaction with one emoji to one post (`post_reactions` row).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reaction {
    pub post_id: PostId,
    pub profile_id: ProfileId,
    pub emoji: String,
    /// Times the profile reacted with this emoji: 1 for an emoji, the claps for [`CLAP`].
    pub count: u32,
    pub updated_at: Timestamp,
}

impl Reaction {
    /// Whether this row holds claps rather than an emoji reaction.
    pub fn is_clap(&self) -> bool {
        self.emoji == CLAP
    }
}
