//! Equal periods over a span of days: one day each while the span fits the wanted count,
//! longer periods (whole days) otherwise. Shared by the concept timeline and the trends.

use jiff::civil::Date;

/// `count` periods of `days` days from `start`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Periods {
    pub start: Date,
    pub days: usize,
    pub count: usize,
}

impl Default for Periods {
    fn default() -> Self {
        Self { start: Date::MIN, days: 1, count: 0 }
    }
}

impl Periods {
    /// At most `max` periods covering `first` to `last` (inclusive).
    pub fn spanning(first: Date, last: Date, max: usize) -> Self {
        let (first, last) = (first.min(last), first.max(last));
        let span = (last - first).get_days() as usize + 1;
        let days = span.div_ceil(max.max(1)).max(1);
        Self { start: first, days, count: span.div_ceil(days) }
    }

    /// The period holding `day`, if it lies in the span.
    pub fn index_of(&self, day: Date) -> Option<usize> {
        let offset = (day - self.start).get_days();
        let index = usize::try_from(offset).ok()? / self.days;
        (index < self.count).then_some(index)
    }

    /// First day of period `index`.
    pub fn start_of(&self, index: usize) -> Date {
        self.start + crate::time::days((index * self.days) as i64)
    }

    /// Last day of period `index`.
    pub fn end_of(&self, index: usize) -> Date {
        self.start_of(index) + crate::time::days(self.days as i64 - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
