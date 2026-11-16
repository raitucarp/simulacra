//! Myers-Briggs archetypes and the Assertive/Turbulent identity modifier.

mod functions;

use crate::domain::trait_option::{trait_enum, trait_table};
use std::fmt;

trait_enum! {
    /// Myers-Briggs Type Indicator (16 Jungian Cognitive Archetypes).
    pub enum MbtiType in "mbti" {
        #[default]
        Intj("intj", "INTJ"),
        Intp("intp", "INTP"),
        Entj("entj", "ENTJ"),
        Entp("entp", "ENTP"),
        Infj("infj", "INFJ"),
        Infp("infp", "INFP"),
        Enfj("enfj", "ENFJ"),
        Enfp("enfp", "ENFP"),
        Istj("istj", "ISTJ"),
        Isfj("isfj", "ISFJ"),
        Estj("estj", "ESTJ"),
        Esfj("esfj", "ESFJ"),
        Istp("istp", "ISTP"),
        Isfp("isfp", "ISFP"),
        Estp("estp", "ESTP"),
        Esfp("esfp", "ESFP"),
    }
}

impl MbtiType {
    /// Four-letter type code (e.g. `"INTJ"`); identical to the label.
    #[inline]
    pub const fn code(self) -> &'static str {
        self.label()
    }
}

trait_table! {
    MbtiType::archetype_name {
        Intj => "Architect",
        Intp => "Logician",
        Entj => "Commander",
        Entp => "Dialectician",
        Infj => "Advocate",
        Infp => "Mediator",
        Enfj => "Protagonist",
        Enfp => "Catalyst",
        Istj => "Archivist",
        Isfj => "Steward",
        Estj => "Organizer",
        Esfj => "Harmonizer",
        Istp => "Craftsperson",
        Isfp => "Aesthete",
        Estp => "Dynamo",
        Esfp => "Performer",
    }
}

impl fmt::Display for MbtiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.code(), self.archetype_name())
    }
}

trait_enum! {
    /// Assertive (-A) vs Turbulent (-T) identity modifier.
    pub enum IdentityVariant in "identity_variant" {
        #[default]
        Assertive("assertive", "Assertive (-A)"),
        Turbulent("turbulent", "Turbulent (-T)"),
    }
}

trait_table! {
    IdentityVariant::suffix {
        Assertive => "A",
        Turbulent => "T",
    }
}
