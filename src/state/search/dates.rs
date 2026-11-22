//! `before:` / `after:` bounds as days: a `YYYY-MM-DD` day, or a word read from today.

use crate::domain::search_query::QueryDate;
use jiff::civil::Date;
use jiff::ToSpan;

/// The day `date` names, read from `today`: a day as typed, or `today`, `yesterday`,
/// `last-week` (7 days ago), `last-month` or `last-year`. `None` for anything else, and the
/// filter is then left out.
pub fn resolve_date(date: &QueryDate, today: Date) -> Option<Date> {
    let raw = match date {
        QueryDate::Day(day) => return Some(*day),
        QueryDate::Raw(raw) => raw.to_ascii_lowercase(),
    };
    let back = match raw.as_str() {
        "today" => 0.days(),
        "yesterday" => 1.day(),
        "last-week" => 1.week(),
        "last-month" => 1.month(),
        "last-year" => 1.year(),
        _ => return None,
    };
    today.checked_sub(back).ok()
}
