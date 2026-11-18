//! Progress towards a goal, from what is linked to it.
//!
//! - **Linked review items:** items named by an item link, items anchored to a linked post,
//!   thread or document, and items tagged with (or asking about) a goal concept. Their
//!   retention is the mean FSRS retrievability today; their memory is the mean stability as
//!   a share of [`MATURE_STABILITY_DAYS`].
//! - **Practice:** the learner's posts on the goal since it started, and tutor sessions on
//!   its material, with the best Bloom level they reached.
//! - **Tasks:** done out of all.
//!
//! The score (0–1) weighs memory [`MEMORY_WEIGHT`], tasks [`TASK_WEIGHT`] and practice
//! [`PRACTICE_WEIGHT`]; a part with nothing to measure (no items, no tasks) is left out and
//! the others share its weight.

use super::{Goal, GoalLink};
use crate::domain::learning::bloom::BloomLevel;
use crate::domain::learning::concept::mentions;
use crate::domain::learning::tutor::TutorSession;
use crate::domain::review::fsrs::Params;
use crate::domain::review::{Anchor, ReviewItem, Subject};
use chrono::NaiveDate;

/// Stability (days) at which an item counts as learnt for good.
pub const MATURE_STABILITY_DAYS: f64 = 21.0;
/// Posts plus tutor sessions that make practice complete.
pub const PRACTICE_TARGET: f64 = 6.0;
pub const MEMORY_WEIGHT: f64 = 0.4;
pub const TASK_WEIGHT: f64 = 0.3;
pub const PRACTICE_WEIGHT: f64 = 0.3;
/// Within practice: the share of the activity count versus the best level reached.
const ACTIVITY_SHARE: f64 = 0.5;

/// A post the learner wrote (as themselves or a persona).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnPost {
    pub id: String,
    /// The thread's first post (itself for a first post).
    pub root_id: String,
    pub text: String,
    pub day: NaiveDate,
}

/// What progress is measured from.
#[derive(Debug, Clone, Copy)]
pub struct GoalInputs<'a> {
    pub items: &'a [ReviewItem],
    pub own_posts: &'a [OwnPost],
    pub sessions: &'a [TutorSession],
    pub today: NaiveDate,
    pub params: &'a Params,
}

/// A goal's measured progress.
#[derive(Debug, Clone, PartialEq)]
pub struct GoalProgress {
    pub items: usize,
    /// Mean retrievability today of the items with a memory.
    pub retention: Option<f64>,
    /// Mean stability as a share of the mature stability (0–1).
    pub memory: Option<f64>,
    pub new_posts: usize,
    pub sessions: usize,
    pub best_level: Option<BloomLevel>,
    pub tasks_done: usize,
    pub tasks_total: usize,
    /// Weighted progress, 0–1.
    pub score: f64,
}

fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// Whether `item` belongs to `goal` (see the module notes).
pub fn is_linked_item(goal: &Goal, item: &ReviewItem) -> bool {
    let concepts = goal.concepts();
    let by_concept = |text: &str| concepts.iter().any(|key| mentions(text, key));
    let anchored = match &item.subject {
        Subject::Anchored { anchor, written, .. } => anchor_linked(goal, anchor) || by_concept(written),
        Subject::Connections { map } => by_concept(&map.title),
    };
    anchored || goal.has_link(&GoalLink::Item { id: item.id.clone() }) || item.tags.iter().any(|tag| by_concept(tag))
}

fn anchor_linked(goal: &Goal, anchor: &Anchor) -> bool {
    goal.links.iter().any(|link| match (link, anchor) {
        (GoalLink::Post { id } | GoalLink::Thread { root_id: id }, Anchor::Post { id: post }) => id == post,
        (GoalLink::Document { id }, Anchor::Passage { document_id, .. }) => id == document_id,
        _ => false,
    })
}

/// Whether the learner's `post` is on the goal (linked, in a linked thread, or on a concept).
pub fn is_goal_post(goal: &Goal, post: &OwnPost) -> bool {
    let concepts = goal.concepts();
    goal.links.iter().any(|link| matches!(link, GoalLink::Post { id } if *id == post.id) || matches!(link, GoalLink::Thread { root_id } if *root_id == post.root_id))
        || concepts.iter().any(|key| mentions(&post.text, key))
}

/// Whether a tutor `session` worked on the goal's material.
pub fn is_goal_session(goal: &Goal, session: &TutorSession) -> bool {
    let on_post = session.source.post_id().is_some_and(|id| goal.links.iter().any(|l| matches!(l, GoalLink::Post { id: p } | GoalLink::Thread { root_id: p } if p == id)));
    let on_document = session.source.document_id().is_some_and(|id| goal.links.iter().any(|l| matches!(l, GoalLink::Document { id: d } if d == id)));
    on_post || on_document || goal.concepts().iter().any(|key| mentions(&session.title, key))
}

/// The measured progress of `goal`.
pub fn goal_progress(goal: &Goal, inputs: &GoalInputs<'_>) -> GoalProgress {
    let items: Vec<&ReviewItem> = inputs.items.iter().filter(|item| !item.retired && is_linked_item(goal, item)).collect();
    let recall: Vec<f64> = items.iter().filter_map(|item| item.schedule.retrievability(inputs.today, inputs.params)).collect();
    let stability: Vec<f64> = items.iter().map(|item| item.schedule.memory.map_or(0.0, |m| (m.stability / MATURE_STABILITY_DAYS).min(1.0))).collect();
    let new_posts = inputs.own_posts.iter().filter(|post| post.day >= goal.created_on && is_goal_post(goal, post)).count();
    let sessions: Vec<&TutorSession> = inputs.sessions.iter().filter(|session| is_goal_session(goal, session)).collect();
    let best_level = sessions.iter().filter_map(|session| session.mastery()).max();
    let tasks_done = goal.tasks.iter().filter(|task| task.done).count();
    let mut progress = GoalProgress {
        items: items.len(),
        retention: mean(&recall),
        memory: mean(&stability),
        new_posts,
        sessions: sessions.len(),
        best_level,
        tasks_done,
        tasks_total: goal.tasks.len(),
        score: 0.0,
    };
    progress.score = score(&progress);
    progress
}

/// The weighted score of `progress` (see the module notes).
pub fn score(progress: &GoalProgress) -> f64 {
    let activity = ((progress.new_posts + progress.sessions) as f64 / PRACTICE_TARGET).min(1.0);
    let practice = ACTIVITY_SHARE * activity + (1.0 - ACTIVITY_SHARE) * progress.best_level.map_or(0.0, BloomLevel::share);
    let tasks = (progress.tasks_total > 0).then(|| progress.tasks_done as f64 / progress.tasks_total as f64);
    let parts = [(progress.memory, MEMORY_WEIGHT), (tasks, TASK_WEIGHT), (Some(practice), PRACTICE_WEIGHT)];
    let weight: f64 = parts.iter().filter(|(value, _)| value.is_some()).map(|(_, w)| w).sum();
    let total: f64 = parts.iter().filter_map(|(value, w)| value.map(|v| v * w)).sum();
    if weight > 0.0 { (total / weight).clamp(0.0, 1.0) } else { 0.0 }
}
