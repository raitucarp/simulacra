//! Reactions and claps in the store (epic R): every change is made as the acting persona on
//! the post's original (an instant repost's target), saved as it happens, and bumps the
//! engagement revision recommendations follow.

use super::AppState;
use crate::domain::engine::reactions_left;
use crate::domain::reaction::{normalize_emoji, PostReactions, Reaction};
use crate::domain::{PostId, ProfileId};
use jiff::Timestamp;

impl AppState {
    /// The reactions to `post` (`None` when it has none).
    pub fn reactions_of(&self, post: &PostId) -> Option<&PostReactions> {
        self.reactions.of(post)
    }

    /// The acting persona reacts to `post` with `emoji`, or takes the reaction back when it
    /// already gave it. Returns the saved row (`None` when nothing changed: unknown post or
    /// text that is not an emoji).
    pub fn toggle_reaction(&mut self, post: &PostId, emoji: &str) -> Option<Reaction> {
        let (post, emoji) = (self.reaction_target(post)?, normalize_emoji(emoji)?);
        let profile = self.active_profile_id.clone();
        let changed = self.reactions.toggle(&post, &profile, &emoji, Timestamp::now());
        self.saved_reaction(changed)
    }

    /// `profile` reacts to `post` with `emoji` (adding it again changes nothing), as agents do.
    pub fn add_reaction_as(&mut self, post: &PostId, profile: &ProfileId, emoji: &str, at: Timestamp) -> Option<Reaction> {
        let (post, emoji) = (self.reaction_target(post)?, normalize_emoji(emoji)?);
        let changed = self.reactions.add(&post, profile, &emoji, at);
        self.saved_reaction(changed)
    }

    /// The agent `agent` reacts to `post` with each of `emoji` (R8), as far as its hourly
    /// limit ([`crate::domain::engine::AGENT_REACTIONS_PER_HOUR`]) allows; an emoji it
    /// already gave is not counted again. Returns how many reactions were added.
    pub fn react_as_agent(&mut self, post: &PostId, agent: &ProfileId, emoji: &[String], now: Timestamp) -> usize {
        let mut added = 0;
        for emoji in emoji {
            if reactions_left(agent, self.reactions.given_by(agent), now) == 0 {
                break;
            }
            added += usize::from(self.add_reaction_as(post, agent, emoji, now).is_some());
        }
        added
    }

    /// The acting persona claps `times` more on `post`.
    pub fn clap(&mut self, post: &PostId, times: u32) -> Option<Reaction> {
        let post = self.reaction_target(post)?;
        let profile = self.active_profile_id.clone();
        let changed = self.reactions.clap(&post, &profile, times, Timestamp::now());
        self.saved_reaction(changed)
    }

    /// The acting persona takes back `times` of its claps on `post`.
    pub fn unclap(&mut self, post: &PostId, times: u32) -> Option<Reaction> {
        let post = self.reaction_target(post)?;
        let profile = self.active_profile_id.clone();
        let changed = self.reactions.unclap(&post, &profile, times, Timestamp::now());
        self.saved_reaction(changed)
    }

    /// The post a reaction lands on: the original of an instant repost, when it exists.
    fn reaction_target(&self, post: &PostId) -> Option<PostId> {
        let target = self.action_target(post);
        self.get_post(&target).is_some().then_some(target)
    }

    /// Saves a changed row and bumps the engagement revision.
    fn saved_reaction(&mut self, changed: Option<Reaction>) -> Option<Reaction> {
        let changed = changed?;
        self.engagement += 1;
        if let Some(ref db) = self.db {
            let _ = db.save_reaction(&changed);
        }
        Some(changed)
    }
}
