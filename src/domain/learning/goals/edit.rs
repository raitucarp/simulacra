//! Edits of a goal as pure functions: a new goal, links added and removed, tasks added,
//! toggled and removed. Each returns the changed goal.

use super::{Goal, GoalId, GoalLink, GoalTask, TASK_ID_PREFIX};
use crate::domain::learning::fresh_id;
use crate::domain::NetworkId;
use chrono::{DateTime, NaiveDate, Utc};

/// What the learner typed for a new goal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoalDraft {
    pub title: String,
    pub why: String,
    pub target: NaiveDate,
}

/// Why a goal draft cannot be saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalProblem {
    MissingTitle,
    /// The target date is not after the day the goal starts.
    TargetInPast,
}

impl GoalProblem {
    /// Locale key of the message.
    pub const fn message_key(self) -> &'static str {
        match self {
            Self::MissingTitle => "learning.goals.problem.title",
            Self::TargetInPast => "learning.goals.problem.target",
        }
    }
}

/// The first problem of `draft` started `today`, if any.
pub fn goal_problem(draft: &GoalDraft, today: NaiveDate) -> Option<GoalProblem> {
    match (draft.title.trim().is_empty(), draft.target <= today) {
        (true, _) => Some(GoalProblem::MissingTitle),
        (false, true) => Some(GoalProblem::TargetInPast),
        _ => None,
    }
}

/// The new goal of `draft` (none while it has a problem).
pub fn new_goal(draft: &GoalDraft, network_id: NetworkId, id: GoalId, today: NaiveDate, now: DateTime<Utc>) -> Option<Goal> {
    goal_problem(draft, today).is_none().then(|| Goal {
        id,
        network_id,
        title: draft.title.trim().to_string(),
        why: draft.why.trim().to_string(),
        target: draft.target,
        links: Vec::new(),
        tasks: Vec::new(),
        completed: false,
        created_on: today,
        created_at: now,
    })
}

/// `goal` with `link` added (once).
pub fn with_link(goal: Goal, link: GoalLink) -> Goal {
    if goal.has_link(&link) {
        return goal;
    }
    let mut links = goal.links.clone();
    links.push(link);
    Goal { links, ..goal }
}

/// `goal` without `link`.
pub fn without_link(goal: Goal, link: &GoalLink) -> Goal {
    Goal { links: goal.links.iter().filter(|l| *l != link).cloned().collect(), ..goal }
}

/// `goal` with a task per text (blank texts and texts it already has are skipped).
pub fn with_tasks(goal: Goal, texts: &[String], today: NaiveDate) -> Goal {
    let mut tasks = goal.tasks.clone();
    for text in texts.iter().map(|t| t.trim()).filter(|t| !t.is_empty()) {
        if !tasks.iter().any(|task| !task.done && task.text == text) {
            tasks.push(GoalTask { id: fresh_id(TASK_ID_PREFIX), text: text.to_string(), done: false, added_on: today, done_on: None });
        }
    }
    Goal { tasks, ..goal }
}

/// `goal` with task `id` ticked or unticked on `today`.
pub fn toggled_task(goal: Goal, id: &str, today: NaiveDate) -> Goal {
    let tasks = goal.tasks.iter().map(|task| match task.id == id {
        true => GoalTask { done: !task.done, done_on: (!task.done).then_some(today), ..task.clone() },
        false => task.clone(),
    }).collect();
    Goal { tasks, ..goal }
}

/// `goal` without task `id`.
pub fn without_task(goal: Goal, id: &str) -> Goal {
    Goal { tasks: goal.tasks.iter().filter(|task| task.id != id).cloned().collect(), ..goal }
}
