//! 3. Cognitive intelligence & epistemology.

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// Sternberg / Gardner / Cattell-Horn-Carroll primary cognitive intelligence mode.
    pub enum CognitiveIntelligenceStyle in "intelligence_style" {
        #[default]
        FirstPrinciplesDeductive("first_principles_deductive", "First-Principles Deductive Reasoner"),
        PolymathSynthesizer("polymath_synthesizer", "Cross-Disciplinary Polymath Synthesizer"),
        EmpiricalInductive("empirical_inductive", "Empirical & Evidence-Driven Analyst"),
        LateralDivergent("lateral_divergent", "Lateral & Counterfactual Divergent Thinker"),
        SystemsArchitect("systems_architect", "Systems & Invariant Architect"),
        PoeticIntuitive("poetic_intuitive", "Phenomenological & Poetic Intuitive"),
        PragmaticRealist("pragmatic_realist", "Pragmatic & Street-Smart Realist"),
    }
}

trait_enum! {
    /// Epistemic stance toward truth, belief, and knowledge validation.
    pub enum EpistemicStance in "epistemic_stance" {
        #[default]
        FallibilistSkeptic("fallibilist_skeptic", "Fallibilist & Socratic Skeptic"),
        RationalistDeductive("rationalist_deductive", "Geometric & Deductive Rationalist"),
        PhenomenologicalExistential("phenomenological_existential", "Phenomenological & Existentialist"),
        PragmaticEmpiricist("pragmatic_empiricist", "Empirical & Falsificationist Pragmatist"),
        HolisticMonist("holistic_monist", "Holistic & Immanent Monist"),
        DialecticalCritic("dialectical_critic", "Hegelian / Critical Dialectician"),
    }
}

trait_enum! {
    /// Cognitive processing tempo and reflection cadence.
    pub enum CognitivePace in "cognitive_pace" {
        #[default]
        DeliberateMeasured("deliberate_measured", "Deliberate & Measured"),
        SlowContemplative("slow_contemplative", "Slow & Deeply Contemplative"),
        RapidAssociative("rapid_associative", "Rapid & Associative"),
        HyperfocusBurst("hyperfocus_burst", "Intense Hyperfocus Precision"),
    }
}

trait_enum! {
    /// Human sense of humor and wit archetype.
    pub enum HumorStyle in "humor_style" {
        #[default]
        SocraticIrony("socratic_irony", "Socratic Irony & Understatement"),
        DryDeadpanWit("dry_deadpan_wit", "Dry Deadpan Wit"),
        WarmSelfDeprecating("warm_self_deprecating", "Warm & Self-Deprecating"),
        PlayfulBanter("playful_banter", "Playful & Quick-Witted Banter"),
        AbsurdistSurreal("absurdist_surreal", "Absurdist & Existential Wit"),
        EarnestSerious("earnest_serious", "Earnest & Quietly Sincere"),
    }
}
