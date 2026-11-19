//! Personality, intelligence, vocabulary and spontaneity bands. The Big Five follow the MBTI
//! letters through their known correlations (E↔Extraversion, N↔Openness, F↔Agreeableness,
//! J↔Conscientiousness) and Neuroticism follows the Assertive / Turbulent identity. Every
//! band's middle fits the balanced-build point budget together.

use super::hint;
use super::super::model::{RangeHint, RangeUnit};
use super::super::param::Param;
use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::{CognitivePace, DictionRegister, EpistemicStance, IdentityVariant};
use crate::state::forge::fields::MAX_SCORE;
use crate::state::forge::ScoreField;

const VALID: (u16, u16) = (0, MAX_SCORE as u16);
const HIGH: (u16, u16) = (60, 85);
const LOW: (u16, u16) = (15, 45);
const MID_LOW: (u16, u16) = (30, 55);
const BALANCED: (u16, u16) = (40, 70);
const SHARP: (u16, u16) = (60, 90);
const DEEP: (u16, u16) = (70, 95);
const HONEST: (u16, u16) = (55, 80);
const TURBULENT: (u16, u16) = (50, 75);
const STEADY: (u16, u16) = (15, 40);
const RICH: (u16, u16) = (65, 90);
const PLAIN: (u16, u16) = (35, 65);
const STREET: (u16, u16) = (15, 40);
const QUICK: (u16, u16) = (55, 85);
const SLOW: (u16, u16) = (10, 35);
const MEASURED: (u16, u16) = (30, 60);
/// MBTI letter positions.
const ENERGY: usize = 0;
const PERCEPTION: usize = 1;
const JUDGEMENT: usize = 2;
const STRUCTURE: usize = 3;

/// `high` when letter `at` of the type is `letter`, else `low`.
fn by_letter(cfg: &AgentConfig, at: usize, letter: char, (high, low): ((u16, u16), (u16, u16))) -> (u16, u16) {
    if cfg.mbti.code().chars().nth(at) == Some(letter) {
        high
    } else {
        low
    }
}

fn vocabulary(register: DictionRegister) -> (u16, u16) {
    use DictionRegister::*;
    match register {
        AcademicPhilosophical | TechnicalPrecision | LiteraryPoetic => RICH,
        SocraticIronic => SHARP,
        EverydayNatural | WarmEmpathetic => PLAIN,
        SlangColloquial => STREET,
    }
}

fn spontaneity(pace: CognitivePace) -> (u16, u16) {
    match pace {
        CognitivePace::RapidAssociative | CognitivePace::HyperfocusBurst => QUICK,
        CognitivePace::SlowContemplative => SLOW,
        CognitivePace::DeliberateMeasured => MEASURED,
    }
}

fn existential(stance: EpistemicStance) -> (u16, u16) {
    match stance {
        EpistemicStance::PhenomenologicalExistential | EpistemicStance::HolisticMonist => DEEP,
        _ => SHARP,
    }
}

fn band(field: ScoreField, cfg: &AgentConfig) -> (u16, u16) {
    use ScoreField::*;
    match field {
        Extraversion => by_letter(cfg, ENERGY, 'E', (HIGH, LOW)),
        Openness => by_letter(cfg, PERCEPTION, 'N', (HIGH, MID_LOW)),
        Agreeableness => by_letter(cfg, JUDGEMENT, 'F', (HIGH, MID_LOW)),
        Conscientiousness => by_letter(cfg, STRUCTURE, 'J', (HIGH, MID_LOW)),
        Neuroticism if cfg.identity_variant == IdentityVariant::Turbulent => TURBULENT,
        Neuroticism => STEADY,
        HonestyHumility => HONEST,
        FluidIntelligence | CrystallizedIntelligence => SHARP,
        EmotionalIntelligence | SocialIntelligence => by_letter(cfg, JUDGEMENT, 'F', (SHARP, BALANCED)),
        ExistentialIntelligence => existential(cfg.epistemic_stance),
        VocabularyComplexity => vocabulary(cfg.diction_register),
        SpontaneityRate => spontaneity(cfg.cognitive_pace),
    }
}

/// The band of score `field` given the draft's type, identity, register and pace.
pub(super) fn score_hint(field: ScoreField, cfg: &AgentConfig) -> RangeHint {
    hint(Param::Score(field), VALID, band(field, cfg), RangeUnit::Plain)
}
