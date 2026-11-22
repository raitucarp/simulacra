//! A parent task is done only once its sub-tasks are settled (S8).

use crate::domain::symbols::*;

fn child(entry: EntrySymbol, state: Option<EntryState>) -> PostSymbols {
    PostSymbols { state, ..PostSymbols::entry(entry) }
}

#[test]
fn only_open_or_pending_tasks_and_events_hold_a_parent_back() {
    let settled = [child(EntrySymbol::Task, Some(EntryState::Done)), child(EntrySymbol::Event, Some(EntryState::Dropped)), PostSymbols::entry(EntrySymbol::Note), PostSymbols::default()];
    assert_eq!(unsettled_children(&settled), 0);
    assert!(may_settle(EntryState::Done, &settled));
    for state in [EntryState::Open, EntryState::Migrated, EntryState::Scheduled, EntryState::Delegated] {
        let pending = [child(EntrySymbol::Task, Some(state))];
        assert_eq!(unsettled_children(&pending), 1, "{state:?}");
        assert!(!may_settle(EntryState::Done, &pending), "{state:?}");
        assert!(may_settle(EntryState::Dropped, &pending) && may_settle(EntryState::Migrated, &pending), "only done waits");
    }
}
