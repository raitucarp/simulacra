use super::{calm, warmth, MAX_HEARTS};
use crate::domain::{ConflictHandlingStyle, MentionAttitude};

#[test]
fn every_attitude_and_conflict_style_fills_one_to_five_hearts() {
    for attitude in MentionAttitude::ALL {
        assert!((1..=MAX_HEARTS).contains(&warmth(attitude)), "{attitude:?}");
    }
    for style in ConflictHandlingStyle::ALL {
        assert!((1..=MAX_HEARTS).contains(&calm(style)), "{style:?}");
    }
}

#[test]
fn warm_listeners_outshine_aloof_ones_and_peacemakers_outshine_zealots() {
    assert!(warmth(MentionAttitude::WarmAndReceptive) > warmth(MentionAttitude::AloofSelective));
    assert!(warmth(MentionAttitude::EmpatheticListener) > warmth(MentionAttitude::DevilAdvocate));
    assert!(calm(ConflictHandlingStyle::DeescalateHarmonize) > calm(ConflictHandlingStyle::PassionateConviction));
}
