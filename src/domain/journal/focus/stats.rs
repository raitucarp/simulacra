//! The session log's statistics: sessions, focus minutes, how often the intention was met,
//! and minutes per day.

use super::{FocusSession, Outcome};
use chrono::NaiveDate;
use std::collections::BTreeMap;

/// Totals over the log.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FocusTotals {
    pub sessions: usize,
    pub minutes: u32,
    pub done: usize,
    pub partly: usize,
    pub not_done: usize,
    /// Share of reflected sessions marked done, when any were reflected on.
    pub done_share: Option<f32>,
}

pub fn totals(sessions: &[FocusSession]) -> FocusTotals {
    let count = |outcome: Outcome| sessions.iter().filter(|s| s.outcome == Some(outcome)).count();
    let (done, partly, not_done) = (count(Outcome::Done), count(Outcome::Partly), count(Outcome::NotDone));
    let reflected = done + partly + not_done;
    FocusTotals {
        sessions: sessions.len(),
        minutes: sessions.iter().map(|s| s.focused_minutes).sum(),
        done,
        partly,
        not_done,
        done_share: (reflected > 0).then(|| done as f32 / reflected as f32),
    }
}

/// Focus minutes per journal day.
pub fn minutes_by_day(sessions: &[FocusSession]) -> BTreeMap<NaiveDate, u32> {
    sessions.iter().fold(BTreeMap::new(), |mut days, s| {
        *days.entry(s.day).or_default() += s.focused_minutes;
        days
    })
}

/// Sessions newest first.
pub fn newest_sessions(sessions: &[FocusSession]) -> Vec<&FocusSession> {
    let mut sorted: Vec<&FocusSession> = sessions.iter().collect();
    sorted.sort_by_key(|a| std::cmp::Reverse(a.started_at));
    sorted
}
