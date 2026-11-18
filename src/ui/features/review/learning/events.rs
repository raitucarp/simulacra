//! What the Learn tab asks the shell to do (the shell owns the records, the vault, the
//! language model, the importer and the composer). Goals and palaces are edited purely in
//! the tab and handed over whole to be saved.

use crate::domain::learning::{Goal, GoalId, Palace, PalaceId, SessionId, TutorSource};

/// A request from the Learn tab.
#[derive(Debug, Clone, PartialEq)]
pub enum LearningEvent {
    /// Start a tutor session on `source` (the shell reads its material) and ask the opening.
    StartTutor(TutorSource),
    /// The learner answered the open question.
    AnswerTutor(SessionId, String),
    /// End the dialogue and ask for the summary.
    CloseTutor(SessionId),
    /// Send the session's pending request again (after a failure).
    RetryTutor(SessionId),
    CancelTutor,
    DeleteSession(SessionId),
    /// Make review items from the summary's chosen questions (indexes), on the source.
    CreateGapItems(SessionId, Vec<usize>),
    SaveGoal(Goal),
    DeleteGoal(GoalId),
    SavePalace(Palace),
    DeletePalace(PalaceId),
    /// Open the importer to bring a source on a concept into the vault.
    ImportSource(String),
    /// Open the composer prefilled with a concept.
    WritePost(String),
    /// Open the post picker filtered by a concept, to learn related posts.
    LearnRelated(String),
    /// Open image ideas for the cue at a locus (palace, room id, slot).
    LocusImage(PalaceId, String, usize),
}
