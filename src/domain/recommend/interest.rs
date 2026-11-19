//! A profile's interest vector: the weighted, recency-decayed mean of the meaning of the
//! posts it engaged with.

use crate::domain::embedding::{centroid, l2_normalized, EmbeddingOwner, VectorIndex};
use crate::domain::PostId;
use jiff::Timestamp;

/// Days after which an engagement counts half as much as a fresh one.
pub const INTEREST_HALF_LIFE_DAYS: f64 = 30.0;
const SECONDS_PER_DAY: f64 = 86_400.0;

/// One engagement with a post, and how much it says about the profile's interests.
#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    pub post: PostId,
    /// Relative strength (writing a post says more than liking one).
    pub weight: f32,
    /// When it happened.
    pub at: Timestamp,
}

/// The unit interest vector of `signals` under `model`, or `None` when none of their posts
/// is embedded yet.
pub fn interest_vector(index: &VectorIndex, model: &str, signals: &[Signal], now: Timestamp) -> Option<Vec<f32>> {
    let mut sum: Option<Vec<f32>> = None;
    for signal in signals {
        let Some(vector) = centroid(index, &EmbeddingOwner::post(&signal.post), model) else { continue };
        let weight = signal.weight * decay(now, signal.at);
        let total = sum.get_or_insert_with(|| vec![0.0; vector.len()]);
        if total.len() == vector.len() {
            total.iter_mut().zip(&vector).for_each(|(t, v)| *t += weight * v);
        }
    }
    sum.map(l2_normalized).filter(|vector| vector.iter().any(|v| *v != 0.0))
}

/// How much an engagement at `at` still counts at `now` (1 when fresh, halving every
/// [`INTEREST_HALF_LIFE_DAYS`]; future times count fully).
fn decay(now: Timestamp, at: Timestamp) -> f32 {
    let days = now.duration_since(at).as_secs_f64().max(0.0) / SECONDS_PER_DAY;
    0.5_f64.powf(days / INTEREST_HALF_LIFE_DAYS) as f32
}
