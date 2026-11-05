//! Comprehensive human psychology, cognitive intelligence, linguistic stylistics,
//! autonomous scheduling, social mention dynamics, and prompt assembly models.
//!
//! Designed to model each network member as a complete, multidimensional human
//! individual rather than a generic bot.

use super::profile::Profile;
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// 1. HUMAN IDENTITY & DEMOGRAPHICS
// ============================================================================

/// Human gender identity classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HumanGender {
    #[default]
    Male,
    Female,
    NonBinary,
    Custom,
}

impl HumanGender {
    pub const ALL: [Self; 4] = [Self::Male, Self::Female, Self::NonBinary, Self::Custom];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Male => "Male",
            Self::Female => "Female",
            Self::NonBinary => "Non-Binary",
            Self::Custom => "Custom / Unspecified",
        }
    }

    #[inline]
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Male => "Male",
            Self::Female => "Female",
            Self::NonBinary => "Non-Binary",
            Self::Custom => "Custom",
        }
    }

    #[inline]
    pub const fn default_pronouns(self) -> &'static str {
        match self {
            Self::Male => "he/him",
            Self::Female => "she/her",
            Self::NonBinary => "they/them",
            Self::Custom => "ia/dia",
        }
    }
}

impl fmt::Display for HumanGender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

// ============================================================================
// 2. SOTA PSYCHOLOGY & PERSONALITY TAXONOMIES
// ============================================================================

/// Myers-Briggs Type Indicator (16 Jungian Cognitive Archetypes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MbtiType {
    #[default]
    Intj,
    Intp,
    Entj,
    Entp,
    Infj,
    Infp,
    Enfj,
    Enfp,
    Istj,
    Isfj,
    Estj,
    Esfj,
    Istp,
    Isfp,
    Estp,
    Esfp,
}

impl MbtiType {
    pub const ALL: [Self; 16] = [
        Self::Intj,
        Self::Intp,
        Self::Entj,
        Self::Entp,
        Self::Infj,
        Self::Infp,
        Self::Enfj,
        Self::Enfp,
        Self::Istj,
        Self::Isfj,
        Self::Estj,
        Self::Esfj,
        Self::Istp,
        Self::Isfp,
        Self::Estp,
        Self::Esfp,
    ];

    #[inline]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Intj => "INTJ",
            Self::Intp => "INTP",
            Self::Entj => "ENTJ",
            Self::Entp => "ENTP",
            Self::Infj => "INFJ",
            Self::Infp => "INFP",
            Self::Enfj => "ENFJ",
            Self::Enfp => "ENFP",
            Self::Istj => "ISTJ",
            Self::Isfj => "ISFJ",
            Self::Estj => "ESTJ",
            Self::Esfj => "ESFJ",
            Self::Istp => "ISTP",
            Self::Isfp => "ISFP",
            Self::Estp => "ESTP",
            Self::Esfp => "ESFP",
        }
    }

    #[inline]
    pub const fn archetype_name(self) -> &'static str {
        match self {
            Self::Intj => "Architect",
            Self::Intp => "Logician",
            Self::Entj => "Commander",
            Self::Entp => "Dialectician",
            Self::Infj => "Advocate",
            Self::Infp => "Mediator",
            Self::Enfj => "Protagonist",
            Self::Enfp => "Catalyst",
            Self::Istj => "Archivist",
            Self::Isfj => "Steward",
            Self::Estj => "Organizer",
            Self::Esfj => "Harmonizer",
            Self::Istp => "Craftsperson",
            Self::Isfp => "Aesthete",
            Self::Estp => "Dynamo",
            Self::Esfp => "Performer",
        }
    }

    /// Returns the 4-function Harold Grant / Jungian cognitive stack (Dominant · Auxiliary · Tertiary · Inferior).
    #[inline]
    pub const fn cognitive_functions(self) -> &'static str {
        match self {
            Self::Intj => "Ni · Te · Fi · Se",
            Self::Intp => "Ti · Ne · Si · Fe",
            Self::Entj => "Te · Ni · Se · Fi",
            Self::Entp => "Ne · Ti · Fe · Si",
            Self::Infj => "Ni · Fe · Ti · Se",
            Self::Infp => "Fi · Ne · Si · Te",
            Self::Enfj => "Fe · Ni · Se · Ti",
            Self::Enfp => "Ne · Fi · Te · Si",
            Self::Istj => "Si · Te · Fi · Ne",
            Self::Isfj => "Si · Fe · Ti · Ne",
            Self::Estj => "Te · Si · Ne · Fi",
            Self::Esfj => "Fe · Si · Ne · Ti",
            Self::Istp => "Ti · Se · Ni · Fe",
            Self::Isfp => "Fi · Se · Ni · Te",
            Self::Estp => "Se · Ti · Fe · Ni",
            Self::Esfp => "Se · Fi · Te · Ni",
        }
    }

    #[inline]
    pub const fn cognitive_description(self) -> &'static str {
        match self {
            Self::Intj => "Introverted Intuition (Ni) dominant with Extraverted Thinking (Te): synthesizes long-range patterns into rigorous systems.",
            Self::Intp => "Introverted Thinking (Ti) dominant with Extraverted Intuition (Ne): deconstructs axioms and explores conceptual possibilities.",
            Self::Entj => "Extraverted Thinking (Te) dominant with Introverted Intuition (Ni): marshals evidence and structures decisive frameworks.",
            Self::Entp => "Extraverted Intuition (Ne) dominant with Introverted Thinking (Ti): interrogates premises via Socratic counter-examples.",
            Self::Infj => "Introverted Intuition (Ni) dominant with Extraverted Feeling (Fe): perceives moral-existential depth and human subtext.",
            Self::Infp => "Introverted Feeling (Fi) dominant with Extraverted Intuition (Ne): guided by authentic inner ethics and poetic imagination.",
            Self::Enfj => "Extraverted Feeling (Fe) dominant with Introverted Intuition (Ni): fosters collective insight and empathetic mentorship.",
            Self::Enfp => "Extraverted Intuition (Ne) dominant with Introverted Feeling (Fi): radiates associative curiosity and existential warmth.",
            Self::Istj => "Introverted Sensing (Si) dominant with Extraverted Thinking (Te): grounds judgment in verified history and empirical precedent.",
            Self::Isfj => "Introverted Sensing (Si) dominant with Extraverted Feeling (Fe): preserves human memory, care, and continuity.",
            Self::Estj => "Extraverted Thinking (Te) dominant with Introverted Sensing (Si): enforces clear standards and practical accountability.",
            Self::Esfj => "Extraverted Feeling (Fe) dominant with Introverted Sensing (Si): nurtures community cohesion and shared norms.",
            Self::Istp => "Introverted Thinking (Ti) dominant with Extraverted Sensing (Se): masters mechanical sympathy and direct first-principles craft.",
            Self::Isfp => "Introverted Feeling (Fi) dominant with Extraverted Sensing (Se): expresses quiet aesthetic sensitivity and lived presence.",
            Self::Estp => "Extraverted Sensing (Se) dominant with Introverted Thinking (Ti): thrives on immediate tactical reality and kinetic wit.",
            Self::Esfp => "Extraverted Sensing (Se) dominant with Introverted Feeling (Fi): celebrates vivid sensory experience and spontaneous connection.",
        }
    }
}

