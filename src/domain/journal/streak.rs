//! Streaks of days with an entry, told gently: today without an entry yet does not break
//! the current streak (it still counts up to yesterday), and the caller never words a missed
//! day as a failure.

use jiff::civil::Date;
use std::collections::BTreeSet;

/// Days with writing, and the streaks they make.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Streaks {
    /// Consecutive days ending today, or yesterday while today is still open.
    pub current: usize,
    pub longest: usize,
    /// Distinct days written on.
    pub days: usize,
}

/// The run of consecutive days in `days` that ends on `last`.
fn run_ending(days: &BTreeSet<Date>, last: Date) -> usize {
    std::iter::successors(Some(last), |day| day.checked_sub(crate::time::days(1)).ok()).take_while(|day| days.contains(day)).count()
}

/// The longest run of consecutive days.
fn longest_run(days: &BTreeSet<Date>) -> usize {
    let mut previous: Option<Date> = None;
    let (mut best, mut run) = (0, 0);
    for &day in days {
        run = if previous.is_some_and(|p| day.duration_since(p) == crate::time::days(1)) { run + 1 } else { 1 };
        best = best.max(run);
        previous = Some(day);
    }
    best
}

/// The streaks of `days` as seen on `today` (days after today are ignored).
pub fn streaks(days: impl IntoIterator<Item = Date>, today: Date) -> Streaks {
    let days: BTreeSet<Date> = days.into_iter().filter(|day| *day <= today).collect();
    let yesterday = today - crate::time::days(1);
    let current = if days.contains(&today) { run_ending(&days, today) } else { run_ending(&days, yesterday) };
    Streaks { current, longest: longest_run(&days), days: days.len() }
}
