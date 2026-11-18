//! Whether a goal is on track: its progress score against the share of its time already
//! spent. Behind by more than [`SLACK`] is at risk; past the target date without reaching
//! [`ACHIEVED_SCORE`] is overdue.

use super::progress::GoalProgress;
use super::Goal;
use crate::domain::document::keyed::keyed_enum;
use crate::domain::review::day::days_between;
use chrono::NaiveDate;

/// Score at which a goal counts as reached.
pub const ACHIEVED_SCORE: f64 = 0.95;
/// How far progress may lag the time spent and still be on track.
pub const SLACK: f64 = 0.1;

keyed_enum! {
    /// Where a goal stands.
    Forecast {
        Achieved => "achieved",
        OnTrack => "on_track",
        AtRisk => "at_risk",
        Overdue => "overdue",
    }
}

/// The share (0–1) of the goal's time spent by `today`.
pub fn expected_share(goal: &Goal, today: NaiveDate) -> f64 {
    let total = days_between(goal.created_on, goal.target).max(1);
    (f64::from(days_between(goal.created_on, today)) / f64::from(total)).clamp(0.0, 1.0)
}

/// Where `goal` stands on `today` with `progress`.
pub fn forecast(goal: &Goal, progress: &GoalProgress, today: NaiveDate) -> Forecast {
    match (goal.completed || progress.score >= ACHIEVED_SCORE, today > goal.target) {
        (true, _) => Forecast::Achieved,
        (false, true) => Forecast::Overdue,
        (false, false) if progress.score + SLACK >= expected_share(goal, today) => Forecast::OnTrack,
        _ => Forecast::AtRisk,
    }
}
