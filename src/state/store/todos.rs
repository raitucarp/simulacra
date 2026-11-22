//! Ticking the to-dos of a post (D6): the user's own posts only; a parent is done only once
//! its children are done or dropped (D7, [`crate::domain::todo::tick`]). A tick is kept in
//! `extras.todo_state` and saved with the post as it is, so it never makes a new version; an
//! edit of the text carries the ticks over to the items they belong to (see
//! [`crate::domain::todo::state`]).

use super::{require_persona, AppState};
use crate::domain::todo::entries::{todo_entries, TodoEntry};
use crate::domain::todo::tick::{apply_tick, TickRefusal};
use crate::domain::symbols::{can_transition, EntryState, EntrySymbol};
use crate::domain::PostId;
use crate::error::{AppError, Result};

/// The state a click on a box in `state` moves it to: done, or open again once done (or
/// dropped, which may only be reopened).
pub fn toggled(state: EntryState) -> EntryState {
    if state != EntryState::Done && can_transition(EntrySymbol::Task, state, EntryState::Done) { EntryState::Done } else { EntryState::Open }
}

fn missing(id: &PostId) -> AppError {
    AppError::DatabaseError(format!("post '{id}' not found"))
}

impl AppState {
    /// The to-dos of post `id` as they stand now (ticks applied); empty for an unknown post.
    pub fn post_todos(&self, id: &PostId) -> Vec<TodoEntry> {
        self.get_post(id).map(|post| todo_entries(&post.content, &post.extras.todo_state)).unwrap_or_default()
    }

    /// Clicks the box of the `index`-th to-do of the user's own post `id`; the state it is in now.
    pub fn toggle_todo(&mut self, id: &PostId, index: usize) -> Result<EntryState> {
        let entry = self.post_todos(id).into_iter().nth(index).ok_or_else(|| missing(id))?;
        let to = toggled(entry.state);
        self.set_todo_state(id, index, to)?;
        Ok(to)
    }

    /// Moves the `index`-th to-do of the user's own post `id` to `to`, without a new version.
    pub fn set_todo_state(&mut self, id: &PostId, index: usize, to: EntryState) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| missing(id))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let entries = self.post_todos(id);
        entries.get(index).ok_or_else(|| missing(id))?;
        let post = &mut self.posts[position];
        let mut state = apply_tick(&entries, &post.extras.todo_state, index, to).map_err(|TickRefusal::OpenChildren(open)| AppError::OpenSubtasks(open))?; // D7
        state.tidy(&post.content);
        if state == post.extras.todo_state {
            return Ok(());
        }
        post.extras.todo_state = state;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(())
    }
}
