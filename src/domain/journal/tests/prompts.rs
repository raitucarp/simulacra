use super::day;
use crate::domain::journal::prompts::{catalogue, prompt_for_day, PromptKind, PROMPT_COUNT};
use std::collections::HashSet;

#[test]
fn a_day_always_gets_the_same_prompt() {
    (0..40).for_each(|offset| assert_eq!(prompt_for_day(day(offset)), prompt_for_day(day(offset))));
}

#[test]
fn every_prompt_comes_once_per_cycle() {
    let start = (0..PROMPT_COUNT as i64).find(|o| (day(*o).num_days_from_ce_value()) % PROMPT_COUNT as i64 == 0).unwrap();
    let cycle: HashSet<_> = (start..start + PROMPT_COUNT as i64).map(|o| prompt_for_day(day(o))).collect();
    assert_eq!(cycle.len(), PROMPT_COUNT, "a cycle shows the whole catalogue");
}

#[test]
fn two_days_in_a_row_never_share_a_prompt() {
    let repeats: Vec<i64> = (0..3_000).filter(|o| prompt_for_day(day(*o)) == prompt_for_day(day(o + 1))).collect();
    assert!(repeats.is_empty(), "repeated on day offsets {repeats:?}");
}

#[test]
fn the_rotation_is_not_catalogue_order() {
    let days: Vec<_> = (0..PROMPT_COUNT as i64).map(|o| prompt_for_day(day(o))).collect();
    let in_order = days.windows(2).all(|w| w[0].next() == w[1]);
    assert!(!in_order, "prompts are shuffled, not walked in order");
}

#[test]
fn catalogue_covers_every_kind_and_next_cycles() {
    let kinds: HashSet<PromptKind> = catalogue().iter().map(|p| p.kind).collect();
    assert_eq!(kinds.len(), PromptKind::ALL.len());
    let first = catalogue()[0];
    let around = (0..PROMPT_COUNT).fold(first, |p, _| p.next());
    assert_eq!(around, first, "next walks the whole catalogue and wraps");
    assert_eq!(first.text_key(), "journal.prompt.expressive_1");
    assert_eq!(catalogue()[PROMPT_COUNT - 1].text_key(), "journal.prompt.best_self_2");
}

/// The day number used by the rotation.
trait DayNumber {
    fn num_days_from_ce_value(&self) -> i64;
}

impl DayNumber for jiff::civil::Date {
    fn num_days_from_ce_value(&self) -> i64 {
        crate::time::days_from_ce(*self)
    }
}
