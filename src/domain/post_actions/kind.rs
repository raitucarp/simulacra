//! The actions a thought offers, grouped by what they do for the reader.

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// One thinking action on a post. The key names its locale strings and its cache entry.
    ActionKind {
        /// Picks a concept from the post and shows vault hits, nearby thoughts and a definition.
        SearchConcept => "search_concept",
        /// Re-explains the post for a chosen audience (age five up to professor).
        Explain => "explain",
        /// Summarises the whole thread: gist, positions and open questions.
        SummariseThread => "summarise_thread",
        /// Lists key concepts and their relations as a small saveable map.
        MapConcepts => "map_concepts",
        /// Translates the post into an app locale or a typed language.
        Translate => "translate",
        /// The strongest version of the argument (principle of charity).
        Steelman => "steelman",
        /// Where the post misrepresents an opposing view, and the fair version of it.
        Strawman => "strawman",
        /// The unstated premises the conclusion depends on (enthymeme reconstruction).
        HiddenPremise => "hidden_premise",
        /// Concrete cases that break the central claim, and the claim that survives them.
        Counterexample => "counterexample",
        /// Cognitive science: heuristics and biases that may shape the reasoning, each with a
        /// debiasing question. Naming a bias makes it inspectable instead of invisible.
        CognitiveBiases => "cognitive_biases",
        /// Epistemology: each claim with the credence a careful reasoner would give it and why.
        /// Separating confidence from conviction is the core habit of calibrated thinking.
        ClaimConfidence => "claim_confidence",
        /// Argumentation theory: the crux between post and replies, the one question whose
        /// answer would change a mind. Disagreements end faster when argued at the crux.
        Crux => "crux",
        /// Socratic questions that probe clarity, assumptions, evidence and implications.
        Socratic => "socratic",
        /// Analogies from several domains, each with the point where it breaks.
        Analogies => "analogies",
        /// Philosophy of science: the claim as falsifiable predictions with a cheap test.
        /// A claim that forbids nothing teaches nothing; a prediction can be checked.
        FalsifiablePrediction => "falsifiable_prediction",
        /// Continues the thought one step further in the author's voice.
        ContinueThought => "continue_thought",
        /// sb E1: argumentation theory: the thread as an argument graph (Toulmin roles, support
        /// and attack), shown in the inspector's Argument view.
        MapArgument => "map_argument",
        /// sb E1: posts and vault passages that may contradict this one (the Tensions view).
        FindTensions => "find_tensions",
        /// sb E1: how atomic the post is (one claim, with context), and a split into one post per claim.
        CheckAtomicity => "check_atomicity",
    }
}

keyed_enum! {
    /// Sections of the actions menu and of the results panel's rail.
    ActionGroup {
        Understand => "understand",
        Challenge => "challenge",
        Explore => "explore",
    }
}

/// The extra choice an action needs before it can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionInput {
    None,
    Audience,
    Language,
    Concept,
}

/// Answer budget (output tokens) of short, list-like and long actions.
const SHORT_ANSWER_TOKENS: u32 = 500;
const LIST_ANSWER_TOKENS: u32 = 700;
const LONG_ANSWER_TOKENS: u32 = 900;

impl ActionKind {
    /// Where the action is listed.
    pub const fn group(self) -> ActionGroup {
        use ActionKind::*;
        match self {
            SearchConcept | Explain | SummariseThread | MapConcepts | Translate | MapArgument | CheckAtomicity => ActionGroup::Understand,
            Steelman | Strawman | HiddenPremise | Counterexample | CognitiveBiases | ClaimConfidence | Crux | FindTensions => ActionGroup::Challenge,
            Socratic | Analogies | FalsifiablePrediction | ContinueThought => ActionGroup::Explore,
        }
    }

    /// The choice the action asks for.
    pub const fn input(self) -> ActionInput {
        match self {
            Self::Explain => ActionInput::Audience,
            Self::Translate => ActionInput::Language,
            Self::SearchConcept => ActionInput::Concept,
            _ => ActionInput::None,
        }
    }

    /// The least output budget the action needs (the user's own cap applies when larger).
    pub const fn answer_tokens(self) -> u32 {
        match self {
            Self::ContinueThought | Self::SearchConcept => SHORT_ANSWER_TOKENS,
            Self::Steelman | Self::SummariseThread | Self::Translate | Self::Explain => LONG_ANSWER_TOKENS,
            _ => LIST_ANSWER_TOKENS,
        }
    }

    /// sb E1: whether the action opens a thinking tool of its own instead of an answer in the
    /// actions sheet.
    pub const fn opens_tool(self) -> bool {
        matches!(self, Self::MapArgument | Self::FindTensions | Self::CheckAtomicity)
    }

    /// Every action of `group`, in display order.
    pub fn in_group(group: ActionGroup) -> impl Iterator<Item = ActionKind> {
        Self::ALL.iter().copied().filter(move |kind| kind.group() == group)
    }
}