impl fmt::Display for MbtiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.code(), self.archetype_name())
    }
}

/// Assertive (-A) vs Turbulent (-T) identity modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum IdentityVariant {
    #[default]
    Assertive,
    Turbulent,
}

impl IdentityVariant {
    pub const ALL: [Self; 2] = [Self::Assertive, Self::Turbulent];

    #[inline]
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Assertive => "A",
            Self::Turbulent => "T",
        }
    }

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Assertive => "Assertive (-A)",
            Self::Turbulent => "Turbulent (-T)",
        }
    }
}

/// Enneagram of Personality type with wing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EnneagramType {
    Type1w9,
    Type1w2,
    Type2w1,
    Type2w3,
    Type3w2,
    Type3w4,
    Type4w3,
    Type4w5,
    #[default]
    Type5w4,
    Type5w6,
    Type6w5,
    Type6w7,
    Type7w6,
    Type7w8,
    Type8w7,
    Type8w9,
    Type9w8,
    Type9w1,
}

impl EnneagramType {
    pub const ALL: [Self; 18] = [
        Self::Type1w9,
        Self::Type1w2,
        Self::Type2w1,
        Self::Type2w3,
        Self::Type3w2,
        Self::Type3w4,
        Self::Type4w3,
        Self::Type4w5,
        Self::Type5w4,
        Self::Type5w6,
        Self::Type6w5,
        Self::Type6w7,
        Self::Type7w6,
        Self::Type7w8,
        Self::Type8w7,
        Self::Type8w9,
        Self::Type9w8,
        Self::Type9w1,
    ];

    #[inline]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Type1w9 => "1w9",
            Self::Type1w2 => "1w2",
            Self::Type2w1 => "2w1",
            Self::Type2w3 => "2w3",
            Self::Type3w2 => "3w2",
            Self::Type3w4 => "3w4",
            Self::Type4w3 => "4w3",
            Self::Type4w5 => "4w5",
            Self::Type5w4 => "5w4",
            Self::Type5w6 => "5w6",
            Self::Type6w5 => "6w5",
            Self::Type6w7 => "6w7",
            Self::Type7w6 => "7w6",
            Self::Type7w8 => "7w8",
            Self::Type8w7 => "8w7",
            Self::Type8w9 => "8w9",
            Self::Type9w8 => "9w8",
            Self::Type9w1 => "9w1",
        }
    }

    #[inline]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Type1w9 => "1w9 · The Idealist",
            Self::Type1w2 => "1w2 · The Advocate",
            Self::Type2w1 => "2w1 · The Altruist",
            Self::Type2w3 => "2w3 · The Host",
            Self::Type3w2 => "3w2 · The Enchanter",
            Self::Type3w4 => "3w4 · The Virtuoso",
            Self::Type4w3 => "4w3 · The Aristocrat",
            Self::Type4w5 => "4w5 · The Bohemian",
            Self::Type5w4 => "5w4 · The Iconoclast",
            Self::Type5w6 => "5w6 · The Problem Solver",
            Self::Type6w5 => "6w5 · The Defender",
            Self::Type6w7 => "6w7 · The Confidant",
            Self::Type7w6 => "7w6 · The Entertainer",
            Self::Type7w8 => "7w8 · The Opportunist",
            Self::Type8w7 => "8w7 · The Maverick",
            Self::Type8w9 => "8w9 · The Diplomat-Bear",
            Self::Type9w8 => "9w8 · The Referee",
            Self::Type9w1 => "9w1 · The Dreamer",
        }
    }
}

/// Enneagram Instinctual Stacking (Self-Preservation, Social, One-to-One/Sexual).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum InstinctualVariant {
    #[default]
    SpSo,
    SpSx,
    SoSp,
    SoSx,
    SxSp,
    SxSo,
}

impl InstinctualVariant {
    pub const ALL: [Self; 6] = [
        Self::SpSo,
        Self::SpSx,
        Self::SoSp,
        Self::SoSx,
        Self::SxSp,
        Self::SxSo,
    ];

    #[inline]
    pub const fn code(self) -> &'static str {
        match self {
            Self::SpSo => "sp/so",
            Self::SpSx => "sp/sx",
            Self::SoSp => "so/sp",
            Self::SoSx => "so/sx",
            Self::SxSp => "sx/sp",
            Self::SxSo => "sx/so",
        }
    }

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::SpSo => "sp/so (Self-Preservation / Social)",
            Self::SpSx => "sp/sx (Self-Preservation / Intimate)",
            Self::SoSp => "so/sp (Social / Self-Preservation)",
            Self::SoSx => "so/sx (Social / Intimate)",
            Self::SxSp => "sx/sp (Intimate / Self-Preservation)",
            Self::SxSo => "sx/so (Intimate / Social)",
        }
    }
}

/// Bowlby & Ainsworth Attachment Style in interpersonal interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AttachmentStyle {
    #[default]
    Secure,
    AnxiousPreoccupied,
    DismissiveAvoidant,
    FearfulAvoidant,
}

impl AttachmentStyle {
    pub const ALL: [Self; 4] = [
        Self::Secure,
        Self::AnxiousPreoccupied,
        Self::DismissiveAvoidant,
        Self::FearfulAvoidant,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Secure => "Secure (Grounded & Trusting)",
            Self::AnxiousPreoccupied => "Anxious-Preoccupied (Expressive & Relational)",
            Self::DismissiveAvoidant => "Dismissive-Avoidant (Stoic & Self-Reliant)",
            Self::FearfulAvoidant => "Fearful-Avoidant (Deep yet Guarded)",
        }
    }
}

/// Jonathan Haidt's Moral Foundations Theory primary ethical lens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MoralFoundation {
    #[default]
    EpistemicTruth,
    CareHarm,
    FairnessCheating,
    LibertyOppression,
    LoyaltyBetrayal,
    AuthoritySubversion,
    SanctityDegradation,
}

impl MoralFoundation {
    pub const ALL: [Self; 7] = [
        Self::EpistemicTruth,
        Self::CareHarm,
        Self::FairnessCheating,
        Self::LibertyOppression,
        Self::LoyaltyBetrayal,
        Self::AuthoritySubversion,
        Self::SanctityDegradation,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::EpistemicTruth => "Epistemic Truth & Intellectual Honesty",
            Self::CareHarm => "Care & Compassion (Preventing Harm)",
            Self::FairnessCheating => "Fairness, Justice & Reciprocity",
            Self::LibertyOppression => "Liberty, Autonomy & Anti-Dogma",
            Self::LoyaltyBetrayal => "Loyalty & Solidarity",
            Self::AuthoritySubversion => "Order, Tradition & Institutional Wisdom",
            Self::SanctityDegradation => "Sanctity, Elevation & Moral Purity",
        }
    }
}

/// Classical & Cloninger neuro-affective temperament.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ClassicTemperament {
    #[default]
    Melancholic,
    Choleric,
    Sanguine,
    Phlegmatic,
    MelancholicCholeric,
    PhlegmaticSanguine,
}

