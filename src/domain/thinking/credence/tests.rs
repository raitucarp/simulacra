//! Credence model, due ordering, scoring rules, Wilson intervals, the calibration curve,
//! Murphy's decomposition, confidence bias and the summary.

use super::*;
use crate::domain::PostId;
use jiff::Timestamp;
use jiff::civil::Date;

const CLOSE: f64 = 1e-9;

fn now() -> Timestamp {
    crate::time::utc(2026, 11, 18, 12, 0, 0)
}

fn day(y: i16, m: i8, d: i8) -> Date {
    jiff::civil::date(y, m, d)
}

fn credence(id: &str, percent: u8) -> Credence {
    Credence::new(PostId::new(id), percent, now())
}

fn forecast(probability: f64, happened: bool) -> Forecast {
    Forecast { probability, happened }
}

fn repeated(probability: f64, hits: usize, misses: usize) -> Vec<Forecast> {
    std::iter::repeat(forecast(probability, true)).take(hits).chain(std::iter::repeat(forecast(probability, false)).take(misses)).collect()
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < CLOSE
}

#[test]
fn percent_is_clamped_and_read_as_probability() {
    assert_eq!(clamp_percent(0), MIN_PERCENT);
    assert_eq!(clamp_percent(150), MAX_PERCENT);
    assert_eq!(clamp_percent(-4), MIN_PERCENT);
    assert_eq!(credence("a", 100).percent, MAX_PERCENT);
    assert_eq!(credence("a", 70).with_percent(0).percent, MIN_PERCENT);
    assert!(close(credence("a", 70).probability(), 0.7));
    assert_eq!(Outcome::from_key("ambiguous"), Some(Outcome::Ambiguous));
}

#[test]
fn resolving_closes_and_reopening_opens() {
    let open = credence("a", 60).with_due(Some(day(2026, 11, 20))).with_sentence(Some("  ".into()));
    assert!(open.is_open() && open.sentence.is_none());
    assert!(!open.is_due(day(2026, 11, 19)));
    assert!(open.is_due(day(2026, 11, 20)));
    let closed = open.clone().resolved(Outcome::True, "  it held ", now());
    assert!(!closed.is_open() && !closed.is_due(day(2026, 12, 1)));
    assert_eq!(closed.note, "it held");
    assert_eq!(closed.resolved_at, Some(now()));
    let again = closed.reopened();
    assert!(again.is_open() && again.resolved_at.is_none());
    assert_eq!(credence("a", 50).with_sentence(Some(" A sentence. ".into())).sentence.as_deref(), Some("A sentence."));
}

#[test]
fn open_predictions_come_in_due_order() {
    let later = Credence { created_at: now() + jiff::SignedDuration::from_hours(1), ..credence("late", 50) };
    let list = vec![
        credence("undated", 50),
        credence("second", 50).with_due(Some(day(2026, 12, 1))),
        credence("closed", 50).with_due(Some(day(2026, 1, 1))).resolved(Outcome::False, "", now()),
        later.with_due(Some(day(2026, 11, 20))),
        credence("first", 50).with_due(Some(day(2026, 11, 20))),
    ];
    let ids: Vec<&str> = open_by_due(&list).iter().map(|c| c.post_id.as_str()).collect();
    assert_eq!(ids, ["first", "late", "second", "undated"]);
    let due: Vec<&str> = due_on(&list, day(2026, 11, 25)).iter().map(|c| c.post_id.as_str()).collect();
    assert_eq!(due, ["first", "late"]);
}

#[test]
fn due_presets_count_from_today() {
    let today = day(2026, 1, 31);
    assert_eq!(DuePreset::Week.date_from(today), day(2026, 2, 7));
    assert_eq!(DuePreset::Month.date_from(today), day(2026, 2, 28));
    assert_eq!(DuePreset::Quarter.date_from(today), day(2026, 4, 30));
    assert_eq!(DuePreset::Year.date_from(today), day(2027, 1, 31));
}

#[test]
fn only_true_and_false_outcomes_are_scored() {
    let list = vec![
        credence("t", 80).resolved(Outcome::True, "", now()),
        credence("f", 30).resolved(Outcome::False, "", now()),
        credence("a", 90).resolved(Outcome::Ambiguous, "", now()),
        credence("o", 40),
    ];
    assert_eq!(forecasts(&list), vec![forecast(0.8, true), forecast(0.3, false)]);
}

#[test]
fn brier_and_log_scores_have_their_known_values() {
    assert_eq!(brier_score(&[]), None);
    assert_eq!(log_score(&[]), None);
    assert!(close(brier_score(&[forecast(1.0, true), forecast(0.0, false)]).unwrap_or(1.0), 0.0));
    assert!(close(brier_score(&[forecast(0.7, true)]).unwrap_or_default(), 0.09));
    assert!(close(brier_score(&[forecast(0.5, true), forecast(0.5, false)]).unwrap_or_default(), 0.25));
    assert!(close(log_score(&[forecast(0.5, false)]).unwrap_or_default(), -1.0));
    assert!(close(log_score(&[forecast(0.25, false)]).unwrap_or_default(), 0.75f64.log2()));
    assert!(log_score(&[forecast(0.99, false)]).unwrap_or_default() < log_score(&[forecast(0.6, false)]).unwrap_or_default());
}

