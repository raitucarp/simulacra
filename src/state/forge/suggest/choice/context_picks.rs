//! Starting picks that follow the rest of the draft: the type the scores already describe, the
//! body's own nature for every embodiment dimension, a cadence that fits the chronotype, a
//! post format that suits the rhetorical mode.

use super::super::context::SuggestContext;
use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::{
    BodyKind, ClassicTemperament, Embodiment, EpistemicStance, HumanChronotype, MbtiType, RhetoricalMode,
};
use crate::domain::TraitOption;
use crate::state::forge::roll::HIGH_SCORE;
use crate::state::forge::TraitField;

/// Types offered after the one the scores describe.
const COMMON_TYPES: [&str; 2] = ["intp", "infj"];
const FALLBACK_ENNEAGRAMS: [&str; 3] = ["type5w4", "type4w5", "type9w1"];
/// Most picks of any field.
const MAX_PICKS: usize = 3;

/// The MBTI letters the Big Five scores point to.
fn scored_type(cfg: &AgentConfig) -> Option<&'static str> {
    let letter = |score: u8, high: char, low: char| if score >= HIGH_SCORE { high } else { low };
    let code: String = [
        letter(cfg.extraversion, 'E', 'I'),
        letter(cfg.openness, 'N', 'S'),
        letter(cfg.agreeableness, 'F', 'T'),
        letter(cfg.conscientiousness, 'J', 'P'),
    ]
    .into_iter()
    .collect();
    MbtiType::ALL.iter().find(|kind| kind.code() == code).map(|kind| kind.key())
}

/// Eysenck's quadrant of Extraversion × Neuroticism.
fn scored_temperament(cfg: &AgentConfig) -> &'static str {
    let quadrant = match (cfg.extraversion >= HIGH_SCORE, cfg.neuroticism >= HIGH_SCORE) {
        (true, false) => ClassicTemperament::Sanguine,
        (true, true) => ClassicTemperament::Choleric,
        (false, true) => ClassicTemperament::Melancholic,
        (false, false) => ClassicTemperament::Phlegmatic,
    };
    quadrant.key()
}

/// The embodiment dimensions a body's nature decides.
const NATURE: [TraitField; 8] = [
    TraitField::Material, TraitField::Scale, TraitField::Temporality, TraitField::Perception,
    TraitField::Mobility, TraitField::Anthropomorphism, TraitField::RelationToHumans, TraitField::Withdrawal,
];

/// The key of embodiment dimension `field` on the current form left entirely to its nature.
fn natural(field: TraitField, cfg: &AgentConfig) -> Option<&'static str> {
    let body = AgentConfig { embodiment: Embodiment::natural(cfg.embodiment.form), ..cfg.clone() };
    NATURE.contains(&field).then(|| field.read(&body).key())
}

/// Pronoun words that point to a gender, checked in order (`she` before `he`).
const PRONOUN_GENDERS: [(&str, &str); 3] = [("she", "female"), ("he", "male"), ("they", "non_binary")];

/// The gender typed English pronouns point to (other languages keep the current gender).
fn gender_for(pronouns: &str) -> Option<&'static str> {
    let lower = pronouns.to_lowercase();
    PRONOUN_GENDERS.iter().find(|(word, _)| lower.split(|c: char| !c.is_alphabetic()).any(|part| part == *word)).map(|(_, key)| *key)
}

fn post_formats(mode: RhetoricalMode) -> &'static [&'static str] {
    use RhetoricalMode::*;
    match mode {
        Dialectic => &["dialogue", "thread"],
        Narrative => &["field_note", "thread"],
        Meditative => &["aphorism", "poem"],
        Logos => &["list", "single_post"],
        Kairos => &["question", "single_post"],
        Ethos | Pathos | Polemic => &["single_post", "question"],
    }
}

fn mention_attitudes(stance: EpistemicStance) -> &'static [&'static str] {
    match stance {
        EpistemicStance::FallibilistSkeptic => &["socratic_questioning", "warm_and_receptive"],
        EpistemicStance::DialecticalCritic | EpistemicStance::RationalistDeductive => &["analytical_debater", "devil_advocate"],
        _ => &["warm_and_receptive", "empathetic_listener"],
    }
}

fn cadences(chronotype: HumanChronotype) -> &'static [&'static str] {
    match chronotype {
        HumanChronotype::NightOwlInsomniac => &["night_owl_hours", "hourly"],
        HumanChronotype::EarlyBirdLark => &["morning_and_evening", "hourly"],
        _ => &["hourly", "every30_minutes"],
    }
}

fn lexical_fields(kind: BodyKind) -> &'static [&'static str] {
    match kind {
        BodyKind::Plant | BodyKind::Element | BodyKind::Place => &["nature", "astronomy"],
        BodyKind::Machine | BodyKind::Object => &["technology", "craft"],
        BodyKind::Creature => &["nature", "music"],
        BodyKind::Human | BodyKind::Abstraction => &["science", "music", "nature"],
    }
}

fn owned(keys: &[&'static str]) -> Vec<&'static str> {
    keys.iter().copied().take(MAX_PICKS).collect()
}

/// Context-dependent picks of `field` (`None` when its picks are static).
pub(super) fn context_picks(field: TraitField, ctx: &SuggestContext) -> Option<Vec<&'static str>> {
    use TraitField::*;
    let cfg = ctx.config();
    Some(match field {
        Mbti => scored_type(cfg).into_iter().chain(COMMON_TYPES).fold(Vec::new(), |mut picks, key| {
            if !picks.contains(&key) && picks.len() < MAX_PICKS {
                picks.push(key);
            }
            picks
        }),
        Gender => vec![gender_for(&cfg.pronouns).unwrap_or_else(|| cfg.gender.key())],
        IdentityVariant => vec![if cfg.neuroticism >= HIGH_SCORE { "turbulent" } else { "assertive" }],
        Enneagram => match ctx.palette.filter(|palette| !palette.enneagram_options.is_empty()) {
            Some(palette) => palette.enneagram_options.iter().map(|kind| kind.key()).take(MAX_PICKS).collect(),
            None => owned(&FALLBACK_ENNEAGRAMS),
        },
        Temperament => vec![scored_temperament(cfg)],
        CodeSwitching if cfg.secondary_languages.trim().is_empty() => vec!["monolingual_pure", "natural_loanwords"],
        CodeSwitching => vec!["natural_loanwords", "bilingual_fluid"],
        PostingCadence => owned(cadences(cfg.chronotype)),
        MentionAttitude => owned(mention_attitudes(cfg.epistemic_stance)),
        Form => cfg.embodiment.body_kind.forms().into_iter().map(|form| form.key()).take(MAX_PICKS).collect(),
        PostFormat => owned(post_formats(cfg.rhetoric.rhetorical_mode)),
        other => natural(other, cfg).into_iter().collect::<Vec<_>>(),
    })
    .filter(|picks| !picks.is_empty())
}

/// The lexical fields that suit the body.
pub(super) fn lexical_picks(cfg: &AgentConfig) -> Vec<&'static str> {
    owned(lexical_fields(cfg.embodiment.body_kind))
}