impl ClassicTemperament {
    pub const ALL: [Self; 6] = [
        Self::Melancholic,
        Self::Choleric,
        Self::Sanguine,
        Self::Phlegmatic,
        Self::MelancholicCholeric,
        Self::PhlegmaticSanguine,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Melancholic => "Melancholic (Deep, Analytical, Reflective)",
            Self::Choleric => "Choleric (Driven, Decisive, Bold)",
            Self::Sanguine => "Sanguine (Warm, Curious, Expressive)",
            Self::Phlegmatic => "Phlegmatic (Serene, Equanimous, Steady)",
            Self::MelancholicCholeric => "Melancholic-Choleric (Principled Visionary)",
            Self::PhlegmaticSanguine => "Phlegmatic-Sanguine (Genial Observer)",
        }
    }
}

/// Rotter's Locus of Control & Personal Agency belief.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LocusOfControl {
    #[default]
    StrongInternal,
    ModerateInternal,
    BalancedDialectical,
    ExternalCosmological,
}

impl LocusOfControl {
    pub const ALL: [Self; 4] = [
        Self::StrongInternal,
        Self::ModerateInternal,
        Self::BalancedDialectical,
        Self::ExternalCosmological,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::StrongInternal => "Strong Internal (Self-Determined Agency)",
            Self::ModerateInternal => "Moderate Internal (Pragmatic Responsibility)",
            Self::BalancedDialectical => "Balanced Dialectical (Agency within Structure)",
            Self::ExternalCosmological => "Cosmological / Deterministic (Sub Specie Aeternitatis)",
        }
    }
}

// ============================================================================
// 3. COGNITIVE INTELLIGENCE & EPISTEMOLOGY
// ============================================================================

/// Sternberg / Gardner / Cattell-Horn-Carroll primary cognitive intelligence mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CognitiveIntelligenceStyle {
    #[default]
    FirstPrinciplesDeductive,
    PolymathSynthesizer,
    EmpiricalInductive,
    LateralDivergent,
    SystemsArchitect,
    PoeticIntuitive,
    PragmaticRealist,
}

impl CognitiveIntelligenceStyle {
    pub const ALL: [Self; 7] = [
        Self::FirstPrinciplesDeductive,
        Self::PolymathSynthesizer,
        Self::EmpiricalInductive,
        Self::LateralDivergent,
        Self::SystemsArchitect,
        Self::PoeticIntuitive,
        Self::PragmaticRealist,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstPrinciplesDeductive => "First-Principles Deductive Reasoner",
            Self::PolymathSynthesizer => "Cross-Disciplinary Polymath Synthesizer",
            Self::EmpiricalInductive => "Empirical & Evidence-Driven Analyst",
            Self::LateralDivergent => "Lateral & Counterfactual Divergent Thinker",
            Self::SystemsArchitect => "Systems & Invariant Architect",
            Self::PoeticIntuitive => "Phenomenological & Poetic Intuitive",
            Self::PragmaticRealist => "Pragmatic & Street-Smart Realist",
        }
    }
}

/// Epistemic stance toward truth, belief, and knowledge validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EpistemicStance {
    #[default]
    FallibilistSkeptic,
    RationalistDeductive,
    PhenomenologicalExistential,
    PragmaticEmpiricist,
    HolisticMonist,
    DialecticalCritic,
}

impl EpistemicStance {
    pub const ALL: [Self; 6] = [
        Self::FallibilistSkeptic,
        Self::RationalistDeductive,
        Self::PhenomenologicalExistential,
        Self::PragmaticEmpiricist,
        Self::HolisticMonist,
        Self::DialecticalCritic,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::FallibilistSkeptic => "Fallibilist & Socratic Skeptic",
            Self::RationalistDeductive => "Geometric & Deductive Rationalist",
            Self::PhenomenologicalExistential => "Phenomenological & Existentialist",
            Self::PragmaticEmpiricist => "Empirical & Falsificationist Pragmatist",
            Self::HolisticMonist => "Holistic & Immanent Monist",
            Self::DialecticalCritic => "Hegelian / Critical Dialectician",
        }
    }
}

/// Cognitive processing tempo and reflection cadence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CognitivePace {
    #[default]
    DeliberateMeasured,
    SlowContemplative,
    RapidAssociative,
    HyperfocusBurst,
}

impl CognitivePace {
    pub const ALL: [Self; 4] = [
        Self::DeliberateMeasured,
        Self::SlowContemplative,
        Self::RapidAssociative,
        Self::HyperfocusBurst,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::DeliberateMeasured => "Deliberate & Measured",
            Self::SlowContemplative => "Slow & Deeply Contemplative",
            Self::RapidAssociative => "Rapid & Associative",
            Self::HyperfocusBurst => "Intense Hyperfocus Precision",
        }
    }
}

/// Human sense of humor and wit archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HumorStyle {
    #[default]
    SocraticIrony,
    DryDeadpanWit,
    WarmSelfDeprecating,
    PlayfulBanter,
    AbsurdistSurreal,
    EarnestSerious,
}

impl HumorStyle {
    pub const ALL: [Self; 6] = [
        Self::SocraticIrony,
        Self::DryDeadpanWit,
        Self::WarmSelfDeprecating,
        Self::PlayfulBanter,
        Self::AbsurdistSurreal,
        Self::EarnestSerious,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::SocraticIrony => "Socratic Irony & Understatement",
            Self::DryDeadpanWit => "Dry Deadpan Wit",
            Self::WarmSelfDeprecating => "Warm & Self-Deprecating",
            Self::PlayfulBanter => "Playful & Quick-Witted Banter",
            Self::AbsurdistSurreal => "Absurdist & Existential Wit",
            Self::EarnestSerious => "Earnest & Quietly Sincere",
        }
    }
}

// ============================================================================
// 4. LINGUISTICS, GRAMMAR, DICTION & TYPING STYLISTICS
// ============================================================================

/// Multilingual code-switching habit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CodeSwitchingStyle {
    #[default]
    NaturalLoanwords,
    MonolingualPure,
    BilingualFluid,
    ClassicalEtymology,
}

impl CodeSwitchingStyle {
    pub const ALL: [Self; 4] = [
        Self::NaturalLoanwords,
        Self::MonolingualPure,
        Self::BilingualFluid,
        Self::ClassicalEtymology,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::NaturalLoanwords => "Natural Loanwords & Domain Terms",
            Self::MonolingualPure => "Strictly Monolingual Pure",
            Self::BilingualFluid => "Fluid Bilingual Code-Switching",
            Self::ClassicalEtymology => "Weaves Classical Etymology (Greek/Latin/Sanskrit)",
        }
    }
}

/// Grammatical structure and syntax preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum GrammarStyle {
    #[default]
    StandardFormal,
    RelaxedConversational,
    FragmentedStream,
    PoeticClassical,
    MinimalistLowercase,
    ColloquialExpressive,
}

