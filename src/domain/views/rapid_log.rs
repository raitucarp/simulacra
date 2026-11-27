//! The rapid log (W2): posts as short bullets grouped by the day they were written. A post
//! with symbols uses its primary symbol as its bullet and its signifiers as marks before it
//! (S4); any other post gets a plain bullet. Days read newest first; within a day the
//! entries read in the order they were written. A locked post keeps only its placeholder.

use crate::domain::symbols::{SymbolArt, SymbolGlyph};
use crate::domain::{Post, PostId};
use jiff::civil::Date;
use jiff::Timestamp;

/// One bullet of the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub id: PostId,
    pub at: Timestamp,
    /// The primary symbol; `None` draws a plain bullet.
    pub bullet: Option<SymbolGlyph>,
    /// Signifiers shown before the bullet.
    pub marks: Vec<SymbolArt>,
    /// The text on one line (empty for a locked post).
    pub text: String,
    /// The post is locked: only its placeholder is shown.
    pub locked: bool,
}

/// One day of the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogDay {
    pub day: Date,
    pub entries: Vec<LogEntry>,
}

/// `post`'s text on one line as a reader sees it, markup and post links left out (empty
/// when locked).
pub fn one_line(post: &Post) -> String {
    if post.is_locked() {
        return String::new();
    }
    crate::domain::post::readable_line(&post.content)
}

fn entry(post: &Post) -> LogEntry {
    let face = post.extras.symbols.face();
    LogEntry {
        id: post.id.clone(),
        at: post.created_at,
        bullet: face.as_ref().map(|face| face.primary.clone()),
        marks: face.map(|face| face.badges).unwrap_or_default(),
        text: one_line(post),
        locked: post.is_locked(),
    }
}

/// The log of `posts`, each placed on `day_of(written)` (the user's calendar day).
pub fn rapid_log<'a>(posts: impl IntoIterator<Item = &'a Post>, day_of: impl Fn(Timestamp) -> Date) -> Vec<LogDay> {
    let mut entries: Vec<(Date, LogEntry)> = posts.into_iter().map(|post| (day_of(post.created_at), entry(post))).collect();
    entries.sort_by(|(a_day, a), (b_day, b)| b_day.cmp(a_day).then(a.at.cmp(&b.at)).then_with(|| a.id.as_str().cmp(b.id.as_str())));
    let mut days: Vec<LogDay> = Vec::new();
    for (day, entry) in entries {
        match days.last_mut() {
            Some(last) if last.day == day => last.entries.push(entry),
            _ => days.push(LogDay { day, entries: vec![entry] }),
        }
    }
    days
}
