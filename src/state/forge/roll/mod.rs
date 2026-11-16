//! "Roll the dice": a coherent random thinker from a seed.

mod coherence;
pub(crate) mod rng;

pub use coherence::{drift_identity, drift_mbti, drift_temperament, HIGH_SCORE};

use super::attributes::{budget_remaining, score, BUDGET_FIELDS, INTELLIGENCE_FIELDS};
use super::draft::AgentDraft;
use super::fields::{ScoreField, MAX_SCORE};
use super::step::ForgeKind;
use super::validate::suggest_handle;
use crate::domain::agent_config::AgentConfig;
use crate::domain::archetype::{ArchetypeClass, NameSuggestion};
use crate::domain::portrait::{suggested_outfit, AvatarMode, PortraitSpec};
use crate::domain::{NetworkId, Profile};
use rng::SplitMix64;

/// How far each personality score may drift from the class preset.
pub const PERSONALITY_JITTER: u8 = 8;
/// How far each intelligence quotient may drift from the class preset.
pub const QUOTIENT_JITTER: u8 = 6;
/// Inclusive age range of a rolled thinker.
pub const ROLL_AGE_RANGE: (u16, u16) = (22, 74);

/// A new draft of `kind` in `network_id`, filled with a random coherent thinker.
pub fn roll_random(seed: u64, kind: ForgeKind, network_id: impl Into<NetworkId>, existing: &[Profile]) -> AgentDraft {
    let mut draft = AgentDraft::new(kind, network_id);
    roll_into(&mut draft, seed, existing);
    draft
}

/// Re-rolls `draft` (one undo step) and returns the class it was rolled from. Personal
/// fields a class never touches (languages, location, prompt assembly) are kept; the
/// avatar becomes a generated portrait from the same seed.
pub fn roll_into(draft: &mut AgentDraft, seed: u64, existing: &[Profile]) -> ArchetypeClass {
    let mut rng = SplitMix64::new(seed);
    let class = ArchetypeClass::ALL[rng.below(ArchetypeClass::ALL.len())];
    let pick = pick_name(class, &mut rng, existing).clone();
    let handle = suggest_handle(&pick.name, draft.id(), existing);
    let mut cfg = draft.profile().config.clone();
    class.apply_to(&mut cfg);
    roll_traits(class, &mut cfg, &mut rng);
    cfg.gender = pick.gender;
    cfg.pronouns = pick.pronouns;
    cfg.avatar = Some(AvatarMode::Pcg(rolled_portrait(seed, &cfg.occupation)));
    draft.transform(|profile| {
        profile.name = pick.name;
        profile.handle = handle;
        profile.config = cfg;
    });
    class
}

/// A random portrait from `seed`, dressed for `occupation` when an outfit fits it.
fn rolled_portrait(seed: u64, occupation: &str) -> PortraitSpec {
    let spec = PortraitSpec::random(seed);
    PortraitSpec { outfit: suggested_outfit(occupation).unwrap_or(spec.outfit), ..spec }
}

/// A suggestion of `class` that no existing profile is named after, starting at a random one.
fn pick_name<'a>(class: ArchetypeClass, rng: &mut SplitMix64, existing: &[Profile]) -> &'a NameSuggestion {
    let names = class.name_suggestions();
    let start = rng.below(names.len());
    let taken = |n: &NameSuggestion| existing.iter().any(|p| p.name.eq_ignore_ascii_case(&n.name));
    (0..names.len())
        .map(|offset| &names[(start + offset) % names.len()])
        .find(|n| !taken(n))
        .unwrap_or(&names[start])
}

/// Jitters scores around the preset, keeps the build balanced and the typology coherent.
fn roll_traits(class: ArchetypeClass, cfg: &mut AgentConfig, rng: &mut SplitMix64) {
    let preset = &class.preset().config;
    for field in BUDGET_FIELDS {
        set_score(cfg, field, rng.jitter(score(preset, field), PERSONALITY_JITTER, MAX_SCORE));
    }
    for field in INTELLIGENCE_FIELDS {
        set_score(cfg, field, rng.jitter(score(preset, field), QUOTIENT_JITTER, MAX_SCORE));
    }
    while budget_remaining(cfg) < 0 {
        let highest = BUDGET_FIELDS.into_iter().max_by_key(|f| score(cfg, *f)).unwrap_or(ScoreField::Openness);
        set_score(cfg, highest, score(cfg, highest).saturating_sub(1));
    }
    cfg.mbti = drift_mbti(preset, cfg);
    cfg.identity_variant = drift_identity(preset, cfg);
    cfg.temperament = drift_temperament(preset, cfg);
    let options = &class.preset().enneagram_options;
    cfg.enneagram = options.get(rng.below(options.len())).copied().unwrap_or(preset.enneagram);
    cfg.age = rng.in_range(ROLL_AGE_RANGE.0, ROLL_AGE_RANGE.1);
}

fn set_score(cfg: &mut AgentConfig, field: ScoreField, value: u8) {
    *field.of_mut(cfg) = value;
}