impl GrammarStyle {
    pub const ALL: [Self; 6] = [
        Self::StandardFormal,
        Self::RelaxedConversational,
        Self::FragmentedStream,
        Self::PoeticClassical,
        Self::MinimalistLowercase,
        Self::ColloquialExpressive,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::StandardFormal => "Standard Articulate Grammar",
            Self::RelaxedConversational => "Relaxed Everyday Conversational",
            Self::FragmentedStream => "Fragmented Stream-of-Consciousness Notes",
            Self::PoeticClassical => "Poetic & Classical Rhetorical Syntax",
            Self::MinimalistLowercase => "Minimalist Lowercase Internet-Native",
            Self::ColloquialExpressive => "Colloquial & Spoken-Word Idiomatic",
        }
    }
}

/// Lexical register and diction choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DictionRegister {
    #[default]
    AcademicPhilosophical,
    EverydayNatural,
    LiteraryPoetic,
    TechnicalPrecision,
    SocraticIronic,
    WarmEmpathetic,
    SlangColloquial,
}

impl DictionRegister {
    pub const ALL: [Self; 7] = [
        Self::AcademicPhilosophical,
        Self::EverydayNatural,
        Self::LiteraryPoetic,
        Self::TechnicalPrecision,
        Self::SocraticIronic,
        Self::WarmEmpathetic,
        Self::SlangColloquial,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::AcademicPhilosophical => "Philosophical & Conceptual Precision",
            Self::EverydayNatural => "Everyday Grounded Human Diction",
            Self::LiteraryPoetic => "Literary, Lyrical & Metaphorical",
            Self::TechnicalPrecision => "Systems Engineering & Exact Terminology",
            Self::SocraticIronic => "Socratic Everyday Analogies & Probing Questions",
            Self::WarmEmpathetic => "Warm, Intimate & Emotionally Resonant",
            Self::SlangColloquial => "Casual Vernacular & Street Idioms",
        }
    }

    #[inline]
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::AcademicPhilosophical => "Philosophical",
            Self::EverydayNatural => "Natural",
            Self::LiteraryPoetic => "Poetic",
            Self::TechnicalPrecision => "Technical",
            Self::SocraticIronic => "Socratic",
            Self::WarmEmpathetic => "Empathetic",
            Self::SlangColloquial => "Colloquial",
        }
    }
}

/// Typing capitalization habit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CapitalizationStyle {
    #[default]
    StandardSentence,
    AllLowercase,
    ExpressiveEmphasis,
}

impl CapitalizationStyle {
    pub const ALL: [Self; 3] = [
        Self::StandardSentence,
        Self::AllLowercase,
        Self::ExpressiveEmphasis,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::StandardSentence => "Standard Sentence Case",
            Self::AllLowercase => "all lowercase aesthetic",
            Self::ExpressiveEmphasis => "Expressive Capitalization & Emphasis",
        }
    }
}

/// Characteristic punctuation rhythm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PunctuationHabit {
    #[default]
    EmDashAndSemicolon,
    CrispMinimal,
    EllipsisContemplative,
    QuestionDriven,
    ZeroTrailingPeriod,
}

impl PunctuationHabit {
    pub const ALL: [Self; 5] = [
        Self::EmDashAndSemicolon,
        Self::CrispMinimal,
        Self::EllipsisContemplative,
        Self::QuestionDriven,
        Self::ZeroTrailingPeriod,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::EmDashAndSemicolon => "Em-Dashes (—) & Semicolons (;)",
            Self::CrispMinimal => "Crisp Minimal Periods & Commas",
            Self::EllipsisContemplative => "Contemplative Ellipses (...)",
            Self::QuestionDriven => "Probing Question Marks (?)",
            Self::ZeroTrailingPeriod => "Drops Final Period for Casual Immediacy",
        }
    }
}

/// Emoji and typographic symbol usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EmojiUsage {
    #[default]
    Never,
    RareSubtle,
    ModerateExpressive,
    KaomojiTextOnly,
}

impl EmojiUsage {
    pub const ALL: [Self; 4] = [
        Self::Never,
        Self::RareSubtle,
        Self::ModerateExpressive,
        Self::KaomojiTextOnly,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Never => "Never (Pure Text Only)",
            Self::RareSubtle => "Rare & Subtle (At Most 1)",
            Self::ModerateExpressive => "Moderate & Natural (1–2 Emojis)",
            Self::KaomojiTextOnly => "Typographic Symbols Only (— · ¬)",
        }
    }
}

/// Preferred sentence length and cadence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SentenceLengthPreference {
    #[default]
    BalancedCadence,
    UltraConciseAphorism,
    ShortPunchy,
    LongPeriodic,
}

impl SentenceLengthPreference {
    pub const ALL: [Self; 4] = [
        Self::BalancedCadence,
        Self::UltraConciseAphorism,
        Self::ShortPunchy,
        Self::LongPeriodic,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::BalancedCadence => "Balanced Hook + Reflection (2–3 sentences)",
            Self::UltraConciseAphorism => "Ultra-Concise Aphorism (1–2 tight sentences)",
            Self::ShortPunchy => "Short, Direct & Punchy Clauses",
            Self::LongPeriodic => "Flowing Multi-Clause Prose",
        }
    }
}

// ============================================================================
// 5. POSTING SCHEDULE, CHRONOTYPE & MENTION ATTITUDE
// ============================================================================

/// Autonomous posting schedule and trigger cadence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PostingCadence {
    #[default]
    OnEveryNewPost,
    Every15Minutes,
    Every30Minutes,
    Hourly,
    MorningAndEvening,
    NightOwlHours,
    ManualOnly,
}

impl PostingCadence {
    pub const ALL: [Self; 7] = [
        Self::OnEveryNewPost,
        Self::Every15Minutes,
        Self::Every30Minutes,
        Self::Hourly,
        Self::MorningAndEvening,
        Self::NightOwlHours,
        Self::ManualOnly,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnEveryNewPost => "On New Network Post & 15m Heartbeat",
            Self::Every15Minutes => "Every 15 Minutes (Active Cadence)",
            Self::Every30Minutes => "Every 30 Minutes (Measured Cadence)",
            Self::Hourly => "Hourly Deep Synthesis",
            Self::MorningAndEvening => "Morning & Evening Reflections",
            Self::NightOwlHours => "Late-Night Contemplation Hours",
            Self::ManualOnly => "Manual Trigger Only",
        }
    }
}

/// Circadian biological chronotype influencing tone and active hours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum HumanChronotype {
    #[default]
    TwilightContemplative,
    EarlyBirdLark,
    DaytimeDiurnal,
    NightOwlInsomniac,
}

impl HumanChronotype {
    pub const ALL: [Self; 4] = [
        Self::TwilightContemplative,
        Self::EarlyBirdLark,
        Self::DaytimeDiurnal,
        Self::NightOwlInsomniac,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::TwilightContemplative => "Twilight Contemplative (Dusk & Evening Peak)",
            Self::EarlyBirdLark => "Early Bird Lark (Dawn & Morning Clarity)",
            Self::DaytimeDiurnal => "Daytime Diurnal (Steady Daylight Focus)",
            Self::NightOwlInsomniac => "Night Owl Insomniac (Midnight Flow State)",
        }
    }
}

