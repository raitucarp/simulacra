//! The future log (W5): what lies ahead, by month: scheduled posts on the day they are due
//! to be published (Y1) and to-dos on their due day (D3), from today on. Done and dropped
//! to-dos are left out; months read soonest first, and within a month by day (posts before
//! to-dos on the same day). A locked scheduled post keeps only its placeholder.

use super::calendar::YearMonth;
use super::rapid_log::one_line;
use crate::domain::symbols::EntryState;
use crate::domain::todo::entries::todo_entries;
use crate::domain::{Post, PostId};
use jiff::civil::Date;
use jiff::Timestamp;

/// What one line of the future log is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FutureItem {
    /// A scheduled post (empty text when locked).
    Post { locked: bool },
    /// The `index`-th to-do of the post, in its state.
    Todo { index: usize, state: EntryState },
}

/// One line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureEntry {
    pub day: Date,
    pub post_id: PostId,
    pub text: String,
    pub item: FutureItem,
}

/// One month of lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureMonth {
    pub month: YearMonth,
    pub entries: Vec<FutureEntry>,
}

fn scheduled_entry(post: &Post, day_of: &impl Fn(Timestamp) -> Date) -> Option<FutureEntry> {
    let at = post.lifecycle.publish_at.filter(|_| post.lifecycle.is_scheduled())?;
    Some(FutureEntry { day: day_of(at), post_id: post.id.clone(), text: one_line(post), item: FutureItem::Post { locked: post.is_locked() } })
}

fn todo_lines(post: &Post, written: Date) -> impl Iterator<Item = FutureEntry> + '_ {
    todo_entries(&post.content, &post.extras.todo_state).into_iter().filter(|entry| !entry.state.is_closed()).filter_map(move |entry| {
        let day = entry.due(written)?;
        Some(FutureEntry { day, post_id: post.id.clone(), text: entry.item.text.clone(), item: FutureItem::Todo { index: entry.index, state: entry.state } })
    })
}

/// The future log of `posts` from `today` on; `day_of` gives the user's calendar day.
pub fn future_log<'a>(posts: impl IntoIterator<Item = &'a Post>, today: Date, day_of: impl Fn(Timestamp) -> Date) -> Vec<FutureMonth> {
    let mut lines: Vec<FutureEntry> = Vec::new();
    for post in posts {
        lines.extend(scheduled_entry(post, &day_of));
        if !post.is_locked() {
            lines.extend(todo_lines(post, day_of(post.created_at)));
        }
    }
    lines.retain(|line| line.day >= today);
    lines.sort_by_key(|line| (line.day, matches!(line.item, FutureItem::Todo { .. })));
    let mut months: Vec<FutureMonth> = Vec::new();
    for line in lines {
        let month = YearMonth::of(line.day);
        match months.last_mut() {
            Some(last) if last.month == month => last.entries.push(line),
            _ => months.push(FutureMonth { month, entries: vec![line] }),
        }
    }
    months
}
