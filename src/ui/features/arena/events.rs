//! What the Arena page asks the shell to do (the shell runs requests and owns the records).

use crate::domain::arena::debate::DebateSetup;
use crate::domain::arena::lab::Scenario;
use crate::domain::arena::ArenaId;
use crate::domain::{PostId, ProfileId};

/// A request from the Arena page.
#[derive(Debug, Clone, PartialEq)]
pub enum ArenaEvent {
    /// Start a new debate of this setup.
    StartDebate(DebateSetup),
    /// Speak the next slot (or judge) of a stopped debate.
    ContinueDebate(ArenaId),
    /// Stop the running debate or experiment (what was said is kept).
    Cancel,
    PublishDebate(ArenaId),
    DeleteDebate(ArenaId),
    /// Save a scenario without running it (`id` replaces a saved one).
    SaveExperiment { id: Option<ArenaId>, scenario: Scenario, agents: Vec<ProfileId>, preset: Option<String> },
    /// Run a scenario: a new experiment, or a saved one again from scratch.
    RunExperiment { id: Option<ArenaId>, scenario: Scenario, agents: Vec<ProfileId>, preset: Option<String> },
    /// Answer the cells a stopped experiment still lacks.
    ResumeExperiment(ArenaId),
    PublishExperiment(ArenaId),
    DeleteExperiment(ArenaId),
    OpenPost(PostId),
}
