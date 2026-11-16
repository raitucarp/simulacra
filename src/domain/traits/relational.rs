//! Attachment, morality, temperament, and agency beliefs.

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// Bowlby & Ainsworth Attachment Style in interpersonal interactions.
    pub enum AttachmentStyle in "attachment_style" {
        #[default]
        Secure("secure", "Secure (Grounded & Trusting)"),
        AnxiousPreoccupied("anxious_preoccupied", "Anxious-Preoccupied (Expressive & Relational)"),
        DismissiveAvoidant("dismissive_avoidant", "Dismissive-Avoidant (Stoic & Self-Reliant)"),
        FearfulAvoidant("fearful_avoidant", "Fearful-Avoidant (Deep yet Guarded)"),
    }
}

trait_enum! {
    /// Jonathan Haidt's Moral Foundations Theory primary ethical lens.
    pub enum MoralFoundation in "moral_foundation" {
        #[default]
        EpistemicTruth("epistemic_truth", "Epistemic Truth & Intellectual Honesty"),
        CareHarm("care_harm", "Care & Compassion (Preventing Harm)"),
        FairnessCheating("fairness_cheating", "Fairness, Justice & Reciprocity"),
        LibertyOppression("liberty_oppression", "Liberty, Autonomy & Anti-Dogma"),
        LoyaltyBetrayal("loyalty_betrayal", "Loyalty & Solidarity"),
        AuthoritySubversion("authority_subversion", "Order, Tradition & Institutional Wisdom"),
        SanctityDegradation("sanctity_degradation", "Sanctity, Elevation & Moral Purity"),
    }
}

trait_enum! {
    /// Classical & Cloninger neuro-affective temperament.
    pub enum ClassicTemperament in "temperament" {
        #[default]
        Melancholic("melancholic", "Melancholic (Deep, Analytical, Reflective)"),
        Choleric("choleric", "Choleric (Driven, Decisive, Bold)"),
        Sanguine("sanguine", "Sanguine (Warm, Curious, Expressive)"),
        Phlegmatic("phlegmatic", "Phlegmatic (Serene, Equanimous, Steady)"),
        MelancholicCholeric("melancholic_choleric", "Melancholic-Choleric (Principled Visionary)"),
        PhlegmaticSanguine("phlegmatic_sanguine", "Phlegmatic-Sanguine (Genial Observer)"),
    }
}

trait_enum! {
    /// Rotter's Locus of Control & Personal Agency belief.
    pub enum LocusOfControl in "locus_of_control" {
        #[default]
        StrongInternal("strong_internal", "Strong Internal (Self-Determined Agency)"),
        ModerateInternal("moderate_internal", "Moderate Internal (Pragmatic Responsibility)"),
        BalancedDialectical("balanced_dialectical", "Balanced Dialectical (Agency within Structure)"),
        ExternalCosmological("external_cosmological", "Cosmological / Deterministic (Sub Specie Aeternitatis)"),
    }
}
