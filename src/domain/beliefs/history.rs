//! The history of one belief as a series for its sparkline: the credence after each revision,
//! oldest first, ending at the current credence.

use super::book::BeliefBook;
use super::model::Belief;

/// Points a sparkline shows at most (the newest are kept).
pub const MAX_SPARK_POINTS: usize = 24;

/// Credences (0–1) of `belief` over time; a belief without logged history is one point.
pub fn credence_series(book: &BeliefBook, belief: &Belief) -> Vec<f32> {
    let mut points: Vec<f32> = book.history(&belief.id).filter_map(|revision| revision.new).map(|credence| credence.unit()).collect();
    if points.last() != Some(&belief.credence.unit()) {
        points.push(belief.credence.unit());
    }
    let skip = points.len().saturating_sub(MAX_SPARK_POINTS);
    points.split_off(skip)
}
