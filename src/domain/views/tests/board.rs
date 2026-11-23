use super::{date, locked, post_at};
use crate::domain::symbols::{EntryState, EntrySymbol, PostSymbols};
use crate::domain::views::board::{board, card_count, BOARD_STATES};

fn written(post: &crate::domain::Post) -> (&crate::domain::Post, jiff::civil::Date) {
    (post, date("2026-11-01"))
}

#[test]
fn every_state_has_a_column_and_todos_land_in_theirs() {
    let mut list = post_at("l", "[ ] buy milk !! ^2026-11-05\n[x] pay rent\n[~] old idea\n[/] ask @sam", "2026-11-01T09:00:00Z");
    list.extras.todo_state.tick_mut(0, &crate::domain::todo::parse_todos(&list.content)[0]).state = Some(EntryState::Migrated);
    let columns = board([written(&list)]);
    assert_eq!(columns.iter().map(|column| column.state).collect::<Vec<_>>(), BOARD_STATES);
    let texts = |state: EntryState| columns.iter().find(|c| c.state == state).expect("column").cards.iter().map(|card| card.text.clone()).collect::<Vec<_>>();
    assert_eq!(texts(EntryState::Migrated), ["buy milk"], "a tick moves the card");
    assert_eq!(texts(EntryState::Done), ["pay rent"]);
    assert_eq!(texts(EntryState::Dropped), ["old idea"]);
    assert_eq!(texts(EntryState::Delegated), ["ask"]);
    let milk = &columns[3].cards[0];
    assert_eq!((milk.priority, milk.due, milk.todo), (Some(2), Some(date("2026-11-05")), Some(0)));
    assert_eq!(card_count(&columns), 4);
}

#[test]
fn a_task_post_is_a_card_and_a_locked_one_shows_only_its_placeholder() {
    let mut task = post_at("t", "call the bank", "2026-11-01T09:00:00Z");
    task.extras.symbols = PostSymbols::entry(EntrySymbol::Task);
    let mut sealed = locked(post_at("s", "secret [ ] hidden todo", "2026-11-01T10:00:00Z"));
    sealed.extras.symbols = PostSymbols::entry(EntrySymbol::Task);
    let columns = board([written(&task), written(&sealed)]);
    let open = &columns[0].cards;
    assert_eq!(open.len(), 2, "the locked post brings no to-dos, only itself");
    assert_eq!((open[0].text.as_str(), open[0].todo), ("call the bank", None));
    assert!(open[1].locked && open[1].text.is_empty());
}
