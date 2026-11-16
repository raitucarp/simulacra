//! The four MBTI temperament families (Keirsey): Analysts (NT), Diplomats (NF), Sentinels (SJ)
//! and Explorers (SP), used to group the sixteen type cards.

use crate::domain::MbtiType;

const GROUP_ROOT: &str = "forge.ui.birthsign.group";
const BLURB_SUFFIX: &str = "blurb";
/// Letter positions in a four-letter type code.
const PERCEIVING_AT: usize = 1;
const JUDGING_AT: usize = 2;
const LIFESTYLE_AT: usize = 3;
const INTUITION: char = 'N';
const THINKING: char = 'T';
const JUDGING: char = 'J';

/// One temperament family of MBTI types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MbtiGroup {
    Analysts,
    Diplomats,
    Sentinels,
    Explorers,
}

impl MbtiGroup {
    pub const ALL: [Self; 4] = [Self::Analysts, Self::Diplomats, Self::Sentinels, Self::Explorers];

    /// The family a type belongs to, from its N/S, T/F and J/P letters.
    pub fn of(ty: MbtiType) -> Self {
        let letter = |at: usize| ty.code().chars().nth(at).unwrap_or_default();
        match (letter(PERCEIVING_AT) == INTUITION, letter(JUDGING_AT) == THINKING, letter(LIFESTYLE_AT) == JUDGING) {
            (true, true, _) => Self::Analysts,
            (true, false, _) => Self::Diplomats,
            (false, _, true) => Self::Sentinels,
            (false, _, false) => Self::Explorers,
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Analysts => "analysts",
            Self::Diplomats => "diplomats",
            Self::Sentinels => "sentinels",
            Self::Explorers => "explorers",
        }
    }

    /// The types of this family, in display order.
    pub fn members(self) -> Vec<MbtiType> {
        MbtiType::ALL.into_iter().filter(|ty| Self::of(*ty) == self).collect()
    }

    /// i18n key of the family name: `forge.ui.birthsign.group.{key}`.
    pub fn label_key(self) -> String {
        format!("{GROUP_ROOT}.{}", self.key())
    }

    /// i18n key of the one-line blurb: `forge.ui.birthsign.group.{key}_blurb`.
    pub fn blurb_key(self) -> String {
        format!("{GROUP_ROOT}.{}_{BLURB_SUFFIX}", self.key())
    }
}
