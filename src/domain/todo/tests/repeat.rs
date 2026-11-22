//! D4: repeat rules and their next occurrence (pure).

use super::super::repeat::{parse_repeat, RepeatRule, RepeatUnit};
use jiff::civil::{date, Date, Weekday};

fn next(rule: &str, anchor: Date, after: Date) -> Option<Date> {
    parse_repeat(rule).expect("a rule").next_after(anchor, after)
}

#[test]
fn rules_are_read_in_english_ignoring_case() {
    let every = |count, unit| Some(RepeatRule::Every { count, unit });
    assert_eq!(parse_repeat("Daily"), every(1, RepeatUnit::Day));
    assert_eq!(parse_repeat("fortnightly"), every(2, RepeatUnit::Week));
    assert_eq!(parse_repeat("quarterly"), every(3, RepeatUnit::Month));
    assert_eq!(parse_repeat("annually"), every(1, RepeatUnit::Year));
    assert_eq!(parse_repeat("3d"), every(3, RepeatUnit::Day));
    assert_eq!(parse_repeat("6m"), every(6, RepeatUnit::Month));
    assert_eq!(parse_repeat("wed,mon,wed"), Some(RepeatRule::Weekdays(vec![Weekday::Monday, Weekday::Wednesday])));
    assert_eq!(parse_repeat("15th,1st"), Some(RepeatRule::MonthDays(vec![1, 15])));
    assert_eq!(parse_repeat("weekends"), Some(RepeatRule::Weekdays(vec![Weekday::Saturday, Weekday::Sunday])));
    for nonsense in ["", "0d", "1000d", "3x", "mon,15th", "sometimes", "mon,,wed", "32nd"] {
        assert_eq!(parse_repeat(nonsense), None, "{nonsense}");
    }
}

#[test]
fn day_and_week_intervals_count_from_the_anchor() {
    let anchor = date(2026, 11, 22);
    assert_eq!(next("daily", anchor, anchor), Some(date(2026, 11, 23)));
    assert_eq!(next("daily", anchor, date(2026, 11, 30)), Some(date(2026, 12, 1)));
    assert_eq!(next("3d", anchor, date(2026, 11, 26)), Some(date(2026, 11, 28)));
    assert_eq!(next("weekly", anchor, date(2026, 11, 24)), Some(date(2026, 11, 29)));
    assert_eq!(next("biweekly", anchor, anchor), Some(date(2026, 12, 6)));
    assert_eq!(next("weekly", anchor, date(2026, 11, 1)), Some(anchor), "before the series starts, the anchor is next");
    assert_eq!(next("daily", date(2026, 12, 31), date(2026, 12, 31)), Some(date(2027, 1, 1)));
}

#[test]
fn month_and_year_intervals_do_not_drift_at_month_ends() {
    let anchor = date(2026, 1, 31);
    assert_eq!(next("monthly", anchor, anchor), Some(date(2026, 2, 28)));
    assert_eq!(next("monthly", anchor, date(2026, 2, 28)), Some(date(2026, 3, 31)), "back on the 31st after February");
    assert_eq!(next("monthly", anchor, date(2026, 4, 15)), Some(date(2026, 4, 30)));
    assert_eq!(next("quarterly", anchor, anchor), Some(date(2026, 4, 30)));
    assert_eq!(next("monthly", date(2028, 1, 31), date(2028, 1, 31)), Some(date(2028, 2, 29)));
    assert_eq!(next("yearly", date(2028, 2, 29), date(2028, 2, 29)), Some(date(2029, 2, 28)));
    assert_eq!(next("yearly", date(2028, 2, 29), date(2031, 6, 1)), Some(date(2032, 2, 29)));
    assert_eq!(next("monthly", anchor, date(2025, 12, 1)), Some(anchor));
}

#[test]
fn weekday_rules_take_the_next_listed_day() {
    let sunday = date(2026, 11, 22);
    assert_eq!(next("mon,wed", sunday, sunday), Some(date(2026, 11, 23)));
    assert_eq!(next("mon,wed", sunday, date(2026, 11, 23)), Some(date(2026, 11, 25)));
    assert_eq!(next("mon,wed", sunday, date(2026, 11, 25)), Some(date(2026, 11, 30)));
    assert_eq!(next("weekdays", sunday, date(2026, 11, 27)), Some(date(2026, 11, 30)), "Friday skips the weekend");
    assert_eq!(next("sun", sunday, sunday), Some(date(2026, 11, 29)));
}

#[test]
fn month_day_rules_use_the_last_day_of_short_months() {
    let anchor = date(2026, 1, 1);
    assert_eq!(next("15th", anchor, date(2026, 11, 22)), Some(date(2026, 12, 15)));
    assert_eq!(next("1st,15th", anchor, date(2026, 12, 1)), Some(date(2026, 12, 15)));
    assert_eq!(next("1st,15th", anchor, date(2026, 12, 15)), Some(date(2027, 1, 1)));
    assert_eq!(next("31st", anchor, date(2026, 1, 31)), Some(date(2026, 2, 28)));
    assert_eq!(next("31st", anchor, date(2026, 2, 28)), Some(date(2026, 3, 31)));
    assert_eq!(next("30th,31st", anchor, date(2026, 4, 29)), Some(date(2026, 4, 30)));
    assert_eq!(next("30th,31st", anchor, date(2026, 4, 30)), Some(date(2026, 5, 30)));
}

#[test]
fn the_help_card_samples_are_real_rules_and_dates() {
    use super::super::syntax::{SyntaxTopic, SYNTAX_ROWS};
    let sample = |topic| SYNTAX_ROWS.iter().find(|row| row.topic == topic).map(|row| row.sample).unwrap_or_default();
    for rule in sample(SyntaxTopic::Repeat).split_whitespace() {
        assert!(parse_repeat(rule.trim_start_matches('^')).is_some(), "{rule}");
    }
    for phrase in sample(SyntaxTopic::Due).split_whitespace() {
        assert!(super::super::due::resolve_due(phrase.trim_start_matches('^'), date(2026, 11, 22)).is_some(), "{phrase}");
    }
}
