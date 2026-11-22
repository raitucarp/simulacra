//! Due phrases (D3): what the `^` of a to-do means as a calendar day, read in English.
//!
//! Phrases are resolved against the day the post was written, in the user's time zone, so
//! `^fri` keeps meaning the Friday after the post was written however late it is read.
//!
//! - `2026-12-01`: that day. `12-01`: the next 1 December on or after the day.
//! - `today`, `tod`; `tomorrow`, `tom`, `tmr`, `tmrw`.
//! - A weekday (`fri`, `friday`, `thurs`): the next one on or after the day (the day itself
//!   when it is that weekday). `next fri`: the next one after the day.
//! - `next week`: the Monday after; `next month`: the 1st of the next month; `next year`:
//!   1 January of the next year. `eow` / `end-of-week`: the Sunday on or after the day;
//!   `eom` / `end-of-month`: the month's last day.
//! - `15th`, `1st`, `22nd`: the next such day of a month on or after the day; a month without
//!   it (the 31st in November) uses its last day.
//! - `+3d`, `+2w`, `+1m`, `+1y`, `+3` (days): that far after the day. A month or year later
//!   lands on the same day of the month or on the month's last day (31 January + 1 month is
//!   28 or 29 February).
//!
//! Words may be joined with `-` instead of a space (`^next-month`).

use jiff::civil::{Date, Weekday};
use jiff::tz::TimeZone;
use jiff::{Timestamp, ToSpan};

/// The longest relative step read, in units (keeps arithmetic far from overflowing).
const MAX_STEP: i64 = 10_000;
const DAYS_PER_WEEK: i64 = 7;
/// Years tried when a month-day (`02-29`) does not exist in the next one.
const MONTH_DAY_YEARS: i16 = 8;

/// The calendar day of `at` in `tz`.
pub fn local_day(at: Timestamp, tz: &TimeZone) -> Date {
    at.to_zoned_datetime(tz.clone()).date()
}

/// The day `phrase` (without `^`) names, written on `today`; `None` when it is not a date.
pub fn resolve_due(phrase: &str, today: Date) -> Option<Date> {
    let phrase = phrase.trim().to_lowercase();
    if let Some(day) = iso_day(&phrase).or_else(|| month_day(&phrase, today)) {
        return Some(day);
    }
    if let Some(step) = phrase.strip_prefix('+') {
        return relative(step, today);
    }
    let words: Vec<&str> = phrase.split(|c: char| c == '-' || c == '_' || c.is_whitespace()).filter(|w| !w.is_empty()).collect();
    match words.as_slice() {
        [word] => single(word, today),
        ["next", word] => next(word, today),
        ["end", "of", "week"] => single("eow", today),
        ["end", "of", "month"] => single("eom", today),
        _ => None,
    }
}

/// Whether a due phrase starting with `word` needs the word after it (`^next month`).
pub fn takes_next_word(word: &str) -> bool {
    word.eq_ignore_ascii_case("next")
}

/// A weekday by its English name or a common abbreviation.
pub fn weekday(word: &str) -> Option<Weekday> {
    Some(match word {
        "mon" | "monday" => Weekday::Monday,
        "tue" | "tues" | "tuesday" => Weekday::Tuesday,
        "wed" | "weds" | "wednesday" => Weekday::Wednesday,
        "thu" | "thur" | "thurs" | "thursday" => Weekday::Thursday,
        "fri" | "friday" => Weekday::Friday,
        "sat" | "saturday" => Weekday::Saturday,
        "sun" | "sunday" => Weekday::Sunday,
        _ => return None,
    })
}

/// The day of the month an ordinal names (`1st`, `2nd`, `3rd`, `15th`), 1 to 31.
pub fn ordinal_day(word: &str) -> Option<i8> {
    let digits = word.strip_suffix("st").or_else(|| word.strip_suffix("nd")).or_else(|| word.strip_suffix("rd")).or_else(|| word.strip_suffix("th"))?;
    let day: i8 = digits.parse().ok()?;
    (1..=31).contains(&day).then_some(day)
}

/// `day` of the month of `month_of`, or the month's last day when it is shorter.
pub fn day_in_month(month_of: Date, day: i8) -> Date {
    let last = month_of.last_of_month();
    if day >= last.day() { last } else { month_of.first_of_month().saturating_add(i64::from(day - 1).days()) }
}

fn iso_day(phrase: &str) -> Option<Date> {
    let shaped = phrase.len() == "2026-12-01".len() && phrase.as_bytes().get(4) == Some(&b'-');
    if shaped { phrase.parse().ok() } else { None }
}

fn month_day(phrase: &str, today: Date) -> Option<Date> {
    let (month, day) = phrase.split_once('-')?;
    let all_digits = |s: &str| !s.is_empty() && s.len() <= 2 && s.chars().all(|c| c.is_ascii_digit());
    if !all_digits(month) || !all_digits(day) {
        return None;
    }
    let (month, day): (i8, i8) = (month.parse().ok()?, day.parse().ok()?);
    (0..MONTH_DAY_YEARS).filter_map(|ahead| Date::new(today.year().checked_add(ahead)?, month, day).ok()).find(|date| *date >= today)
}

fn relative(step: &str, today: Date) -> Option<Date> {
    let split = step.find(|c: char| !c.is_ascii_digit()).unwrap_or(step.len());
    let count: i64 = step[..split].parse().ok().filter(|n| *n <= MAX_STEP)?;
    let span = match &step[split..] {
        "" | "d" => count.days(),
        "w" => (count * DAYS_PER_WEEK).days(),
        "m" => count.months(),
        "y" => count.years(),
        _ => return None,
    };
    today.checked_add(span).ok()
}

fn single(word: &str, today: Date) -> Option<Date> {
    match word {
        "today" | "tod" => Some(today),
        "tomorrow" | "tom" | "tmr" | "tmrw" => today.tomorrow().ok(),
        "eow" => Some(on_or_after(today, Weekday::Sunday)),
        "eom" => Some(today.last_of_month()),
        _ => weekday(word).map(|day| on_or_after(today, day)).or_else(|| ordinal_day(word).map(|day| next_day_of_month(today, day))),
    }
}

fn next(word: &str, today: Date) -> Option<Date> {
    match word {
        "week" => Some(after(today, Weekday::Monday)),
        "month" => today.first_of_month().checked_add(1.month()).ok(),
        "year" => today.first_of_year().checked_add(1.year()).ok(),
        _ => weekday(word).map(|day| after(today, day)),
    }
}

/// The first `day` on or after `today`.
pub fn on_or_after(today: Date, day: Weekday) -> Date {
    today.saturating_add(i64::from(today.weekday().until(day)).days())
}

/// The first `day` strictly after `today`.
fn after(today: Date, day: Weekday) -> Date {
    on_or_after(today.saturating_add(1.day()), day)
}

/// The first date on or after `today` on the `day`-th of its month (clamped to short months).
fn next_day_of_month(today: Date, day: i8) -> Date {
    let this_month = day_in_month(today, day);
    if this_month >= today { this_month } else { day_in_month(today.first_of_month().saturating_add(1.month()), day) }
}
