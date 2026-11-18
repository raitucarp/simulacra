//! What the shell hands the Learn tab: the active network's goals (with their measured
//! progress and forecast), tutor sessions and palaces, the gap radar (overall and per goal),
//! the weekly digest, the vault's documents and the tutor's request status.

use crate::domain::learning::goals::GoalProgress;
use crate::domain::learning::weekly::WeeklyDigest;
use crate::domain::learning::{Forecast, GapReport, Goal, GoalId, LearningRecords, SessionId};
use std::collections::HashMap;

/// A goal as listed: the goal, its progress, its forecast and the share of its time spent.
#[derive(Debug, Clone, PartialEq)]
pub struct GoalView {
    pub goal: Goal,
    pub progress: GoalProgress,
    pub forecast: Forecast,
    pub expected: f64,
}

/// Where the tutor's request stands.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TutorStatus {
    #[default]
    Idle,
    /// A request for this session is on its way.
    Running(SessionId),
    /// The last request for this session failed (the message is localized).
    Failed(SessionId, String),
}

impl TutorStatus {
    pub fn is_running(&self, id: &SessionId) -> bool {
        matches!(self, Self::Running(running) if running == id)
    }

    pub fn failure(&self, id: &SessionId) -> Option<&str> {
        match self {
            Self::Failed(failed, message) if failed == id => Some(message),
            _ => None,
        }
    }
}

/// The Learn tab's data.
#[derive(Debug, Clone, Default)]
pub struct LearningData {
    /// The active network's records (sessions newest first).
    pub records: LearningRecords,
    pub goals: Vec<GoalView>,
    /// The radar over every concept.
    pub gaps: GapReport,
    /// Each goal's radar (its concepts only).
    pub goal_gaps: HashMap<GoalId, GapReport>,
    pub digest: WeeklyDigest,
    /// Vault documents (id, title), for the tutor.
    pub documents: Vec<(String, String)>,
    pub tutor: TutorStatus,
}

impl LearningData {
    pub fn goal(&self, id: &GoalId) -> Option<&GoalView> {
        self.goals.iter().find(|view| &view.goal.id == id)
    }

    /// The radar of `goal` (the overall one without a goal).
    pub fn radar(&self, goal: Option<&GoalId>) -> &GapReport {
        goal.and_then(|id| self.goal_gaps.get(id)).unwrap_or(&self.gaps)
    }
}
