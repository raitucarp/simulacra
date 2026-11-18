//! The Socratic moves a question can make, each described for the prompt.

use crate::domain::document::keyed::keyed_enum;
use crate::domain::learning::bloom::BloomLevel;

keyed_enum! {
    /// What a question does.
    SocraticMove {
        /// Draw out the learner's current model before anything else.
        Elicit => "elicit",
        /// Ask what a key term means, exactly.
        Define => "define",
        /// Ask for the idea in their own words.
        Explain => "explain",
        Example => "example",
        Counterexample => "counterexample",
        /// Put two of their statements side by side.
        Contradiction => "contradiction",
        Apply => "apply",
        Analyse => "analyse",
        Evaluate => "evaluate",
        Create => "create",
    }
}

impl SocraticMove {
    /// The move that tests `level` when nothing calls for a probe.
    pub const fn for_level(level: BloomLevel) -> Self {
        match level {
            BloomLevel::Remember => Self::Define,
            BloomLevel::Understand => Self::Explain,
            BloomLevel::Apply => Self::Apply,
            BloomLevel::Analyse => Self::Analyse,
            BloomLevel::Evaluate => Self::Evaluate,
            BloomLevel::Create => Self::Create,
        }
    }

    /// What the question should do, as the prompt asks for it (prompts are English).
    pub const fn describe(self) -> &'static str {
        match self {
            Self::Elicit => "ask what they currently think the idea is, before teaching anything",
            Self::Define => "ask them to define the key term precisely, in their own words",
            Self::Explain => "ask them to explain the idea in their own words, as to a friend",
            Self::Example => "ask for a concrete example of their own",
            Self::Counterexample => "offer no answer; ask for a case where their claim would fail",
            Self::Contradiction => "quote two of their own statements that pull apart and ask how both can hold",
            Self::Apply => "give a new, concrete situation and ask what the idea says about it",
            Self::Analyse => "ask which assumption the idea rests on, or how two of its parts relate",
            Self::Evaluate => "ask them to judge it: the strongest objection and whether it survives",
            Self::Create => "ask them to extend it: a new case, a synthesis with another idea, or a test",
        }
    }
}
