//! Due reminders (D10): on every lifecycle heartbeat, each of the user's to-dos that is due
//! today or overdue and still to do is reminded of once per due day (a repeating item is
//! reminded of again on its next day). What was reminded is kept in the item's tick, so a
//! restart does not repeat it; it makes no version.

use super::AppState;
use crate::domain::todo::entries::todo_entries;
use crate::domain::PostId;
use crate::error::Result;
use jiff::civil::Date;

/// One to-do to remind the user of.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueReminder {
    pub post_id: PostId,
    /// The item's place among the post's to-dos.
    pub index: usize,
    pub text: String,
    pub due: Date,
}

impl DueReminder {
    /// Whether the day has already gone by on `today`.
    pub fn is_overdue(&self, today: Date) -> bool {
        self.due < today
    }
}

impl AppState {
    /// The user's to-dos due on or before `today` not yet reminded of for that day, marked as
    /// reminded (and saved) as they are returned.
    pub fn remind_due(&mut self, today: Date) -> Result<Vec<DueReminder>> {
        let mut reminders = Vec::new();
        let own: Vec<usize> = (0..self.posts.len()).filter(|at| self.author_of(&self.posts[*at]).is_human_persona()).collect();
        for at in own {
            let found = self.remind_in(at, today);
            if found.is_empty() {
                continue;
            }
            if let Some(ref db) = self.db {
                db.save_post(&self.posts[at])?;
            }
            reminders.extend(found);
        }
        Ok(reminders)
    }

    /// The reminders of the post at `at`, marking them in its ticks.
    fn remind_in(&mut self, at: usize, today: Date) -> Vec<DueReminder> {
        let post = &mut self.posts[at];
        let written = crate::time::local_date(post.created_at);
        let mut found = Vec::new();
        for entry in todo_entries(&post.content, &post.extras.todo_state) {
            let Some(due) = entry.due(written).filter(|due| *due <= today && !entry.state.is_closed()) else { continue };
            let tick = post.extras.todo_state.tick_mut(entry.index, &entry.item);
            if tick.reminded == Some(due) {
                continue;
            }
            tick.reminded = Some(due);
            found.push(DueReminder { post_id: post.id.clone(), index: entry.index, text: entry.item.text.clone(), due });
        }
        found
    }
}
