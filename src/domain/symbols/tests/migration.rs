//! The monthly migration review (S16).

use crate::domain::symbols::*;
use jiff::civil::date;

#[test]
fn only_open_tasks_are_reviewed_and_it_is_due_once_a_month() {
    assert!(needs_review(&PostSymbols::entry(EntrySymbol::Task)));
    assert!(!needs_review(&PostSymbols::entry(EntrySymbol::Event)) && !needs_review(&PostSymbols::entry(EntrySymbol::Note)));
    assert!(!needs_review(&PostSymbols { state: Some(EntryState::Migrated), ..PostSymbols::entry(EntrySymbol::Task) }));
    assert!(review_due(None, date(2026, 11, 22)));
    assert!(!review_due(Some(date(2026, 11, 1)), date(2026, 11, 30)));
    assert!(review_due(Some(date(2026, 10, 31)), date(2026, 11, 1)));
    assert!(review_due(Some(date(2025, 12, 5)), date(2026, 1, 2)));
}

#[test]
fn the_walk_decides_each_task_once_and_counts_the_choices() {
    let mut review = MigrationReview::new(vec!["a", "b", "c", "d"]);
    assert_eq!(review.current(), Some(&"a"));
    for choice in [ReviewChoice::Migrate, ReviewChoice::Drop, ReviewChoice::Migrate, ReviewChoice::Keep] {
        assert!(review.decide(choice));
    }
    assert!(review.is_done() && !review.decide(ReviewChoice::Drop));
    assert_eq!(ReviewChoice::ALL.map(|choice| review.count(choice)), [2, 0, 1, 1]);
    assert_eq!(review.decided[1], ("b", ReviewChoice::Drop));
    assert_eq!(ReviewChoice::Schedule.state(), Some(EntryState::Scheduled));
    assert_eq!(ReviewChoice::Keep.state(), None);
    assert!(MigrationReview::<&str>::new(Vec::new()).is_done());
}
