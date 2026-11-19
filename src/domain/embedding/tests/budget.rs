use super::*;

const TODAY: &str = "2026-11-17";
const TOMORROW: &str = "2026-11-18";

#[test]
fn zero_limits_allow_everything() {
    let limits = BudgetLimits { daily_requests: 0, daily_tokens: 0 };
    let usage = DailyUsage { day: TODAY.into(), requests: 9_999, tokens: 9_999_999 };
    assert!(usage.allows(limits, TODAY, 1, 1_000));
}

#[test]
fn requests_and_tokens_are_capped_per_day() {
    let limits = BudgetLimits { daily_requests: 2, daily_tokens: 100 };
    let usage = DailyUsage::default().recorded(TODAY, 1, 60);
    assert!(usage.allows(limits, TODAY, 1, 40));
    assert!(!usage.allows(limits, TODAY, 1, 41), "the token cap");
    let usage = usage.recorded(TODAY, 1, 10);
    assert!(!usage.allows(limits, TODAY, 1, 1), "the request cap");
    assert!(usage.allows(limits, TOMORROW, 1, 1), "a new day starts over");
    assert_eq!(usage.recorded(TOMORROW, 1, 5), DailyUsage { day: TOMORROW.into(), requests: 1, tokens: 5 });
}

#[test]
fn usage_round_trips_and_tokens_are_estimated() {
    let usage = DailyUsage { day: TODAY.into(), requests: 3, tokens: 42 };
    assert_eq!(DailyUsage::from_json(&usage.to_json()), usage);
    assert_eq!(DailyUsage::from_json("not json"), DailyUsage::default());
    assert_eq!(estimate_tokens(["abcd", "abcde", ""]), 1 + 2 + 1);
    let limits = BudgetLimits::from_prefs(&crate::domain::preferences::AiPrefs::default());
    assert_eq!(limits, BudgetLimits { daily_requests: 0, daily_tokens: 0 });
}
