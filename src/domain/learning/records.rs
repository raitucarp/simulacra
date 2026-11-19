//! Every goal, tutor session and palace (all networks), as loaded from the vault, with pure
//! replace and remove operations.

use super::goals::{Goal, GoalId};
use super::palace::{Palace, PalaceId};
use super::tutor::{SessionId, TutorSession};
use crate::domain::NetworkId;

/// The learning records of the vault.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LearningRecords {
    pub goals: Vec<Goal>,
    pub sessions: Vec<TutorSession>,
    pub palaces: Vec<Palace>,
}

/// `items` with `item` replacing the one with the same id (or added at the end).
fn replaced<T: Clone>(items: &[T], item: T, same: impl Fn(&T) -> bool) -> Vec<T> {
    match items.iter().position(same) {
        Some(at) => items.iter().enumerate().map(|(i, old)| if i == at { item.clone() } else { old.clone() }).collect(),
        None => items.iter().cloned().chain(std::iter::once(item)).collect(),
    }
}

impl LearningRecords {
    /// The records of `network`; sessions newest first.
    pub fn of_network(&self, network: &NetworkId) -> LearningRecords {
        let mut sessions: Vec<TutorSession> = self.sessions.iter().filter(|s| &s.network_id == network).cloned().collect();
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        LearningRecords {
            goals: self.goals.iter().filter(|g| &g.network_id == network).cloned().collect(),
            sessions,
            palaces: self.palaces.iter().filter(|p| &p.network_id == network).cloned().collect(),
        }
    }

    pub fn goal(&self, id: &GoalId) -> Option<&Goal> {
        self.goals.iter().find(|goal| &goal.id == id)
    }

    pub fn session(&self, id: &SessionId) -> Option<&TutorSession> {
        self.sessions.iter().find(|session| &session.id == id)
    }

    pub fn palace(&self, id: &PalaceId) -> Option<&Palace> {
        self.palaces.iter().find(|palace| &palace.id == id)
    }

    pub fn with_goal(self, goal: Goal) -> Self {
        Self { goals: replaced(&self.goals, goal.clone(), |g| g.id == goal.id), ..self }
    }

    pub fn without_goal(self, id: &GoalId) -> Self {
        Self { goals: self.goals.into_iter().filter(|g| &g.id != id).collect(), ..self }
    }

    pub fn with_session(self, session: TutorSession) -> Self {
        Self { sessions: replaced(&self.sessions, session.clone(), |s| s.id == session.id), ..self }
    }

    pub fn without_session(self, id: &SessionId) -> Self {
        Self { sessions: self.sessions.into_iter().filter(|s| &s.id != id).collect(), ..self }
    }

    pub fn with_palace(self, palace: Palace) -> Self {
        Self { palaces: replaced(&self.palaces, palace.clone(), |p| p.id == palace.id), ..self }
    }

    pub fn without_palace(self, id: &PalaceId) -> Self {
        Self { palaces: self.palaces.into_iter().filter(|p| &p.id != id).collect(), ..self }
    }
}
