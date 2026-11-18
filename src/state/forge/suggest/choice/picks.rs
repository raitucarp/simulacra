//! Good starting picks of the trait fields that do not depend on the rest of the draft: the
//! options that read naturally in a social stream and are easy to grow from.

use crate::state::forge::{TraitField, TraitSet};

/// One to three starting options of `field`, best first (`None` when they depend on context).
pub(super) fn static_picks(field: TraitField) -> Option<&'static [&'static str]> {
    use TraitField::*;
    Some(match field {
        Gender => &["female", "male", "non_binary"],
        InstinctualVariant => &["sp_so", "so_sx", "sx_sp"],
        AttachmentStyle => &["secure"],
        MoralFoundation => &["epistemic_truth", "care_harm", "fairness_cheating"],
        LocusOfControl => &["balanced_dialectical", "moderate_internal"],
        HumorStyle => &["dry_deadpan_wit", "socratic_irony", "warm_self_deprecating"],
        IntelligenceStyle => &["first_principles_deductive", "polymath_synthesizer", "empirical_inductive"],
        EpistemicStance => &["fallibilist_skeptic", "pragmatic_empiricist", "dialectical_critic"],
        CognitivePace => &["deliberate_measured", "slow_contemplative"],
        GrammarStyle => &["relaxed_conversational", "standard_formal"],
        DictionRegister => &["everyday_natural", "academic_philosophical", "warm_empathetic"],
        SentenceLength => &["balanced_cadence", "short_punchy"],
        Capitalization => &["standard_sentence"],
        PunctuationHabit => &["crisp_minimal", "em_dash_and_semicolon"],
        EmojiUsage => &["never", "rare_subtle"],
        Chronotype => &["daytime_diurnal", "twilight_contemplative"],
        ConflictHandling => &["dialectical_synthesis", "calm_rational_refutation"],
        AssemblyStrategy => &["full_human_synthesis"],
        PrePostReasoning => &["embodied_human_perspective", "dialectical_network_weaver"],
        BodyKind => &["human", "creature", "abstraction"],
        RhetoricalMode => &["logos", "dialectic", "narrative"],
        StylisticSchool => &["plain", "classical", "minimalist"],
        SemioticMode => &["symbolic", "typographic"],
        SentenceStructure => &["simple", "compound", "cumulative"],
        _ => return None,
    })
}

/// Starting picks of a multi-select (`None` when they depend on context).
pub(super) fn static_set_picks(field: TraitSet) -> Option<&'static [&'static str]> {
    match field {
        TraitSet::RhetoricalDevices => Some(&["metaphor", "analogy", "rhetorical_question"]),
        TraitSet::ActiveDays => Some(&["monday", "wednesday", "friday"]),
        TraitSet::LexicalFields => None,
    }
}