#[test]
fn wilson_intervals_bracket_the_frequency() {
    assert_eq!(wilson_interval(0, 0, WILSON_Z), None);
    let half = wilson_interval(5, 10, WILSON_Z).expect("interval");
    assert!((half.low - 0.2366).abs() < 1e-3 && (half.high - 0.7634).abs() < 1e-3, "{half:?}");
    let none = wilson_interval(0, 4, WILSON_Z).expect("interval");
    assert!(close(none.low, 0.0) && none.high > 0.0 && none.high < 1.0);
    let all = wilson_interval(4, 4, WILSON_Z).expect("interval");
    assert!(close(all.high, 1.0) && all.low > 0.0);
    let wide = wilson_interval(1, 2, WILSON_Z).expect("interval");
    let narrow = wilson_interval(50, 100, WILSON_Z).expect("interval");
    assert!(wide.high - wide.low > narrow.high - narrow.low);
}

#[test]
fn the_calibration_curve_bins_by_stated_probability() {
    assert_eq!(bin_index(0.0, CALIBRATION_BINS), 0);
    assert_eq!(bin_index(0.25, CALIBRATION_BINS), 2);
    assert_eq!(bin_index(1.0, CALIBRATION_BINS), CALIBRATION_BINS - 1);
    let mut list = repeated(0.72, 3, 1);
    list.push(forecast(1.0, true));
    let curve = calibration_curve(&list, CALIBRATION_BINS);
    assert_eq!(curve.len(), CALIBRATION_BINS);
    let seventh = &curve[7];
    assert_eq!((seventh.count, seventh.hits), (4, 3));
    assert!(close(seventh.mean_forecast, 0.72) && close(seventh.observed, 0.75));
    assert!(close(seventh.low, 0.7) && close(seventh.high, 0.8));
    assert!(seventh.interval.is_some());
    assert_eq!(curve[9].count, 1);
    assert_eq!((curve[0].count, curve[0].interval), (0, None));
    assert_eq!(calibration_curve(&list, 0).len(), 1);
}

#[test]
fn murphy_parts_add_up_to_the_brier_score() {
    assert_eq!(murphy_decomposition(&[], CALIBRATION_BINS), None);
    let list: Vec<Forecast> = repeated(0.25, 1, 3).into_iter().chain(repeated(0.75, 3, 1)).collect();
    let parts = murphy_decomposition(&list, CALIBRATION_BINS).expect("parts");
    assert!(close(parts.reliability, 0.0));
    assert!(close(parts.resolution, 0.0625));
    assert!(close(parts.uncertainty, 0.25));
    assert!(close(parts.brier, 0.1875));
    assert!(close(parts.brier, brier_score(&list).unwrap_or_default()));
    assert!(close(parts.reliability - parts.resolution + parts.uncertainty, parts.brier));
    let miscalibrated = murphy_decomposition(&repeated(0.95, 1, 1), CALIBRATION_BINS).expect("parts");
    assert!(close(miscalibrated.reliability, 0.45 * 0.45));
    assert!(close(miscalibrated.brier, brier_score(&repeated(0.95, 1, 1)).unwrap_or_default()));
}

#[test]
fn confidence_bias_has_the_right_sign() {
    let sure = repeated(0.9, 7, 3);
    assert!(close(overconfidence(&sure).unwrap_or_default(), 0.2));
    assert_eq!(confidence_bias(&sure).map(|(bias, _)| bias), Some(ConfidenceBias::Overconfident));
    let timid = repeated(0.6, 10, 0);
    assert!(close(overconfidence(&timid).unwrap_or_default(), -0.4));
    assert_eq!(confidence_bias(&timid).map(|(bias, _)| bias), Some(ConfidenceBias::Underconfident));
    let mirrored = vec![forecast(0.2, false), forecast(0.2, false), forecast(0.2, false), forecast(0.2, false), forecast(0.2, true)];
    assert_eq!(confidence_bias(&mirrored).map(|(bias, _)| bias), Some(ConfidenceBias::Calibrated));
    assert_eq!(overconfidence(&[forecast(0.5, true)]), None);
}

#[test]
fn the_summary_counts_and_scores() {
    let list = vec![
        credence("t", 80).resolved(Outcome::True, "", now()),
        credence("f", 20).resolved(Outcome::False, "", now()),
        credence("a", 60).resolved(Outcome::Ambiguous, "", now()),
        credence("o", 40),
    ];
    let summary = summarize(&list);
    assert_eq!((summary.resolved, summary.open, summary.ambiguous), (2, 1, 1));
    assert!(close(summary.brier.unwrap_or_default(), 0.04));
    assert!(summary.log.is_some() && summary.murphy.is_some() && summary.bias.is_some());
    assert_eq!(summary.bins.len(), CALIBRATION_BINS);
    let empty = summarize(&[credence("o", 40)]);
    assert_eq!((empty.brier, empty.murphy, empty.bias), (None, None, None));
}
