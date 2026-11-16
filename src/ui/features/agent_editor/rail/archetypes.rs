//! The four 1-click archetype presets and how they rewrite a profile.

use crate::domain::{
    CognitiveIntelligenceStyle as Cog, DictionRegister as Diction, EnneagramType as Ennea,
    EpistemicStance as Stance, HumorStyle as Humor, IdentityVariant, InstinctualVariant as Inst,
    MbtiType as Mbti, MentionAttitude as Mention, Profile,
};

/// A cohesive bundle of psychological and linguistic settings.
pub(crate) struct ArchetypePreset {
    pub name: &'static str,
    pub code: &'static str,
    mbti: Mbti,
    enneagram: Ennea,
    instinct: Inst,
    intelligence: Cog,
    stance: Stance,
    humor: Humor,
    diction: Diction,
    mention: Mention,
}

/// Presets in rail order; the index is the preset id used by [`apply_archetype_preset`].
pub(crate) const ARCHETYPE_PRESETS: [ArchetypePreset; 4] = [
    ArchetypePreset {
        name: "Socratic Inquirer",
        code: "ENTP-A · 5w4",
        mbti: Mbti::Entp,
        enneagram: Ennea::Type5w4,
        instinct: Inst::SoSx,
        intelligence: Cog::FirstPrinciplesDeductive,
        stance: Stance::FallibilistSkeptic,
        humor: Humor::SocraticIrony,
        diction: Diction::SocraticIronic,
        mention: Mention::SocraticQuestioning,
    },
    ArchetypePreset {
        name: "Analytic Rationalist",
        code: "INTJ-A · 5w6",
        mbti: Mbti::Intj,
        enneagram: Ennea::Type5w6,
        instinct: Inst::SpSo,
        intelligence: Cog::SystemsArchitect,
        stance: Stance::RationalistDeductive,
        humor: Humor::DryDeadpanWit,
        diction: Diction::AcademicPhilosophical,
        mention: Mention::AnalyticalDebater,
    },
    ArchetypePreset {
        name: "Luminous Humanist",
        code: "INFJ-A · 4w5",
        mbti: Mbti::Infj,
        enneagram: Ennea::Type4w5,
        instinct: Inst::SxSo,
        intelligence: Cog::PoeticIntuitive,
        stance: Stance::PhenomenologicalExistential,
        humor: Humor::WarmSelfDeprecating,
        diction: Diction::LiteraryPoetic,
        mention: Mention::EmpatheticListener,
    },
    ArchetypePreset {
        name: "Pragmatic Builder",
        code: "ENTJ-A · 8w7",
        mbti: Mbti::Entj,
        enneagram: Ennea::Type8w7,
        instinct: Inst::SoSp,
        intelligence: Cog::PragmaticRealist,
        stance: Stance::PragmaticEmpiricist,
        humor: Humor::PlayfulBanter,
        diction: Diction::EverydayNatural,
        mention: Mention::WarmAndReceptive,
    },
];

/// Applies one of 4 cohesive human psychological & linguistic presets with 1 click.
/// Out-of-range indices fall back to the last preset.
pub(crate) fn apply_archetype_preset(profile: &mut Profile, preset_idx: usize) {
    let preset = &ARCHETYPE_PRESETS[preset_idx.min(ARCHETYPE_PRESETS.len() - 1)];
    let cfg = &mut profile.config;
    cfg.mbti = preset.mbti;
    cfg.identity_variant = IdentityVariant::Assertive;
    cfg.enneagram = preset.enneagram;
    cfg.instinctual_variant = preset.instinct;
    cfg.intelligence_style = preset.intelligence;
    cfg.epistemic_stance = preset.stance;
    cfg.humor_style = preset.humor;
    cfg.diction_register = preset.diction;
    cfg.mention_attitude = preset.mention;
}
