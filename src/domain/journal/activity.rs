//! What the user did on each journal day, gathered by the shell from data it already has:
//! posts the user wrote, review sessions (answers more than a gap apart start a new session)
//! and focus minutes. Used only for the private associations on the Mood tab.

use chrono::{DateTime, NaiveDate, Utc};
use std::collections::BTreeMap;

/// Minutes between two answers after which a new review session starts.
pub const SESSION_GAP_MINUTES: i64 = 30;

/// One day's activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DayActivity {
    pub posts: usize,
    pub review_sessions: usize,
    pub focus_minutes: u32,
}

/// Activity by journal day.
pub type ActivityByDay = BTreeMap<NaiveDate, DayActivity>;

/// An activity measure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivityKind {
    PostsWritten,
    ReviewSessions,
    FocusMinutes,
}

impl ActivityKind {
    pub const ALL: [Self; 3] = [Self::PostsWritten, Self::ReviewSessions, Self::FocusMinutes];

    pub const fn key(self) -> &'static str {
        match self {
            Self::PostsWritten => "posts",
            Self::ReviewSessions => "reviews",
            Self::FocusMinutes => "focus",
        }
    }

    pub fn of(self, day: &DayActivity) -> f64 {
        match self {
            Self::PostsWritten => day.posts as f64,
            Self::ReviewSessions => day.review_sessions as f64,
            Self::FocusMinutes => f64::from(day.focus_minutes),
        }
    }
}

/// Review sessions per day from the answer times (each with its day): a session starts at
/// the first answer of a day and after every gap longer than [`SESSION_GAP_MINUTES`].
pub fn review_sessions_by_day(answers: &[(NaiveDate, DateTime<Utc>)]) -> BTreeMap<NaiveDate, usize> {
    let mut sorted = answers.to_vec();
    sorted.sort_by_key(|(_, at)| *at);
    let mut sessions: BTreeMap<NaiveDate, usize> = BTreeMap::new();
    let mut previous: Option<(NaiveDate, DateTime<Utc>)> = None;
    for (day, at) in sorted {
        let continues = previous.is_some_and(|(d, p)| d == day && (at - p).num_minutes() <= SESSION_GAP_MINUTES);
        if !continues {
            *sessions.entry(day).or_default() += 1;
        }
        previous = Some((day, at));
    }
    sessions
}

/// Merges per-day counts into one activity map.
pub fn activity_by_day(posts: &BTreeMap<NaiveDate, usize>, reviews: &BTreeMap<NaiveDate, usize>, focus: &BTreeMap<NaiveDate, u32>) -> ActivityByDay {
    let mut days = ActivityByDay::new();
    posts.iter().for_each(|(d, n)| days.entry(*d).or_default().posts = *n);
    reviews.iter().for_each(|(d, n)| days.entry(*d).or_default().review_sessions = *n);
    focus.iter().for_each(|(d, n)| days.entry(*d).or_default().focus_minutes = *n);
    days
}
