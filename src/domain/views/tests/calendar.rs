use super::{date, locked, post_at, utc_day};
use crate::domain::preferences::FirstWeekday;
use crate::domain::views::calendar::{column_weekdays, month_grid, YearMonth, WEEK_DAYS};
use jiff::civil::Weekday;

const NOVEMBER: YearMonth = YearMonth { year: 2026, month: 11 };

#[test]
fn the_grid_starts_on_the_first_weekday_and_covers_the_month() {
    let weeks = month_grid(NOVEMBER, FirstWeekday::Monday, std::iter::empty(), utc_day);
    assert!(weeks.iter().all(|week| week.len() == WEEK_DAYS));
    assert_eq!(weeks[0][0].date, date("2026-10-26"), "1 November 2026 is a Sunday");
    assert!(!weeks[0][0].in_month && weeks[0][6].in_month);
    assert_eq!(weeks.len(), 6);
    let sunday_first = month_grid(NOVEMBER, FirstWeekday::Sunday, std::iter::empty(), utc_day);
    assert_eq!(sunday_first[0][0].date, date("2026-11-01"));
    assert_eq!(sunday_first.len(), 5);
    assert_eq!(column_weekdays(FirstWeekday::Saturday)[..2], [Weekday::Saturday, Weekday::Sunday]);
}

#[test]
fn posts_land_on_their_day_in_the_order_written_and_other_months_stay_out() {
    let posts = [
        post_at("late", "evening", "2026-11-03T20:00:00Z"),
        post_at("early", "morning", "2026-11-03T07:00:00Z"),
        post_at("october", "outside", "2026-10-31T12:00:00Z"),
        locked(post_at("sealed", "secret", "2026-11-04T12:00:00Z")),
    ];
    let weeks = month_grid(NOVEMBER, FirstWeekday::Monday, &posts, utc_day);
    let day = |raw: &str| weeks.iter().flatten().find(|cell| cell.date == date(raw)).expect("in grid").clone();
    assert_eq!(day("2026-11-03").posts.iter().map(|id| id.as_str()).collect::<Vec<_>>(), ["early", "late"]);
    assert!(day("2026-10-31").posts.is_empty(), "padding days list nothing");
    assert_eq!(day("2026-11-04").posts.len(), 1, "a locked post is placed; its card shows the placeholder");
}

#[test]
fn months_step_across_years() {
    assert_eq!(NOVEMBER.shifted(2), YearMonth { year: 2027, month: 1 });
    assert_eq!(NOVEMBER.shifted(-11), YearMonth { year: 2025, month: 12 });
    assert_eq!(YearMonth::of(date("2026-02-28")).first_day(), date("2026-02-01"));
}
