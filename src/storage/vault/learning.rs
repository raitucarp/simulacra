//! Vault operations of the learning records (sb E2): one startup read, queued writes.

use super::Vault;
use crate::domain::learning::{Goal, LearningRecords, Palace, TutorSession};
use crate::error::Result;

impl Vault {
    /// Reads every goal, tutor session and palace (after every queued write).
    pub fn load_learning(&self) -> Result<LearningRecords> {
        self.read(|db| db.load_learning())
    }

    /// Queues inserting or replacing `goal`.
    pub fn save_goal(&self, goal: &Goal) -> Result<()> {
        let goal = goal.clone();
        self.write(move |db| db.save_goal(&goal))
    }

    pub fn delete_goal(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.write(move |db| db.delete_goal(&id))
    }

    /// Queues inserting or replacing `session`.
    pub fn save_tutor_session(&self, session: &TutorSession) -> Result<()> {
        let session = session.clone();
        self.write(move |db| db.save_tutor_session(&session))
    }

    pub fn delete_tutor_session(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.write(move |db| db.delete_tutor_session(&id))
    }

    /// Queues inserting or replacing `palace`.
    pub fn save_palace(&self, palace: &Palace) -> Result<()> {
        let palace = palace.clone();
        self.write(move |db| db.save_palace(&palace))
    }

    pub fn delete_palace(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        self.write(move |db| db.delete_palace(&id))
    }
}
