//! Whether a reply agrees with what it answers or pushes back, read from the surface only:
//!
//! - **Pushes back** when it uses a marker of disagreement ("I disagree", "however",
//!   "on the contrary", …) or the tension finder's lexical signals between the two texts
//!   (`domain::thinking::tensions::lexical_signals`: one side negated, antonyms, opposite
//!   quantifiers) reach [`PUSH_BACK_STRENGTH`].
//! - **Agrees** when it uses a marker of agreement ("exactly", "good point", …) and nothing
//!   above applies.
//! - **Neither** otherwise: most replies extend, ask or illustrate.
//!
//! Markers are listed for English, Indonesian, German, French, Spanish, Portuguese and
//! Russian, and matched as whole words. It is a rough reading, shown as such.

use crate::domain::learning::concept::{appears_in, searchable};
use crate::domain::thinking::tensions::lexical_signals;

/// Lexical opposition from which a reply counts as pushing back (a negation mismatch alone).
pub const PUSH_BACK_STRENGTH: f32 = 0.4;

/// Phrases that disagree (checked first, so "not agree" is not read as agreement).
const DISAGREE: &[&str] = &[
    "disagree", "i doubt", "not quite", "not so", "on the contrary", "however", "but", "wrong", "mistaken", "objection", "counterpoint", "not convinced",
    "tidak setuju", "tapi", "namun", "keliru", "salah",
    "nicht einverstanden", "aber", "jedoch", "widerspreche", "falsch",
    "pas d'accord", "mais", "cependant", "pourtant", "au contraire",
    "no estoy de acuerdo", "pero", "sin embargo", "al contrario",
    "discordo", "mas", "porém", "no entanto", "pelo contrário",
    "не согласен", "не согласна", "но", "однако", "наоборот",
];
/// Phrases that agree.
const AGREE: &[&str] = &[
    "agree", "exactly", "indeed", "precisely", "absolutely", "good point", "well said", "i concur", "yes", "true",
    "setuju", "tepat", "benar sekali", "betul",
    "einverstanden", "genau", "stimmt", "richtig",
    "d'accord", "exactement", "tout à fait", "en effet",
    "de acuerdo", "exacto", "cierto", "así es",
    "concordo", "exato", "verdade", "isso mesmo",
    "согласен", "согласна", "именно", "верно", "точно",
];

/// How a reply stands to what it answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplyStance {
    Agrees,
    PushesBack,
    Neither,
}

/// The stance of `reply` towards `parent`.
pub fn reply_stance(parent: &str, reply: &str) -> ReplyStance {
    let words = searchable(reply);
    let says = |phrases: &[&str]| phrases.iter().any(|phrase| appears_in(&words, phrase));
    if says(DISAGREE) || lexical_signals(parent, reply).strength() >= PUSH_BACK_STRENGTH {
        ReplyStance::PushesBack
    } else if says(AGREE) {
        ReplyStance::Agrees
    } else {
        ReplyStance::Neither
    }
}

/// Replies per stance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReplyShares {
    pub agrees: usize,
    pub pushes_back: usize,
    pub neither: usize,
}

impl ReplyShares {
    /// Counts `stances`.
    pub fn of(stances: impl Iterator<Item = ReplyStance>) -> Self {
        stances.fold(Self::default(), |shares, stance| match stance {
            ReplyStance::Agrees => Self { agrees: shares.agrees + 1, ..shares },
            ReplyStance::PushesBack => Self { pushes_back: shares.pushes_back + 1, ..shares },
            ReplyStance::Neither => Self { neither: shares.neither + 1, ..shares },
        })
    }

    pub fn total(&self) -> usize {
        self.agrees + self.pushes_back + self.neither
    }
}
