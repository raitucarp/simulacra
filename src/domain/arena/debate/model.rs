//! A debate's setup and its turns: the motion, two to four agents with their sides, the
//! format, the number of rounds, an optional moderator and the judge.

use crate::domain::document::keyed::keyed_enum;
use crate::domain::{PostId, ProfileId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// How the debate runs.
    DebateFormat {
        /// Sides alternate: openings, rebuttals, closings.
        Oxford => "oxford",
        /// Two debaters: constructive speeches, each followed by cross-examination, then closings.
        LincolnDouglas => "lincoln_douglas",
        /// No sides: each round one participant asks and the others answer in turn.
        Socratic => "socratic",
    }
}

keyed_enum! {
    /// Where a speaker stands on the motion.
    Side {
        For => "for",
        Against => "against",
        /// A Socratic participant or the moderator.
        Open => "open",
    }
}

keyed_enum! {
    /// What a turn is for.
    TurnRole {
        Opening => "opening",
        Rebuttal => "rebuttal",
        CrossExamination => "cross_examination",
        Closing => "closing",
        Question => "question",
        Answer => "answer",
        /// The moderator's introduction or wrap-up.
        Moderation => "moderation",
    }
}

/// An agent in the debate and its side.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Debater {
    pub agent: ProfileId,
    pub side: Side,
}

/// Who scores the debate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Judge {
    /// The global model, with no persona.
    #[default]
    GlobalModel,
    /// An agent, through its own engine and dossier.
    Agent(ProfileId),
}

/// Everything chosen before the first turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebateSetup {
    pub motion: String,
    /// The post or thread the motion was taken from.
    #[serde(default)]
    pub source_post: Option<PostId>,
    pub debaters: Vec<Debater>,
    pub format: DebateFormat,
    pub rounds: u8,
    #[serde(default)]
    pub moderator: Option<ProfileId>,
    #[serde(default)]
    pub judge: Judge,
}

/// One speaking slot of the schedule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Slot {
    /// 1-based; 0 for the moderator's introduction.
    pub round: u8,
    pub speaker: ProfileId,
    pub side: Side,
    pub role: TurnRole,
    /// Whom a question or a cross-examination addresses.
    #[serde(default)]
    pub addressee: Option<ProfileId>,
}

/// A spoken turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Turn {
    pub slot: Slot,
    pub text: String,
    pub at: DateTime<Utc>,
}

impl DebateSetup {
    /// The side of `agent`, if it debates.
    pub fn side_of(&self, agent: &ProfileId) -> Option<Side> {
        self.debaters.iter().find(|debater| &debater.agent == agent).map(|debater| debater.side)
    }

    /// The debaters on `side`, in setup order.
    pub fn on_side(&self, side: Side) -> Vec<&ProfileId> {
        self.debaters.iter().filter(|debater| debater.side == side).map(|debater| &debater.agent).collect()
    }

    /// Every agent that speaks (moderator first).
    pub fn speakers(&self) -> Vec<&ProfileId> {
        self.moderator.iter().chain(self.debaters.iter().map(|debater| &debater.agent)).collect()
    }
}
