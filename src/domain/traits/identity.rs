//! 1. Human identity & demographics.

use crate::domain::trait_option::{trait_enum, trait_table};

trait_enum! {
    /// Human gender identity classification.
    pub enum HumanGender in "gender" with display {
        #[default]
        Male("male", "Male"),
        Female("female", "Female"),
        NonBinary("non_binary", "Non-Binary"),
        Custom("custom", "Custom / Unspecified", short: "Custom"),
    }
}

trait_table! {
    HumanGender::default_pronouns {
        Male => "he/him",
        Female => "she/her",
        NonBinary => "they/them",
        Custom => "ia/dia",
    }
}
