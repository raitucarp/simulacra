use crate::domain::review::fsrs::*;
use crate::domain::review::Grade;

fn params() -> Params {
    Params::default()
}

#[test]
fn stability_is_the_ninety_percent_point_of_the_curve() {
    let p = params();
    for stability in [0.5, 3.0, 40.0, 900.0] {
        let r = retrievability(&p, stability, stability);
        assert!((r - STABILITY_RETENTION).abs() < 1e-9, "R(S, S) = {r}");
        assert!((interval_days(&p, stability, STABILITY_RETENTION) - stability).abs() < 1e-9);
    }
    assert_eq!(retrievability(&p, 0.0, 5.0), 1.0, "nothing is forgotten at once");
    assert!(retrievability(&p, 30.0, 5.0) < retrievability(&p, 10.0, 5.0), "the curve falls");
}

#[test]
fn a_lower_retention_target_gives_longer_intervals() {
    let p = params();
    let strict = interval_days(&p, 10.0, 0.95);
    let normal = interval_days(&p, 10.0, 0.90);
    let relaxed = interval_days(&p, 10.0, 0.80);
    assert!(strict < normal && normal < relaxed, "{strict} < {normal} < {relaxed}");
}

#[test]
fn first_review_weights_and_difficulty_order() {
    let p = params();
    let stabilities = Grade::ALL.map(|g| initial_stability(&p, g));
    assert_eq!(stabilities, [DEFAULT_WEIGHTS[0], DEFAULT_WEIGHTS[1], DEFAULT_WEIGHTS[2], DEFAULT_WEIGHTS[3]]);
    let difficulties = Grade::ALL.map(|g| initial_difficulty(&p, g));
    assert!(difficulties.windows(2).all(|w| w[0] > w[1]), "Again is hardest: {difficulties:?}");
    assert!(difficulties.iter().all(|d| (MIN_DIFFICULTY..=MAX_DIFFICULTY).contains(d)));
    assert!((difficulties[0] - DEFAULT_WEIGHTS[4]).abs() < 1e-9, "D0(Again) = w4");
}

#[test]
fn difficulty_moves_with_the_grade_and_stays_in_bounds() {
    let p = params();
    assert!(next_difficulty(&p, 5.0, Grade::Again) > 5.0);
    assert!(next_difficulty(&p, 5.0, Grade::Easy) < 5.0);
    let hardest = next_difficulty(&p, MAX_DIFFICULTY, Grade::Again);
    assert!(hardest <= MAX_DIFFICULTY && hardest > MAX_DIFFICULTY - 0.1, "only mean reversion moves the hardest card: {hardest}");
    assert!(next_difficulty(&p, MIN_DIFFICULTY, Grade::Easy) >= MIN_DIFFICULTY);
}

#[test]
fn recall_grows_stability_and_lapses_shrink_it() {
    let p = params();
    let (d, s, r) = (5.0, 10.0, 0.9);
    let hard = recall_stability(&p, d, s, r, Grade::Hard);
    let good = recall_stability(&p, d, s, r, Grade::Good);
    let easy = recall_stability(&p, d, s, r, Grade::Easy);
    assert!(s < hard && hard < good && good < easy, "{s} < {hard} < {good} < {easy}");
    let lapse = lapse_stability(&p, d, s, r);
    assert!(lapse < s, "a lapse never raises stability: {lapse}");
    assert!(recall_stability(&p, d, s, 0.7, Grade::Good) > good, "harder retrievals strengthen more (spacing effect)");
    assert!(recall_stability(&p, 9.0, s, r, Grade::Good) < good, "difficult cards grow more slowly");
}

#[test]
fn same_day_reviews_never_shrink_on_success() {
    let p = params();
    assert!(same_day_stability(&p, 2.0, Grade::Good) >= 2.0);
    assert!(same_day_stability(&p, 2.0, Grade::Easy) >= same_day_stability(&p, 2.0, Grade::Good));
    assert!(same_day_stability(&p, 2.0, Grade::Again) < 2.0);
}
