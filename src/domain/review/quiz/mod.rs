//! Quiz modes beyond the flashcard: multiple choice, free recall and ordering. Each is pure;
//! randomness comes from a seed the caller chooses.

pub mod affinity;
pub mod choice;
pub mod order;
pub mod order_source;
pub mod recall;
pub mod run;
pub mod shuffle;

pub use run::{ChoiceRun, RecallRun};

pub use affinity::SourceVectors;
pub use choice::{build_choice, pick_distractors, ChoiceQuestion, DISTRACTORS};
pub use order::{OrderPuzzle, OrderScore, MAX_PIECES, MIN_PIECES};
pub use order_source::{piece_text, prose_paragraphs, sentence_window, thread_line, PIECE_CHARS};
pub use recall::{judge_recall, normalize_answer, similarity, RecallVerdict};
pub use shuffle::{mix_seed, shuffled};

/// Which quiz is running.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuizKind {
    Choice,
    Recall,
    Order,
}

impl QuizKind {
    pub const ALL: [Self; 3] = [Self::Choice, Self::Recall, Self::Order];

    /// Locale key stem (`review.quiz.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Choice => "choice",
            Self::Recall => "recall",
            Self::Order => "order",
        }
    }
}
