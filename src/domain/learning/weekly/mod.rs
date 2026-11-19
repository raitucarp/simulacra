//! The weekly review: a guided look back over the last seven review days (what was
//! recalled, written and practised with the tutor), what comes due next, the weakest
//! concepts of the radar, and next actions the learner ticks to make them goal tasks.

pub mod actions;
pub mod reminder;

pub use actions::{suggest_actions, NextAction};
pub use reminder::{weekly_review_due, WeeklyReminder};

use super::concept::{concepts_of, marked_concepts};
use super::goals::OwnPost;
use super::tutor::TutorSession;
use crate::domain::review::day::add_days;
use crate::domain::review::{ItemId, ReviewEntry, ReviewItem};
use chrono::NaiveDate;
use std::collections::HashMap;

/// Days a week review looks back over (today included).
pub const WEEK_DAYS: u32 = 7;
/// Entries listed per section at most.
pub const LISTED: usize = 6;
/// Posts an unmarked concept needs to count among the week's concepts.
const REPEATED: usize = 2;

/// The seven review days ending today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekWindow {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl WeekWindow {
    pub fn ending(today: NaiveDate) -> Self {
        Self { start: today - chrono::Duration::days(i64::from(WEEK_DAYS - 1)), end: today }
    }

    pub fn contains(&self, day: NaiveDate) -> bool {
        day >= self.start && day <= self.end
    }
}

/// The look back and ahead of one weekly review.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WeeklyDigest {
    pub reviews: usize,
    /// Share of this week's answers that were recalled.
    pub recall_share: Option<f64>,
    /// Items reviewed for the first time.
    pub items_started: usize,
    /// Distinct items recalled, most recent first (at most [`LISTED`]).
    pub recalled: Vec<ItemId>,
    pub posts_written: usize,
    /// The week's own posts, newest first (at most [`LISTED`]).
    pub posts: Vec<String>,
    /// The week's most written-about concepts with their post counts.
    pub concepts: Vec<(String, usize)>,
    pub tutor_sessions: usize,
    pub due_today: usize,
    /// Active items due within the coming week (today included).
    pub due_week: usize,
}

/// What the digest is computed from.
#[derive(Debug, Clone, Copy)]
pub struct WeekInputs<'a> {
    pub items: &'a [ReviewItem],
    pub log: &'a [ReviewEntry],
    pub own_posts: &'a [OwnPost],
    pub sessions: &'a [TutorSession],
    pub today: NaiveDate,
}

fn recalled_items(log: &[&ReviewEntry]) -> Vec<ItemId> {
    let mut recent: Vec<&&ReviewEntry> = log.iter().filter(|entry| entry.grade.is_recall()).collect();
    recent.sort_by_key(|entry| std::cmp::Reverse(entry.reviewed_at));
    recent.iter().map(|entry| entry.item_id.clone()).fold(Vec::new(), |mut ids, id| {
        if !ids.contains(&id) && ids.len() < LISTED {
            ids.push(id);
        }
        ids
    })
}

/// The week's concepts: the marked ones, and others written about at least twice.
fn top_concepts(posts: &[&OwnPost]) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    posts.iter().flat_map(|post| concepts_of(&post.text)).for_each(|key| *counts.entry(key).or_default() += 1);
    let marked: Vec<String> = posts.iter().flat_map(|post| marked_concepts(&post.text)).collect();
    let mut ranked: Vec<(String, usize)> = counts.into_iter().filter(|(key, count)| *count >= REPEATED || marked.contains(key)).collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ranked.truncate(LISTED);
    ranked
}

/// The digest of the week ending `inputs.today`.
pub fn weekly_digest(inputs: &WeekInputs<'_>) -> WeeklyDigest {
    let window = WeekWindow::ending(inputs.today);
    let log: Vec<&ReviewEntry> = inputs.log.iter().filter(|entry| window.contains(entry.day)).collect();
    let mut posts: Vec<&OwnPost> = inputs.own_posts.iter().filter(|post| window.contains(post.day)).collect();
    posts.sort_by_key(|a| std::cmp::Reverse(a.day));
    let horizon = add_days(inputs.today, WEEK_DAYS - 1);
    let active = || inputs.items.iter().filter(|item| item.is_active() && !item.schedule.is_new());
    WeeklyDigest {
        reviews: log.len(),
        recall_share: (!log.is_empty()).then(|| log.iter().filter(|e| e.grade.is_recall()).count() as f64 / log.len() as f64),
        items_started: log.iter().filter(|entry| entry.phase_before == crate::domain::review::Phase::New).count(),
        recalled: recalled_items(&log),
        posts_written: posts.len(),
        posts: posts.iter().take(LISTED).map(|post| post.id.clone()).collect(),
        concepts: top_concepts(&posts),
        tutor_sessions: inputs.sessions.iter().filter(|s| window.contains(s.updated_at.date_naive())).count(),
        due_today: active().filter(|item| item.schedule.due <= inputs.today).count(),
        due_week: active().filter(|item| item.schedule.due <= horizon).count(),
    }
}
