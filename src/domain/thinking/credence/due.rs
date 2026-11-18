//! Which predictions are open, in the order they come due, and the due-date presets of the
//! credence popover.

use super::model::Credence;
use crate::domain::document::keyed::keyed_enum;
use chrono::{Days, Months, NaiveDate};
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

const WEEK_DAYS: u64 = 7;
const MONTH: u32 = 1;
const QUARTER_MONTHS: u32 = 3;
const YEAR_MONTHS: u32 = 12;

impl DuePreset {
    /// The date this preset names, from `today` (the last valid day when a month is short).
    pub fn date_from(self, today: NaiveDate) -> NaiveDate {
        let later = match self {
            Self::Week => today.checked_add_days(Days::new(WEEK_DAYS)),
            Self::Month => today.checked_add_months(Months::new(MONTH)),
            Self::Quarter => today.checked_add_months(Months::new(QUARTER_MONTHS)),
            Self::Year => today.checked_add_months(Months::new(YEAR_MONTHS)),
        };
        later.unwrap_or(today)
    }
}

/// Dated before undated, earlier dates first, then the older credence first.
fn by_due(a: &&Credence, b: &&Credence) -> Ordering {
    let rank = |c: &Credence| (c.resolves_by.is_none(), c.resolves_by);
    rank(*a).cmp(&rank(*b)).then(a.created_at.cmp(&b.created_at))
}

/// Every open prediction in the order it comes due.
pub fn open_by_due(credences: &[Credence]) -> Vec<&Credence> {
    let mut open: Vec<&Credence> = credences.iter().filter(|c| c.is_open()).collect();
    open.sort_by(by_due);
    open
}

/// Open predictions whose date has come by `today`, in due order.
pub fn due_on(credences: &[Credence], today: NaiveDate) -> Vec<&Credence> {
    open_by_due(credences).into_iter().filter(|c| c.is_due(today)).collect()
}
