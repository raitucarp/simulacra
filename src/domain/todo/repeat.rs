//! Repeat rules (D4): what the `^^` of a to-do means, and when it next comes round.
//!
//! - `daily`, `weekly`, `biweekly` / `fortnightly`, `monthly`, `quarterly`, `yearly` /
//!   `annually`, and `3d`, `2w`, `6m`, `1y`: every so many days, weeks, months or years,
//!   counted from the anchor (the item's first due day). Months and years are always counted
//!   from the anchor, so a series on the 31st lands on the 30th in April and on the 31st
//!   again in May instead of drifting.
//! - `mon,wed`, `weekdays` (Monday to Friday), `weekends`: on those weekdays.
//! - `15th`, `1st,15th`: on those days of the month; a month without the day uses its last.
//!
//! Read in English, ignoring case. Anything else is not a rule.

use super::due::{day_in_month, ordinal_day, weekday};
use jiff::civil::{Date, Weekday};
use jiff::ToSpan;

const DAYS_PER_WEEK: i64 = 7;
const MONTHS_PER_YEAR: i64 = 12;
/// The largest count of a rule (`999d`).
const MAX_COUNT: u16 = 999;
/// Months searched for the next day of a month (every day list fits in one year and a bit).
const MONTHS_SEARCHED: i64 = 13;
const WORK_DAYS: [Weekday; 5] = [Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday];
const WEEKEND: [Weekday; 2] = [Weekday::Saturday, Weekday::Sunday];

/// The unit an interval rule counts in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatUnit {
    Day,
    Week,
    Month,
    Year,
}

/// A parsed repeat rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepeatRule {
    /// Every `count` units from the anchor.
    Every { count: u16, unit: RepeatUnit },
    /// On these weekdays, Monday first.
    Weekdays(Vec<Weekday>),
    /// On these days of the month, in order.
    MonthDays(Vec<i8>),
}

/// Reads a rule (without `^^`); `None` when it is not one.
pub fn parse_repeat(rule: &str) -> Option<RepeatRule> {
    let rule = rule.trim().to_lowercase();
    if let Some(found) = named(&rule).or_else(|| interval(&rule)) {
        return Some(found);
    }
    let parts: Vec<&str> = rule.split(',').map(str::trim).collect();
    if let Some(mut days) = parts.iter().map(|part| weekday(part)).collect::<Option<Vec<_>>>() {
        days.sort_by_key(|day| day.to_monday_zero_offset());
        days.dedup();
        return Some(RepeatRule::Weekdays(days));
    }
    let mut days = parts.iter().map(|part| ordinal_day(part)).collect::<Option<Vec<_>>>()?;
    days.sort_unstable();
    days.dedup();
    Some(RepeatRule::MonthDays(days))
}

fn named(rule: &str) -> Option<RepeatRule> {
    let every = |count, unit| Some(RepeatRule::Every { count, unit });
    match rule {
        "daily" | "everyday" => every(1, RepeatUnit::Day),
        "weekly" => every(1, RepeatUnit::Week),
        "biweekly" | "fortnightly" => every(2, RepeatUnit::Week),
        "monthly" => every(1, RepeatUnit::Month),
        "quarterly" => every(3, RepeatUnit::Month),
        "yearly" | "annually" => every(1, RepeatUnit::Year),
        "weekdays" => Some(RepeatRule::Weekdays(WORK_DAYS.to_vec())),
        "weekends" => Some(RepeatRule::Weekdays(WEEKEND.to_vec())),
        _ => None,
    }
}

fn interval(rule: &str) -> Option<RepeatRule> {
    let split = rule.find(|c: char| !c.is_ascii_digit())?;
    let count: u16 = rule[..split].parse().ok().filter(|n| (1..=MAX_COUNT).contains(n))?;
    let unit = match &rule[split..] {
        "d" => RepeatUnit::Day,
        "w" => RepeatUnit::Week,
        "m" => RepeatUnit::Month,
        "y" => RepeatUnit::Year,
        _ => return None,
    };
    Some(RepeatRule::Every { count, unit })
}

impl RepeatRule {
    /// The first occurrence strictly after `after` of the series that starts on `anchor`
    /// (interval rules count from it; weekday and month-day rules ignore it).
    pub fn next_after(&self, anchor: Date, after: Date) -> Option<Date> {
        match self {
            Self::Every { count, unit: unit @ (RepeatUnit::Day | RepeatUnit::Week) } => {
                let step = i64::from(*count) * if *unit == RepeatUnit::Week { DAYS_PER_WEEK } else { 1 };
                let gone = i64::from((after - anchor).get_days());
                let steps = if gone < 0 { 0 } else { gone / step + 1 };
                anchor.checked_add((steps * step).days()).ok()
            }
            Self::Every { count, unit } => {
                let step = i64::from(*count) * if *unit == RepeatUnit::Year { MONTHS_PER_YEAR } else { 1 };
                let months_gone = (i64::from(after.year()) - i64::from(anchor.year())) * MONTHS_PER_YEAR + i64::from(after.month() - anchor.month());
                let first = (months_gone / step - 1).max(0);
                (first..).map(|k| anchor.checked_add((k * step).months()).ok()).take(3).flatten().find(|day| *day > after)
            }
            Self::Weekdays(days) => (1..=DAYS_PER_WEEK).map(|ahead| after.saturating_add(ahead.days())).find(|day| days.contains(&day.weekday())),
            Self::MonthDays(days) => (0..MONTHS_SEARCHED)
                .map(|ahead| after.first_of_month().saturating_add(ahead.months()))
                .flat_map(|month| days.iter().map(move |day| day_in_month(month, *day)))
                .find(|day| *day > after),
        }
    }
}
