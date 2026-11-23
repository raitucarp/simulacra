//! The calendar view (W4): one month as a grid of weeks, each day holding the posts written
//! on it (in the order written). The grid starts on the user's first weekday and pads the
//! first and last weeks with days of the neighbouring months (marked outside the month,
//! with their posts left out).

use crate::domain::preferences::FirstWeekday;
use crate::domain::{Post, PostId};
use jiff::civil::{Date, Weekday};
use jiff::{Timestamp, ToSpan};

/// Days in a week row.
pub const WEEK_DAYS: usize = 7;

/// A shown month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YearMonth {
    pub year: i16,
    /// 1 to 12.
    pub month: i8,
}

impl YearMonth {
    /// The month `day` falls in.
    pub fn of(day: Date) -> Self {
        Self { year: day.year(), month: day.month() }
    }

    /// Its first day.
    pub fn first_day(self) -> Date {
        Date::new(self.year, self.month, 1).unwrap_or_default()
    }

    /// The month `delta` months away (negative goes back).
    pub fn shifted(self, delta: i32) -> Self {
        Self::of(self.first_day().saturating_add(delta.months()))
    }
}

/// One cell of the grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarDay {
    pub date: Date,
    /// The day belongs to the shown month (padding days do not).
    pub in_month: bool,
    pub posts: Vec<PostId>,
}

/// The weekday a grid starting on `first` opens with.
pub fn first_column(first: FirstWeekday) -> Weekday {
    match first {
        FirstWeekday::Monday => Weekday::Monday,
        FirstWeekday::Sunday => Weekday::Sunday,
        FirstWeekday::Saturday => Weekday::Saturday,
    }
}

/// The weekdays of the columns, from `first`.
pub fn column_weekdays(first: FirstWeekday) -> [Weekday; WEEK_DAYS] {
    let start = first_column(first);
    std::array::from_fn(|i| start.wrapping_add(i as i64))
}

/// `month` as weeks of seven days from `first`, with the `posts` written on each day
/// (placed by `day_of`).
pub fn month_grid<'a>(month: YearMonth, first: FirstWeekday, posts: impl IntoIterator<Item = &'a Post>, day_of: impl Fn(Timestamp) -> Date) -> Vec<Vec<CalendarDay>> {
    let opening = month.first_day();
    let lead = i64::from(opening.weekday().since(first_column(first)));
    let start = opening.saturating_sub(lead.days());
    let mut written: Vec<(Date, Timestamp, PostId)> = posts.into_iter().map(|post| (day_of(post.created_at), post.created_at, post.id.clone())).filter(|(day, _, _)| YearMonth::of(*day) == month).collect();
    written.sort();
    let mut weeks = Vec::new();
    let mut day = start;
    while weeks.is_empty() || YearMonth::of(day) == month {
        let week: Vec<CalendarDay> = (0..WEEK_DAYS)
            .map(|offset| {
                let date = day.saturating_add((offset as i64).days());
                let posts = written.iter().filter(|(on, _, _)| *on == date).map(|(_, _, id)| id.clone()).collect();
                CalendarDay { date, in_month: YearMonth::of(date) == month, posts }
            })
            .collect();
        day = day.saturating_add((WEEK_DAYS as i64).days());
        weeks.push(week);
    }
    weeks
}
