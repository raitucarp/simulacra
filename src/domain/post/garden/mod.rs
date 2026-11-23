//! N3: the garden. A post or thread promoted to the garden becomes an evergreen note that
//! lives outside the stream and matures as it is tended: seed, then budding, then evergreen.
//! The promoted post keeps a [`GardenNote`] in `extras.garden`; the replies under it go with
//! it. Nothing is copied: the note is the post, so its versions, symbols and lock stay.

use crate::domain::document::keyed::keyed_enum;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// How grown a garden note is, youngest first. The key is stored: never rename one.
    Maturity {
        /// Just planted: a rough thought worth keeping.
        Seed => "seed",
        /// Being worked on: linked, rewritten, growing.
        Budding => "budding",
        /// Settled: a note that holds up and is returned to.
        Evergreen => "evergreen",
    }
}

impl Maturity {
    /// The stage after this one (`None` once evergreen).
    pub fn next(self) -> Option<Self> {
        Self::ALL.get(self.rank() + 1).copied()
    }

    /// The stage before this one (`None` for a seed).
    pub fn previous(self) -> Option<Self> {
        self.rank().checked_sub(1).and_then(|at| Self::ALL.get(at).copied())
    }

    /// The art of this stage (`illustrations/garden/<key>.svg`, a symbol canvas).
    pub fn art_path(self) -> String {
        format!("illustrations/garden/{}.svg", self.key())
    }

    fn rank(self) -> usize {
        Self::ALL.iter().position(|stage| *stage == self).unwrap_or_default()
    }
}

/// A post kept in the garden (`extras.garden`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GardenNote {
    pub maturity: Maturity,
    /// When it was promoted.
    pub planted_at: Timestamp,
    /// When its stage last changed (`None`: not since it was planted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tended_at: Option<Timestamp>,
}

impl GardenNote {
    /// A seed planted at `now`.
    pub fn planted(now: Timestamp) -> Self {
        Self { maturity: Maturity::Seed, planted_at: now, tended_at: None }
    }

    /// The note grown (or cut back) to `maturity` at `now`; unchanged when already there.
    #[must_use]
    pub fn tended(self, maturity: Maturity, now: Timestamp) -> Self {
        if maturity == self.maturity {
            return self;
        }
        Self { maturity, tended_at: Some(now), ..self }
    }
}

#[cfg(test)]
mod tests;
