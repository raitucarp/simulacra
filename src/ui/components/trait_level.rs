//! Semantic interpretation bands for 0..=100 trait scores.

/// Upper bound of a trait score.
pub const TRAIT_MAX: u8 = 100;

/// Lower bounds (inclusive) of the semantic score bands.
const EXCEPTIONAL_MIN: u8 = 90;
const HIGH_MIN: u8 = 75;
const BALANCED_MIN: u8 = 45;
const MODERATE_LOW_MIN: u8 = 25;

const LABEL_EXCEPTIONAL: &str = "Exceptional";
const LABEL_HIGH: &str = "High";
const LABEL_BALANCED: &str = "Balanced";
const LABEL_MODERATE_LOW: &str = "Moderate-Low";
const LABEL_RESERVED: &str = "Reserved";

/// Returns a human-friendly semantic interpretation label for a 0..=100 trait score.
/// Out-of-range scores (above [`TRAIT_MAX`]) fall back to the lowest band.
pub const fn trait_semantic_label(value: u8) -> &'static str {
    if value > TRAIT_MAX {
        LABEL_RESERVED
    } else if value >= EXCEPTIONAL_MIN {
        LABEL_EXCEPTIONAL
    } else if value >= HIGH_MIN {
        LABEL_HIGH
    } else if value >= BALANCED_MIN {
        LABEL_BALANCED
    } else if value >= MODERATE_LOW_MIN {
        LABEL_MODERATE_LOW
    } else {
        LABEL_RESERVED
    }
}
