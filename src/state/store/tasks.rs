//! The Tasks view's source (D9): the to-dos of the user's own posts in the active network
//! (agents' checklists are theirs, not the user's).

use super::AppState;
use crate::domain::todo::tasks::{collect_tasks, task_contexts, TaskFilter, TaskRow};
use crate::domain::Post;
use jiff::civil::Date;

impl AppState {
    /// The user's posts in the active network, newest first.
    fn own_posts(&self) -> impl Iterator<Item = &Post> {
        self.current_network_posts().filter(|post| self.author_of(post).is_human_persona())
    }

    /// The to-dos that pass `filter` on `today`, soonest first.
    pub fn task_rows(&self, filter: &TaskFilter, today: Date) -> Vec<TaskRow> {
        collect_tasks(self.own_posts().map(|post| (post, crate::time::local_date(post.created_at))), filter, today)
    }

    /// Every context the user's to-dos carry.
    pub fn task_contexts(&self) -> Vec<String> {
        task_contexts(self.own_posts())
    }
}
