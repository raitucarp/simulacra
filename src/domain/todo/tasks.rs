//! The Tasks view across posts (D9): every to-do of the user's posts in one list, narrowed by
//! due day, priority and context.
//!
//! - Due: any, overdue (before today), today, this week (today and the six days after, and
//!   anything overdue), or no due day.
//! - Priority: at least this many `!`.
//! - Context: carries this `&context` (ignoring case).
//! - Done and dropped items are left out unless asked for.
//!
//! The list reads soonest first: overdue and dated items by due day, then undated ones; ties
//! go to the higher priority, then to the newer post, then to reading order.

use super::entries::{todo_entries, TodoEntry};
use crate::domain::{Post, PostId};
use jiff::civil::Date;
use jiff::ToSpan;
use std::cmp::Reverse;

/// The days after today "this week" reaches.
const WEEK_AHEAD: i64 = 6;

/// Which due days the view shows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum DueFilter {
    #[default]
    Any,
    Overdue,
    Today,
    Week,
    NoDate,
}

impl DueFilter {
    pub const ALL: [Self; 5] = [Self::Any, Self::Overdue, Self::Today, Self::Week, Self::NoDate];

    /// Stable key (locale keys `todo.tasks.due.<key>`, element ids).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Overdue => "overdue",
            Self::Today => "today",
            Self::Week => "week",
            Self::NoDate => "no_date",
        }
    }

    /// Whether an item due on `due` passes, on `today`.
    pub fn admits(self, due: Option<Date>, today: Date) -> bool {
        match (self, due) {
            (Self::Any, _) => true,
            (Self::NoDate, due) => due.is_none(),
            (_, None) => false,
            (Self::Overdue, Some(due)) => due < today,
            (Self::Today, Some(due)) => due == today,
            (Self::Week, Some(due)) => due <= today.saturating_add(WEEK_AHEAD.days()),
        }
    }
}

/// What the view is narrowed to.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskFilter {
    pub due: DueFilter,
    /// At least this priority (1 to 3).
    pub priority: Option<u8>,
    pub context: Option<String>,
    /// Done and dropped items are shown too.
    pub show_closed: bool,
}

impl TaskFilter {
    fn admits(&self, entry: &TodoEntry, due: Option<Date>, today: Date) -> bool {
        let priority = self.priority.is_none_or(|least| entry.item.meta.priority.is_some_and(|level| level >= least));
        let context = self.context.as_ref().is_none_or(|wanted| entry.item.meta.contexts.iter().any(|c| c.eq_ignore_ascii_case(wanted)));
        (self.show_closed || !entry.state.is_closed()) && priority && context && self.due.admits(due, today)
    }
}

/// One to-do in the view, with the post it lives in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRow {
    pub post_id: PostId,
    /// The day the post was written (due phrases count from it).
    pub written: Date,
    pub entry: TodoEntry,
    pub due: Option<Date>,
}

/// The rows of `posts` (newest first, each with the day it was written) that pass `filter`
/// on `today`, soonest first.
pub fn collect_tasks<'a>(posts: impl IntoIterator<Item = (&'a Post, Date)>, filter: &TaskFilter, today: Date) -> Vec<TaskRow> {
    let mut rows: Vec<(usize, TaskRow)> = Vec::new();
    for (order, (post, written)) in posts.into_iter().enumerate() {
        for entry in todo_entries(&post.content, &post.extras.todo_state) {
            let due = entry.due(written);
            if filter.admits(&entry, due, today) {
                rows.push((order, TaskRow { post_id: post.id.clone(), written, entry, due }));
            }
        }
    }
    rows.sort_by_key(|(order, row)| (row.due.is_none(), row.due, Reverse(row.entry.item.meta.priority), *order, row.entry.index));
    rows.into_iter().map(|(_, row)| row).collect()
}

/// Every context the to-dos of `posts` carry, in first-seen order without duplicates (ignoring case).
pub fn task_contexts<'a>(posts: impl IntoIterator<Item = &'a Post>) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for post in posts {
        for entry in todo_entries(&post.content, &post.extras.todo_state) {
            for context in entry.item.meta.contexts {
                if !seen.iter().any(|known| known.eq_ignore_ascii_case(&context)) {
                    seen.push(context);
                }
            }
        }
    }
    seen
}