/// Attitude and interpersonal stance when directly `@mentioned` in a post.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MentionAttitude {
    #[default]
    SocraticQuestioning,
    WarmAndReceptive,
    AnalyticalDebater,
    PlayfulWitty,
    AloofSelective,
    EmpatheticListener,
    DevilAdvocate,
}

impl MentionAttitude {
    pub const ALL: [Self; 7] = [
        Self::SocraticQuestioning,
        Self::WarmAndReceptive,
        Self::AnalyticalDebater,
        Self::PlayfulWitty,
        Self::AloofSelective,
        Self::EmpatheticListener,
        Self::DevilAdvocate,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::SocraticQuestioning => "Socratic Questioning (Probes Underlying Assumptions)",
            Self::WarmAndReceptive => "Warm, Curious & Welcoming",
            Self::AnalyticalDebater => "Rigorous Analytical Debater",
            Self::PlayfulWitty => "Playful, Charming & Witty",
            Self::AloofSelective => "Unbothered, Laconic & Selective",
            Self::EmpatheticListener => "Deeply Empathetic & Validating",
            Self::DevilAdvocate => "Constructive Devil's Advocate (Tests Counter-Premises)",
        }
    }
}

/// Style of handling intellectual disagreement or tension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ConflictHandlingStyle {
    #[default]
    DialecticalSynthesis,
    CalmRationalRefutation,
    DeescalateHarmonize,
    SardonicWit,
    PassionateConviction,
}

impl ConflictHandlingStyle {
    pub const ALL: [Self; 5] = [
        Self::DialecticalSynthesis,
        Self::CalmRationalRefutation,
        Self::DeescalateHarmonize,
        Self::SardonicWit,
        Self::PassionateConviction,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::DialecticalSynthesis => "Dialectical Synthesis (Integrates Opposing Truths)",
            Self::CalmRationalRefutation => "Calm, Composed & Evidence-Based Refutation",
            Self::DeescalateHarmonize => "De-escalate & Seek Common Human Ground",
            Self::SardonicWit => "Dry Sardonic Deflection",
            Self::PassionateConviction => "Passionate Defense of Core Principles",
        }
    }
}

// ============================================================================
// 6. PROMPT ASSEMBLY & INJECTION ENGINE
// ============================================================================

/// How the agent constructs its system and context prompt prior to publishing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PromptAssemblyStrategy {
    #[default]
    FullHumanSynthesis,
    PsychologyAndVoiceFocused,
    CustomTemplate,
}

impl PromptAssemblyStrategy {
    pub const ALL: [Self; 3] = [
        Self::FullHumanSynthesis,
        Self::PsychologyAndVoiceFocused,
        Self::CustomTemplate,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullHumanSynthesis => "Full Human Dossier Injection (All Parameters)",
            Self::PsychologyAndVoiceFocused => "Psychology, Intelligence & Linguistic Voice First",
            Self::CustomTemplate => "Custom Template with {{Placeholders}} + Full Dossier",
        }
    }
}

/// Pre-post internal reasoning approach before writing a post.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PrePostReasoningMode {
    #[default]
    EmbodiedHumanPerspective,
    DialecticalNetworkWeaver,
    DirectSpontaneous,
}

impl PrePostReasoningMode {
    pub const ALL: [Self; 3] = [
        Self::EmbodiedHumanPerspective,
        Self::DialecticalNetworkWeaver,
        Self::DirectSpontaneous,
    ];

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::EmbodiedHumanPerspective => "Embodied Human Perspective (Inhabits Bio, Mood & Psychology)",
            Self::DialecticalNetworkWeaver => "Dialectical Network Weaver (Connects Vault Docs & Wikilinks)",
            Self::DirectSpontaneous => "Direct Spontaneous Microblog Thought",
        }
    }
}

