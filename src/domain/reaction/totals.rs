//! Totals over one post's reactions: one chip per emoji, the claps, and what each profile gave.

use super::{PostReactions, CLAP};
use crate::domain::ProfileId;

/// One emoji's chip on the reaction bar: its total and who reacted, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactionChip {
    pub emoji: String,
    pub total: u32,
    pub profiles: Vec<ProfileId>,
}

impl ReactionChip {
    /// Whether `profile` is among those who reacted with this emoji.
    pub fn includes(&self, profile: &ProfileId) -> bool {
        self.profiles.contains(profile)
    }
}

impl PostReactions {
    /// One chip per emoji other than claps, in the order each emoji was first given.
    pub fn chips(&self) -> Vec<ReactionChip> {
        let mut chips: Vec<ReactionChip> = Vec::new();
        for row in self.rows().iter().filter(|row| !row.is_clap()) {
            match chips.iter_mut().find(|chip| chip.emoji == row.emoji) {
                Some(chip) => {
                    chip.total = chip.total.saturating_add(row.count);
                    chip.profiles.push(row.profile_id.clone());
                }
                None => chips.push(ReactionChip { emoji: row.emoji.clone(), total: row.count, profiles: vec![row.profile_id.clone()] }),
            }
        }
        chips
    }

    /// Every clap the post received.
    pub fn claps(&self) -> u32 {
        self.rows().iter().filter(|row| row.is_clap()).fold(0, |sum, row| sum.saturating_add(row.count))
    }

    /// The claps `profile` gave the post.
    pub fn claps_by(&self, profile: &ProfileId) -> u32 {
        self.rows().iter().find(|row| row.is_clap() && row.profile_id == *profile).map_or(0, |row| row.count)
    }

    /// Who clapped and how many times, in the order they first clapped.
    pub fn clappers(&self) -> Vec<(ProfileId, u32)> {
        self.rows().iter().filter(|row| row.emoji == CLAP).map(|row| (row.profile_id.clone(), row.count)).collect()
    }

    /// Each profile's total (emoji reactions count one each, claps count each clap), in the
    /// order profiles first reacted.
    pub fn profile_totals(&self) -> Vec<(ProfileId, u32)> {
        let mut totals: Vec<(ProfileId, u32)> = Vec::new();
        for row in self.rows() {
            match totals.iter_mut().find(|(profile, _)| *profile == row.profile_id) {
                Some((_, total)) => *total = total.saturating_add(row.count),
                None => totals.push((row.profile_id.clone(), row.count)),
            }
        }
        totals
    }

    /// The emoji `profile` reacted with (claps left out), in order.
    pub fn emoji_of(&self, profile: &ProfileId) -> Vec<&str> {
        self.rows().iter().filter(|row| !row.is_clap() && row.profile_id == *profile).map(|row| row.emoji.as_str()).collect()
    }
}
