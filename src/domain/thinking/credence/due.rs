//! Which predictions are open, in the order they come due, and the due-date presets of the
//! credence popover.

use super::model::Credence;
use crate::domain::document::keyed::keyed_enum;
use jiff::civil::Date;
use jiff::Span;
use std::cmp::Ordering;

keyed_enum! {
    /// "Resolves by" shortcuts, counted from today.
    DuePreset {
        Week => "week",
        Month => "month",
        Quarter => "quarter",
        Year => "year",
    }
}

const WEEK_DAYS: i64 = 7;
const MONTH: i64 = 1;
const QUARTER_MONTHS: i64 = 3;
const YEAR_MONTHS: i64 = 12;

impl DuePreset {
    /// The date this preset names, from `today` (the last valid day when a month is short).
    pub fn date_from(self, today: Date) -> Date {
        let later = match self {
            Self::Week => today.checked_add(Span::new().days(WEEK_DAYS)).ok(),
            Self::Month => today.checked_add(Span::new().months(MONTH)).ok(),
            Self::Quarter => today.checked_add(Span::new().months(QUARTER_MONTHS)).ok(),
            Self::Year => today.checked_add(Span::new().months(YEAR_MONTHS)).ok(),
        };
        later.unwrap_or(today)
    }
}

/// Dated before undated, earlier dates first, then the older credence first.
fn by_due(a: &&Credence, b: &&Credence) -> Ordering {
    let rank = |c: &Credence| (c.resolves_by.is_none(), c.resolves_by);
    rank(a).cmp(&rank(b)).then(a.created_at.cmp(&b.created_at))
}

/// Every open prediction in the order it comes due.
pub fn open_by_due(credences: &[Credence]) -> Vec<&Credence> {
    let mut open: Vec<&Credence> = credences.iter().filter(|c| c.is_open()).collect();
    open.sort_by(by_due);
    open
}

/// Open predictions whose date has come by `today`, in due order.
pub fn due_on(credences: &[Credence], today: Date) -> Vec<&Credence> {
    open_by_due(credences).into_iter().filter(|c| c.is_due(today)).collect()
}