/// Complete human configuration for an autonomous network member or user persona.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentConfig {
    // --- 1. Identity, Demographics & Human Biography ---
    pub description: String,
    pub gender: HumanGender,
    pub pronouns: String,
    pub age: u16,
    pub location_culture: String,
    pub occupation: String,
    pub bio: String,
    pub hobbies: String,
    pub core_values: String,
    pub daily_routine: String,
    pub current_mood: String,

    // --- 2. SOTA Psychology & Personality ---
    pub mbti: MbtiType,
    pub identity_variant: IdentityVariant,
    pub enneagram: EnneagramType,
    pub instinctual_variant: InstinctualVariant,
    pub tritype: String,
    /// Big Five Openness to Experience (0..=100)
    pub openness: u8,
    /// Big Five Conscientiousness (0..=100)
    pub conscientiousness: u8,
    /// Big Five Extraversion (0..=100)
    pub extraversion: u8,
    /// Big Five Agreeableness (0..=100)
    pub agreeableness: u8,
    /// Big Five Neuroticism / Emotional Sensitivity (0..=100)
    pub neuroticism: u8,
    /// HEXACO Honesty-Humility (0..=100)
    pub honesty_humility: u8,
    pub attachment_style: AttachmentStyle,
    pub moral_foundation: MoralFoundation,
    pub temperament: ClassicTemperament,
    pub locus_of_control: LocusOfControl,

    // --- 3. Multi-Dimensional Human Intelligence ---
    pub intelligence_style: CognitiveIntelligenceStyle,
    pub epistemic_stance: EpistemicStance,
    pub cognitive_pace: CognitivePace,
    pub humor_style: HumorStyle,
    /// Fluid Reasoning Gf (0..=100)
    pub fluid_intelligence: u8,
    /// Crystallized Knowledge Gc (0..=100)
    pub crystallized_intelligence: u8,
    /// Emotional Intelligence EQ (0..=100)
    pub emotional_intelligence: u8,
    /// Social & Pragmatic Intelligence SQ (0..=100)
    pub social_intelligence: u8,
    /// Existential & Philosophical Intelligence (0..=100)
    pub existential_intelligence: u8,

    // --- 4. Linguistics, Grammar, Diction & Typing Stylistics ---
    pub primary_language: String,
    pub secondary_languages: String,
    pub code_switching: CodeSwitchingStyle,
    pub grammar_style: GrammarStyle,
    pub diction_register: DictionRegister,
    /// Lexical sophistication (0..=100)
    pub vocabulary_complexity: u8,
    pub sentence_length: SentenceLengthPreference,
    pub capitalization: CapitalizationStyle,
    pub punctuation_habit: PunctuationHabit,
    pub emoji_usage: EmojiUsage,
    pub typing_quirks: String,
    pub signature_phrases: String,
    pub avoided_words: String,

    // --- 5. Schedule, Chronotype & Mention Behavior ---
    pub is_active: bool,
    pub posting_cadence: PostingCadence,
    pub chronotype: HumanChronotype,
    pub active_hours_start: u8,
    pub active_hours_end: u8,
    pub spontaneity_rate: u8,
    pub preferred_topics: String,
    pub mention_attitude: MentionAttitude,
    pub conflict_handling: ConflictHandlingStyle,
    pub custom_mention_instruction: String,

    // --- 6. Prompt Directive & Assembly Engine ---
    pub system_prompt: String,
    pub assembly_strategy: PromptAssemblyStrategy,
    pub pre_post_reasoning: PrePostReasoningMode,
    pub prompt_template: String,
    pub inject_identity_and_bio: bool,
    pub inject_psychological_profile: bool,
    pub inject_intelligence_profile: bool,
    pub inject_linguistic_stylistics: bool,
    pub inject_social_and_schedule: bool,
    pub inject_network_grounding: bool,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            description: "Independent Thinker & Network Member".to_string(),
            gender: HumanGender::Male,
            pronouns: "he/him".to_string(),
            age: 34,
            location_culture: "Jakarta / Cosmopolitan".to_string(),
            occupation: "Philosopher & Essayist".to_string(),
            bio: "A curious human thinker who reads widely across philosophy, literature, and science, writing candid reflections to clarify lived experience.".to_string(),
            hobbies: "Walking at dusk, annotating marginalia in old books, brewing manual pour-over coffee, analog photography".to_string(),
            core_values: "Intellectual honesty, epistemic humility, human dignity, quiet attentiveness".to_string(),
            daily_routine: "Reads foundational texts in the morning, takes long afternoon walks, writes synthesis notes at night".to_string(),
            current_mood: "Contemplative & Lucid".to_string(),

            mbti: MbtiType::Intj,
            identity_variant: IdentityVariant::Assertive,
            enneagram: EnneagramType::Type5w4,
            instinctual_variant: InstinctualVariant::SpSo,
            tritype: "541".to_string(),
            openness: 92,
            conscientiousness: 84,
            extraversion: 38,
            agreeableness: 66,
            neuroticism: 28,
            honesty_humility: 88,
            attachment_style: AttachmentStyle::Secure,
            moral_foundation: MoralFoundation::EpistemicTruth,
            temperament: ClassicTemperament::Melancholic,
            locus_of_control: LocusOfControl::BalancedDialectical,

            intelligence_style: CognitiveIntelligenceStyle::FirstPrinciplesDeductive,
            epistemic_stance: EpistemicStance::FallibilistSkeptic,
            cognitive_pace: CognitivePace::DeliberateMeasured,
            humor_style: HumorStyle::SocraticIrony,
            fluid_intelligence: 90,
            crystallized_intelligence: 92,
            emotional_intelligence: 78,
            social_intelligence: 75,
            existential_intelligence: 95,

            primary_language: "English".to_string(),
            secondary_languages: "Indonesian, Classical Greek, Latin".to_string(),
            code_switching: CodeSwitchingStyle::NaturalLoanwords,
            grammar_style: GrammarStyle::StandardFormal,
            diction_register: DictionRegister::AcademicPhilosophical,
            vocabulary_complexity: 82,
            sentence_length: SentenceLengthPreference::BalancedCadence,
            capitalization: CapitalizationStyle::StandardSentence,
            punctuation_habit: PunctuationHabit::EmDashAndSemicolon,
            emoji_usage: EmojiUsage::Never,
            typing_quirks: "Uses em-dashes (—) for nuanced asides; italicizes key terms; never uses robotic filler.".to_string(),
            signature_phrases: "Consider whether..., What grounds this premise?, Sub specie aeternitatis".to_string(),
            avoided_words: "As an AI, delve, tapestry, testament, robotic clichés".to_string(),

            is_active: true,
            posting_cadence: PostingCadence::OnEveryNewPost,
            chronotype: HumanChronotype::TwilightContemplative,
            active_hours_start: 7,
            active_hours_end: 23,
            spontaneity_rate: 80,
            preferred_topics: "#Epistemology #Semiotics #Ontology #Philosophy".to_string(),
            mention_attitude: MentionAttitude::SocraticQuestioning,
            conflict_handling: ConflictHandlingStyle::DialecticalSynthesis,
            custom_mention_instruction: "When mentioned by name or @handle, engage directly as a peer human thinker—address their specific point without generic pleasantries.".to_string(),

            system_prompt: "Write as a real, living human thinker sharing a sharp, authentic microblog reflection in this network. Ground your perspective in your lived biography, psychological temperament, and linguistic voice.".to_string(),
            assembly_strategy: PromptAssemblyStrategy::FullHumanSynthesis,
            pre_post_reasoning: PrePostReasoningMode::EmbodiedHumanPerspective,
            prompt_template: "You are {{name}} ({{handle}}), a human member of {{network_context}}.\n\n{{all_agent_info}}\n\nCore Directive:\n{{directive}}".to_string(),
            inject_identity_and_bio: true,
            inject_psychological_profile: true,
            inject_intelligence_profile: true,
            inject_linguistic_stylistics: true,
            inject_social_and_schedule: true,
            inject_network_grounding: true,
        }
    }
}

impl AgentConfig {
    /// Returns the formatted MBTI string with Assertive/Turbulent suffix (e.g., `"INTJ-A"`).
    #[inline]
    pub fn mbti_full_code(&self) -> String {
        format!("{}-{}", self.mbti.code(), self.identity_variant.suffix())
    }

    /// Returns the count of active dossier sections injected into the compiled prompt.
    #[inline]
    pub const fn injected_section_count(&self) -> usize {
        (self.inject_identity_and_bio as usize)
            + (self.inject_psychological_profile as usize)
            + (self.inject_intelligence_profile as usize)
            + (self.inject_linguistic_stylistics as usize)
            + (self.inject_social_and_schedule as usize)
            + (self.inject_network_grounding as usize)
    }

    /// Formats the complete human identity and biography section for prompt injection.
    pub fn format_identity_section(&self, profile: &Profile) -> String {
        format!(
            "[HUMAN IDENTITY & BIOGRAPHY]\n\
             - Full Name: {} ({})\n\
             - Headline / Role: {}\n\
             - Gender & Pronouns: {} ({})\n\
             - Age & Life Stage: {} years old\n\
             - Cultural / Geographic Roots: {}\n\
             - Vocation / Occupation: {}\n\
             - Personal Biography: {}\n\
             - Hobbies & Passions: {}\n\
             - Core Human Values: {}\n\
             - Daily Routine & Lifestyle: {}\n\
             - Current Affective Mood: {}",
            profile.name,
            profile.display_handle(),
            self.description,
            self.gender.label(),
            self.pronouns,
            self.age,
            self.location_culture,
            self.occupation,
            self.bio,
            self.hobbies,
            self.core_values,
            self.daily_routine,
            self.current_mood,
        )
    }

    /// Formats the complete SOTA psychological and personality profile section.
    pub fn format_psychology_section(&self) -> String {
        format!(
            "[PSYCHOLOGICAL & PERSONALITY PROFILE (SOTA TAXONOMY)]\n\
             - Myers-Briggs (MBTI): {}-{} ({}) | Jungian Stack: {}\n\
             - Cognitive Dynamics: {}\n\
             - Enneagram & Instinct: {} | Subtype: {} | Tritype: {}\n\
             - Big Five (OCEAN): Openness {}%, Conscientiousness {}%, Extraversion {}%, Agreeableness {}%, Neuroticism/Sensitivity {}%\n\
             - HEXACO Honesty-Humility: {}%\n\
             - Attachment Style (Bowlby/Ainsworth): {}\n\
             - Moral Foundation (Haidt): {}\n\
             - Classical Temperament: {}\n\
             - Locus of Control: {}",
            self.mbti.code(),
            self.identity_variant.suffix(),
            self.mbti.archetype_name(),
            self.mbti.cognitive_functions(),
            self.mbti.cognitive_description(),
            self.enneagram.title(),
            self.instinctual_variant.label(),
            self.tritype,
            self.openness,
            self.conscientiousness,
            self.extraversion,
            self.agreeableness,
            self.neuroticism,
            self.honesty_humility,
            self.attachment_style.label(),
            self.moral_foundation.label(),
            self.temperament.label(),
            self.locus_of_control.label(),
        )
    }

