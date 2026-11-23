//! N16: the lens fan. One click asks several agents to reply to a post, each through a
//! different lens (ethics, history, economics…). The post keeps who was asked through which
//! lens ([`LensFan`], `extras.fan`); each reply keeps its lens (`extras.lens`), so the fan reads
//! side by side. Every agent gets one lens, the one its knowledge fits best. A locked post is
//! never fanned out: an agent would read only its placeholder.

mod route;

pub use route::{assign, lens_fit};

use crate::domain::document::keyed::keyed_enum;
use crate::domain::ProfileId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// The most lenses one fan asks through.
pub const MAX_LENSES: usize = 4;

keyed_enum! {
    /// A way of looking at a post. The key is stored: never rename one.
    Lens {
        Ethics => "ethics",
        History => "history",
        Economics => "economics",
        Science => "science",
        Psychology => "psychology",
        Culture => "culture",
        Ecology => "ecology",
        Law => "law",
    }
}

impl Lens {
    /// The lenses a fan starts with.
    pub const DEFAULTS: [Self; 3] = [Self::Ethics, Self::History, Self::Economics];

    /// What the agent is asked to attend to (English: models read it).
    pub const fn instruction(self) -> &'static str {
        match self {
            Self::Ethics => "what is right or wrong here, who is affected, and which values pull against each other",
            Self::History => "what came before, which precedents or past cases it echoes, and how views of it changed over time",
            Self::Economics => "costs and benefits, incentives, trade-offs, and who pays",
            Self::Science => "what the evidence says, how it could be tested, and what is still unknown",
            Self::Psychology => "what people think, feel and do here, and the biases at play",
            Self::Culture => "the customs, stories and meanings different communities bring to it",
            Self::Ecology => "living systems, resources, and the long-run effects on the environment",
            Self::Law => "the rules, rights and duties it touches, and how a court or regulator might see it",
        }
    }

    /// The prompt line that turns a reply into a reply through this lens.
    pub fn prompt_line(self) -> String {
        format!("Reply through the lens of {}: {}. Stay in your own voice; keep to this lens.", self.key(), self.instruction())
    }
}

/// One agent asked, and the lens it was asked through.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanEntry {
    pub agent: ProfileId,
    pub lens: Lens,
}

/// A post fanned out through lenses (`extras.fan`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LensFan {
    pub entries: Vec<FanEntry>,
    pub at: Timestamp,
}

/// One line of a fan as it stands: the lens, who was asked, and the reply once there is one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanLine {
    pub lens: Lens,
    pub agent: ProfileId,
    pub reply: Option<crate::domain::PostId>,
}

#[cfg(test)]
mod tests;
