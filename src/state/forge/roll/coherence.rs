//! Keeping typologies consistent with jittered Big Five scores.
//!
//! A rolled thinker keeps its class's MBTI, identity and temperament, except where the
//! jitter pushed a score across the midpoint: then the matching letter / quadrant follows
//! the score, using the Big Five ↔ MBTI correlations (E↔Extraversion, N↔Openness,
//! F↔Agreeableness, J↔Conscientiousness) and Eysenck's Extraversion × Neuroticism quadrants.

use crate::domain::agent_config::AgentConfig;
use crate::domain::{ClassicTemperament, IdentityVariant, MbtiType};

/// Scores at or above this count as "high" on a dimension.
pub const HIGH_SCORE: u8 = 50;

type Score = fn(&AgentConfig) -> u8;

/// `(letter position, score, letter when high, letter when low)` per MBTI axis.
const MBTI_AXES: [(usize, Score, char, char); 4] = [
    (0, |c| c.extraversion, 'E', 'I'),
    (1, |c| c.openness, 'N', 'S'),
    (2, |c| c.agreeableness, 'F', 'T'),
    (3, |c| c.conscientiousness, 'J', 'P'),
];

/// `Some(is_high)` when `score` sits on a different side of the midpoint in `rolled`.
fn crossed(score: Score, preset: &AgentConfig, rolled: &AgentConfig) -> Option<bool> {
    let now = score(rolled) >= HIGH_SCORE;
    (now != (score(preset) >= HIGH_SCORE)).then_some(now)
}

/// The preset's MBTI with every crossed axis flipped to follow the rolled score.
pub fn drift_mbti(preset: &AgentConfig, rolled: &AgentConfig) -> MbtiType {
    let mut letters: Vec<char> = preset.mbti.code().chars().collect();
    for (position, score, high, low) in MBTI_AXES {
        if let (Some(is_high), Some(letter)) = (crossed(score, preset, rolled), letters.get_mut(position)) {
            *letter = if is_high { high } else { low };
        }
    }
    let code: String = letters.into_iter().collect();
    MbtiType::ALL.into_iter().find(|t| t.code() == code).unwrap_or(preset.mbti)
}

/// The preset's identity, switched when Neuroticism crossed the midpoint.
pub fn drift_identity(preset: &AgentConfig, rolled: &AgentConfig) -> IdentityVariant {
    match crossed(|c| c.neuroticism, preset, rolled) {
        Some(true) => IdentityVariant::Turbulent,
        Some(false) => IdentityVariant::Assertive,
        None => preset.identity_variant,
    }
}

/// The preset's temperament, or the Eysenck quadrant when E or N crossed the midpoint.
pub fn drift_temperament(preset: &AgentConfig, rolled: &AgentConfig) -> ClassicTemperament {
    let moved = crossed(|c| c.extraversion, preset, rolled).is_some()
        || crossed(|c| c.neuroticism, preset, rolled).is_some();
    if !moved {
        return preset.temperament;
    }
    match (rolled.extraversion >= HIGH_SCORE, rolled.neuroticism >= HIGH_SCORE) {
        (true, false) => ClassicTemperament::Sanguine,
        (true, true) => ClassicTemperament::Choleric,
        (false, true) => ClassicTemperament::Melancholic,
        (false, false) => ClassicTemperament::Phlegmatic,
    }
}
