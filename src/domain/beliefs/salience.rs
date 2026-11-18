//! How much a belief matters to an agent right now, for ordering the profile list and for
//! choosing which beliefs go into the prompt:
//!
//! - **conviction:** distance from 0.5 (firm views shape what the agent says more than open
//!   questions);
//! - **recency:** halves every [`RECENCY_HALF_LIFE_DAYS`] days since the last change;
//! - **revisions:** a belief that was revisited is live (capped);
//! - **pinned:** a pinned belief always ranks above the rest.

use super::model::Belief;
use chrono::{DateTime, Utc};

/// Days after which the recency weight halves.
pub const RECENCY_HALF_LIFE_DAYS: f32 = 7.0;
/// Weight of each revision, and how many count.
const REVISION_WEIGHT: f32 = 0.1;
const MAX_COUNTED_REVISIONS: usize = 5;
/// Added for a pinned belief (above any unpinned score).
const PINNED_BOOST: f32 = 10.0;
const SECONDS_PER_DAY: f32 = 86_400.0;

/// The salience of `belief`, changed `revisions` times, at `now`.
pub fn salience(belief: &Belief, revisions: usize, now: DateTime<Utc>) -> f32 {
    let age_days = (now - belief.updated_at).num_seconds().max(0) as f32 / SECONDS_PER_DAY;
    let recency = 0.5_f32.powf(age_days / RECENCY_HALF_LIFE_DAYS);
    let revisited = revisions.min(MAX_COUNTED_REVISIONS) as f32 * REVISION_WEIGHT;
    let pinned = if belief.pinned { PINNED_BOOST } else { 0.0 };
    belief.credence.conviction() + recency + revisited + pinned
}

/// `beliefs` ordered by salience, highest first (ties by proposition, so the order is stable).
pub fn by_salience<'a>(beliefs: impl IntoIterator<Item = (&'a Belief, usize)>, now: DateTime<Utc>) -> Vec<&'a Belief> {
    let mut scored: Vec<(f32, &Belief)> = beliefs.into_iter().map(|(belief, revisions)| (salience(belief, revisions, now), belief)).collect();
    scored.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.proposition.cmp(&b.1.proposition)));
    scored.into_iter().map(|(_, belief)| belief).collect()
}