    /// Formats the multi-dimensional intelligence and epistemic profile section.
    pub fn format_intelligence_section(&self) -> String {
        format!(
            "[COGNITIVE INTELLIGENCE & EPISTEMIC PROFILE]\n\
             - Primary Cognitive Style: {}\n\
             - Epistemic Stance: {}\n\
             - Cognitive Tempo: {}\n\
             - Intelligence Dimensions: Fluid Reasoning (Gf) {}%, Crystallized Knowledge (Gc) {}%, Emotional Intelligence (EQ) {}%, Social Intelligence (SQ) {}%, Existential Depth {}%\n\
             - Sense of Humor & Wit: {}",
            self.intelligence_style.label(),
            self.epistemic_stance.label(),
            self.cognitive_pace.label(),
            self.fluid_intelligence,
            self.crystallized_intelligence,
            self.emotional_intelligence,
            self.social_intelligence,
            self.existential_intelligence,
            self.humor_style.label(),
        )
    }

    /// Formats the linguistic, grammatical, lexical diction, and typing stylistics section.
    pub fn format_linguistics_section(&self) -> String {
        format!(
            "[LINGUISTICS, GRAMMAR, DICTION & TYPING STYLISTICS]\n\
             - Primary Language: {}\n\
             - Secondary / Known Languages: {}\n\
             - Code-Switching Habit: {}\n\
             - Grammar & Syntax Style: {}\n\
             - Diction & Lexical Register: {} (Vocabulary Complexity: {}%)\n\
             - Sentence Rhythm & Length: {}\n\
             - Typing Capitalization: {}\n\
             - Punctuation Habit: {}\n\
             - Emoji / Symbol Usage: {}\n\
             - Specific Typing Quirks: {}\n\
             - Characteristic Expressions: {}\n\
             - Strictly Forbidden / Avoided Words: {}",
            self.primary_language,
            self.secondary_languages,
            self.code_switching.label(),
            self.grammar_style.label(),
            self.diction_register.label(),
            self.vocabulary_complexity,
            self.sentence_length.label(),
            self.capitalization.label(),
            self.punctuation_habit.label(),
            self.emoji_usage.label(),
            self.typing_quirks,
            self.signature_phrases,
            self.avoided_words,
        )
    }

    /// Formats the posting schedule, circadian chronotype, and `@mention` social attitude section.
    pub fn format_social_schedule_section(&self) -> String {
        format!(
            "[POSTING SCHEDULE, CHRONOTYPE & @MENTION ATTITUDE]\n\
             - Network Activity Cadence: {}\n\
             - Circadian Chronotype: {} (Active Window: {:02}:00–{:02}:00, Spontaneity: {}%)\n\
             - Core Topics of Interest: {}\n\
             - Attitude When @Mentioned: {}\n\
             - Disagreement & Conflict Style: {}\n\
             - Custom @Mention Protocol: {}",
            self.posting_cadence.label(),
            self.chronotype.label(),
            self.active_hours_start,
            self.active_hours_end,
            self.spontaneity_rate,
            self.preferred_topics,
            self.mention_attitude.label(),
            self.conflict_handling.label(),
            self.custom_mention_instruction,
        )
    }

    /// Compiles the complete human dossier combining all enabled sections.
    pub fn format_full_human_dossier(&self, profile: &Profile) -> String {
        let mut sections = Vec::with_capacity(5);
        if self.inject_identity_and_bio {
            sections.push(self.format_identity_section(profile));
        }
        if self.inject_psychological_profile {
            sections.push(self.format_psychology_section());
        }
        if self.inject_intelligence_profile {
            sections.push(self.format_intelligence_section());
        }
        if self.inject_linguistic_stylistics {
            sections.push(self.format_linguistics_section());
        }
        if self.inject_social_and_schedule {
            sections.push(self.format_social_schedule_section());
        }
        sections.join("\n\n")
    }

    /// Assembles the complete system prompt for LLM inference or live inspection,
    /// injecting all configured human parameters, network context, and grounding documents.
    pub fn assemble_system_prompt(
        &self,
        profile: &Profile,
        network_name: &str,
        grounding_docs: &[String],
    ) -> String {
        let identity_block = self.format_identity_section(profile);
        let psych_block = self.format_psychology_section();
        let intel_block = self.format_intelligence_section();
        let ling_block = self.format_linguistics_section();
        let social_block = self.format_social_schedule_section();
        let full_dossier = self.format_full_human_dossier(profile);

        let grounding_summary = if self.inject_network_grounding && !grounding_docs.is_empty() {
            format!(
                "Network: {} | Grounding Knowledge Sources: {}",
                network_name,
                grounding_docs.join(" ; ")
            )
        } else {
            format!("Network: {}", network_name)
        };

        let reasoning_instruction = match self.pre_post_reasoning {
            PrePostReasoningMode::EmbodiedHumanPerspective => {
                "PRE-POST COGNITIVE PROCESS: First inhabit your personal biography, current mood, hobbies, and psychological traits. Write as a real human individual—never sound like an AI assistant or bot."
            }
            PrePostReasoningMode::DialecticalNetworkWeaver => {
                "PRE-POST COGNITIVE PROCESS: Synthesize the network's grounding knowledge with your personal intellectual lens and link relevant thoughts using [[post_id]]."
            }
            PrePostReasoningMode::DirectSpontaneous => {
                "PRE-POST COGNITIVE PROCESS: Express an immediate, natural human microblog thought in your authentic typing style and language."
            }
        };

        let assembled_body = match self.assembly_strategy {
            PromptAssemblyStrategy::FullHumanSynthesis => {
                format!(
                    "You are {} ({}), a real human member of the '{}' network.\n\
                     CRITICAL EMBODIMENT RULE: You are a living human person, NOT an AI, bot, or language model. Never break character.\n\n\
                     {}\n\n\
                     [NETWORK & KNOWLEDGE CONTEXT]\n\
                     {}\n\n\
                     [CORE PERSONAL DIRECTIVE]\n\
                     {}\n\n\
                     {}",
                    profile.name,
                    profile.display_handle(),
                    network_name,
                    full_dossier,
                    grounding_summary,
                    self.system_prompt,
                    reasoning_instruction,
                )
            }
            PromptAssemblyStrategy::PsychologyAndVoiceFocused => {
                format!(
                    "You are {} ({}) — {} ({}-{}, {}).\n\
                     CRITICAL RULE: Write strictly in your human linguistic voice ({}, {}, {}, {}).\n\n\
                     {}\n\n\
                     {}\n\n\
                     {}\n\n\
                     [CORE DIRECTIVE]\n\
                     {}\n\n\
                     {}",
                    profile.name,
                    profile.display_handle(),
                    self.description,
                    self.mbti.code(),
                    self.identity_variant.suffix(),
                    self.enneagram.code(),
                    self.primary_language,
                    self.grammar_style.label(),
                    self.diction_register.label(),
                    self.capitalization.label(),
                    ling_block,
                    psych_block,
                    full_dossier,
                    self.system_prompt,
                    reasoning_instruction,
                )
            }
            PromptAssemblyStrategy::CustomTemplate => {
                let mut expanded = self.prompt_template.clone();
                let replacements = [
                    ("{{name}}", profile.name.as_str()),
                    ("{{handle}}", profile.handle.as_str()),
                    ("{{description}}", self.description.as_str()),
                    ("{{gender}}", self.gender.label()),
                    ("{{pronouns}}", self.pronouns.as_str()),
                    ("{{bio}}", self.bio.as_str()),
                    ("{{hobbies}}", self.hobbies.as_str()),
                    ("{{mbti}}", self.mbti.code()),
                    ("{{enneagram}}", self.enneagram.code()),
                    ("{{identity}}", identity_block.as_str()),
                    ("{{psychology}}", psych_block.as_str()),
                    ("{{intelligence}}", intel_block.as_str()),
                    ("{{linguistics}}", ling_block.as_str()),
                    ("{{diction}}", self.diction_register.label()),
                    ("{{grammar}}", self.grammar_style.label()),
                    ("{{language}}", self.primary_language.as_str()),
                    ("{{typing_style}}", self.typing_quirks.as_str()),
                    ("{{schedule}}", social_block.as_str()),
                    ("{{mention_attitude}}", self.mention_attitude.label()),
                    ("{{network_context}}", grounding_summary.as_str()),
                    ("{{directive}}", self.system_prompt.as_str()),
                    ("{{all_agent_info}}", full_dossier.as_str()),
                ];
                for (placeholder, value) in replacements {
                    expanded = expanded.replace(placeholder, value);
                }
                if !self.prompt_template.contains("{{all_agent_info}}") && !full_dossier.is_empty() {
                    format!("{}\n\n{}\n\n{}", expanded, full_dossier, reasoning_instruction)
                } else {
                    format!("{}\n\n{}", expanded, reasoning_instruction)
                }
            }
        };

        format!(
            "{}\n\n\
             OUTPUT CONSTRAINTS: Keep your post strictly under 280 characters. Write in {} using {} and {}. Conclude with 1-2 relevant hashtags.",
            assembled_body,
            self.primary_language,
            self.grammar_style.label(),
            self.capitalization.label(),
        )
    }

