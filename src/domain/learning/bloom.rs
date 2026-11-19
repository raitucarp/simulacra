//! Bloom's levels of understanding (revised taxonomy, Anderson and Krathwohl 2001), from
//! recalling a fact to producing something new with it. The tutor climbs them one at a time;
//! the order of the variants is the order of the ladder.

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// One rung of Bloom's ladder (the variant order is the ladder's order).
    BloomLevel {
        Remember => "remember",
        Understand => "understand",
        Apply => "apply",
        Analyse => "analyse",
        Evaluate => "evaluate",
        Create => "create",
    }
}

impl BloomLevel {
    /// The first rung.
    pub const LOWEST: Self = Self::Remember;
    /// The last rung.
    pub const HIGHEST: Self = Self::Create;

    /// One-based height on the ladder (Remember = 1, Create = 6).
    pub fn rank(self) -> usize {
        Self::ALL.iter().position(|level| *level == self).map_or(1, |at| at + 1)
    }

    /// Height as a share of the ladder (Remember ≈ 0.17, Create = 1).
    pub fn share(self) -> f64 {
        self.rank() as f64 / Self::ALL.len() as f64
    }

    /// The rung above (Create stays Create).
    pub fn next(self) -> Self {
        Self::ALL.get(self.rank()).copied().unwrap_or(Self::HIGHEST)
    }

    /// The rung below (Remember stays Remember).
    pub fn previous(self) -> Self {
        self.rank().checked_sub(2).and_then(|at| Self::ALL.get(at).copied()).unwrap_or(Self::LOWEST)
    }

    /// What reaching the level shows, as the tutor's prompt describes it (prompts are English).
    pub const fn describe(self) -> &'static str {
        match self {
            Self::Remember => "recall the key terms and claims accurately",
            Self::Understand => "explain the idea in their own words and give an example",
            Self::Apply => "use the idea on a new, concrete case",
            Self::Analyse => "take the idea apart: its assumptions, parts and how they relate",
            Self::Evaluate => "judge the idea: its strengths, limits and the best objection",
            Self::Create => "build something new with it: an extension, a synthesis or a test",
        }
    }
}
