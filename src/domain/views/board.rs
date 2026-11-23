//! The board (W6): to-dos in columns by the state they are in (D6), with task posts (S1)
//! beside them. Every state has a column, in a fixed order, so an empty column still shows
//! where things would go. Within a column, cards follow the posts' order and then reading
//! order. A locked post brings no to-dos (its text is sealed); a locked task post shows
//! only its placeholder.

use super::rapid_log::one_line;
use crate::domain::symbols::{EntryState, EntrySymbol};
use crate::domain::todo::entries::todo_entries;
use crate::domain::{Post, PostId};
use jiff::civil::Date;

/// The columns, left to right.
pub const BOARD_STATES: [EntryState; 6] = [EntryState::Open, EntryState::Scheduled, EntryState::Delegated, EntryState::Migrated, EntryState::Done, EntryState::Dropped];

/// One card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardCard {
    pub post_id: PostId,
    /// The to-do's place among the post's to-dos; `None` for a task post itself.
    pub todo: Option<usize>,
    pub text: String,
    pub due: Option<Date>,
    pub priority: Option<u8>,
    pub locked: bool,
}

/// One column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardColumn {
    pub state: EntryState,
    pub cards: Vec<BoardCard>,
}

fn task_card(post: &Post) -> Option<(EntryState, BoardCard)> {
    let symbols = &post.extras.symbols;
    let state = symbols.state.filter(|_| symbols.entry == Some(EntrySymbol::Task))?;
    Some((state, BoardCard { post_id: post.id.clone(), todo: None, text: one_line(post), due: None, priority: None, locked: post.is_locked() }))
}

fn todo_cards(post: &Post, written: Date) -> Vec<(EntryState, BoardCard)> {
    if post.is_locked() {
        return Vec::new();
    }
    todo_entries(&post.content, &post.extras.todo_state)
        .into_iter()
        .map(|entry| {
            let card = BoardCard { post_id: post.id.clone(), todo: Some(entry.index), due: entry.due(written), priority: entry.item.meta.priority, text: entry.item.text, locked: false };
            (entry.state, card)
        })
        .collect()
}

/// The board of `posts`, each with the day it was written.
pub fn board<'a>(posts: impl IntoIterator<Item = (&'a Post, Date)>) -> Vec<BoardColumn> {
    let mut columns: Vec<BoardColumn> = BOARD_STATES.iter().map(|state| BoardColumn { state: *state, cards: Vec::new() }).collect();
    for (post, written) in posts {
        for (state, card) in task_card(post).into_iter().chain(todo_cards(post, written)) {
            if let Some(column) = columns.iter_mut().find(|column| column.state == state) {
                column.cards.push(card);
            }
        }
    }
    columns
}

/// How many cards the board holds.
pub fn card_count(columns: &[BoardColumn]) -> usize {
    columns.iter().map(|column| column.cards.len()).sum()
}