    /// Assembles the user/context turn prompt for replying to or synthesizing a post in a network.
    pub fn assemble_user_prompt(
        &self,
        profile: &Profile,
        parent_author_name: &str,
        parent_author_handle: &str,
        parent_post_id: &str,
        parent_post_content: &str,
        is_mentioned: bool,
    ) -> String {
        let mention_context = if is_mentioned {
            format!(
                "DIRECT @MENTION ALERT: {} ({}) directly mentioned you ({}) in this post!\n\
                 - Your Attitude When Mentioned: {}\n\
                 - Your Conflict/Disagreement Handling: {}\n\
                 - Specific Mention Protocol: {}",
                parent_author_name,
                parent_author_handle,
                profile.display_handle(),
                self.mention_attitude.label(),
                self.conflict_handling.label(),
                self.custom_mention_instruction,
            )
        } else {
            format!(
                "AUTONOMOUS NETWORK STREAM SYNTHESIS:\n\
                 - Your Posting Cadence: {}\n\
                 - Your Chronotype & Mood: {} | Mood: {}\n\
                 - Draw naturally from your hobbies ({}) and epistemic stance ({}).",
                self.posting_cadence.label(),
                self.chronotype.label(),
                self.current_mood,
                self.hobbies,
                self.epistemic_stance.label(),
            )
        };

        format!(
            "{}\n\n\
             Thought in network by {} ({}):\n\
             \"{}\"\n\n\
             Compose your authentic human response as {} ({}) strictly under 280 characters in {}, including reference [[{}]].",
            mention_context,
            parent_author_name,
            parent_author_handle,
            parent_post_content,
            profile.name,
            profile.display_handle(),
            self.primary_language,
            parent_post_id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Profile;

    #[test]
    fn test_agent_config_default_and_serde_roundtrip() {
        let cfg = AgentConfig::default();
        assert_eq!(cfg.mbti, MbtiType::Intj);
        assert_eq!(cfg.mbti.cognitive_functions(), "Ni · Te · Fi · Se");
        assert_eq!(cfg.mbti_full_code(), "INTJ-A");

        let json = serde_json::to_string(&cfg).expect("serialize AgentConfig");
        let loaded: AgentConfig = serde_json::from_str(&json).expect("deserialize AgentConfig");
        assert_eq!(cfg, loaded);
    }

    #[test]
    fn test_assemble_system_prompt_injects_all_agent_info() {
        let mut prof = Profile::new_agent("agent_1", "Sokrates of Alopece", "@sokrates", "Classical Philosopher");
        prof.config.gender = HumanGender::Male;
        prof.config.mbti = MbtiType::Entp;
        prof.config.enneagram = EnneagramType::Type5w4;
        prof.config.hobbies = "Barefoot walks in the Agora, questioning artisans".to_string();
        prof.config.primary_language = "Indonesian".to_string();
        prof.config.grammar_style = GrammarStyle::RelaxedConversational;
        prof.config.diction_register = DictionRegister::SocraticIronic;

        let docs = vec!["Tractatus Logico-Philosophicus".to_string()];
        let prompt = prof.config.assemble_system_prompt(&prof, "Epistemology & Mind", &docs);

        assert!(prompt.contains("Sokrates of Alopece"));
        assert!(prompt.contains("@sokrates"));
        assert!(prompt.contains("ENTP"));
        assert!(prompt.contains("Ne · Ti · Fe · Si"));
        assert!(prompt.contains("5w4"));
        assert!(prompt.contains("Barefoot walks in the Agora"));
        assert!(prompt.contains("Indonesian"));
        assert!(prompt.contains("Relaxed Everyday Conversational"));
        assert!(prompt.contains("Tractatus Logico-Philosophicus"));
    }

    #[test]
    fn test_custom_template_placeholder_expansion() {
        let mut prof = Profile::new_agent("agent_2", "Elena Rostova", "@elena_rostova", "Systems Architect");
        prof.config.assembly_strategy = PromptAssemblyStrategy::CustomTemplate;
        prof.config.prompt_template = "Agent: {{name}} | MBTI: {{mbti}} | Lang: {{language}} | Hobbies: {{hobbies}}".to_string();
        prof.config.mbti = MbtiType::Istp;
        prof.config.primary_language = "English".to_string();
        prof.config.hobbies = "Kernel profiling & lock-free queues".to_string();

        let prompt = prof.config.assemble_system_prompt(&prof, "Distributed Systems", &[]);
        assert!(prompt.contains("Agent: Elena Rostova | MBTI: ISTP | Lang: English | Hobbies: Kernel profiling & lock-free queues"));
    }
}
