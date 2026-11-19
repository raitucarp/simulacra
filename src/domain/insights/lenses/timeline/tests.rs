use super::lifecycle::phase;
use super::*;

fn day(offset: i64) -> Date {
    jiff::civil::date(2026, 1, 1) + crate::time::days(offset)
}

fn source(offset: i64, concepts: &[&str]) -> Source {
    Source { kind: SourceKind::Post, id: format!("p{offset}-{}", concepts.join("-")), day: day(offset), excerpt: String::new(), concepts: concepts.iter().map(|c| c.to_string()).collect() }
}

/// Forty days: "ethics" only in the first week, "logic" every few days and more at the end,
/// "memory" only in the last days, "once" a single time.
fn sources() -> Vec<Source> {
    let mut all = vec![source(0, &["ethics", "logic"]), source(2, &["ethics"]), source(5, &["ethics", "logic"]), source(9, &["once"])];
    all.extend((10..30).step_by(5).map(|d| source(d, &["logic"])));
    all.extend((34..40).map(|d| source(d, &["logic", "memory"])));
    all.sort_by_key(|s| s.day);
    all
}

#[test]
fn lanes_keep_concepts_mentioned_twice_busiest_first() {
    let timeline = ConceptTimeline::of(sources(), day(39));
    let keys: Vec<&str> = timeline.lanes.iter().map(|l| l.key.as_str()).collect();
    assert_eq!(keys, ["logic", "memory", "ethics"], "'once' has a single mention");
    assert_eq!(timeline.periods.count, 40, "one period per day while the span fits");
    let logic = &timeline.lanes[0];
    assert_eq!((logic.total, logic.first_day, logic.last_day), (12, day(0), day(39)));
    assert_eq!(logic.counts.iter().sum::<u32>(), logic.total);
}

#[test]
fn phases_tell_where_each_concept_stands() {
    let timeline = ConceptTimeline::of(sources(), day(39));
    let phase_of = |key: &str| timeline.lanes.iter().find(|l| l.key == key).map(|l| l.phase);
    assert_eq!(phase_of("ethics"), Some(Phase::Dormant));
    assert_eq!(phase_of("memory"), Some(Phase::New));
    assert_eq!(phase_of("logic"), Some(Phase::Rising));
}

#[test]
fn lifecycle_rules() {
    assert_eq!(phase(&[1, 1, 1, 1, 1, 1, 1, 1], 0), Phase::Steady);
    assert_eq!(phase(&[4, 4, 3, 2, 1, 1, 1, 0], 1), Phase::Declining);
    assert_eq!(phase(&[1, 0, 1, 0, 1, 0, 3, 3], 0), Phase::Rising);
    assert_eq!(phase(&[0, 0, 0, 0, 0, 0, 1, 1], 0), Phase::New);
    assert_eq!(phase(&[2, 2, 0, 0, 0, 0, 0, 0], 6), Phase::Dormant);
}

#[test]
fn long_spans_group_days_into_periods() {
    let timeline = ConceptTimeline::of(vec![source(0, &["a"]), source(400, &["a"])], day(400));
    assert!(timeline.periods.count <= MAX_PERIODS);
    assert_eq!(timeline.periods.index_of(day(400)), Some(timeline.periods.count - 1));
    assert_eq!(timeline.lanes[0].counts.iter().sum::<u32>(), 2);
}

#[test]
fn a_zoomed_range_lists_mentions_and_neighbours() {
    let timeline = ConceptTimeline::of(sources(), day(39));
    let logic = timeline.lanes.iter().position(|l| l.key == "logic").unwrap();
    let memory = timeline.lanes.iter().position(|l| l.key == "memory").unwrap();
    let ethics = timeline.lanes.iter().position(|l| l.key == "ethics").unwrap();
    let late = (34, 39);
    assert_eq!(timeline.mentions_of(logic, late).len(), 6);
    assert!(timeline.mentions_of(logic, late).windows(2).all(|w| w[0].day <= w[1].day));
    assert_eq!(timeline.active_lanes(late, 10), vec![logic, memory]);
    assert_eq!(timeline.total_in(ethics, late), 0);
    let early = timeline.co_occurring(logic, (0, 9), 3);
    assert_eq!(early.iter().map(|p| p.period).collect::<Vec<_>>(), vec![0, 5]);
    assert_eq!(early[0].with, vec![(ethics, 1)]);
    assert_eq!(timeline.clamp((50, 3)), (3, 39));
}

#[test]
fn nothing_to_draw_without_sources() {
    assert!(ConceptTimeline::of(Vec::new(), day(0)).is_empty());
}
