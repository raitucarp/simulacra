use super::enneagram::*;
use super::groups::MbtiGroup;
use crate::domain::{EnneagramType, MbtiType};
use std::collections::HashSet;

const EPSILON: f32 = 1e-5;

#[test]
fn every_type_has_a_core_and_an_adjacent_wing() {
    for ty in EnneagramType::ALL {
        let (c, w) = (core(ty), wing(ty));
        assert!((1..=POINTS).contains(&c), "{ty:?}");
        assert!(is_valid_wing(c, w), "{ty:?} has wing {w} not next to {c}");
    }
    assert_eq!((core(EnneagramType::Type5w4), wing(EnneagramType::Type5w4)), (5, 4));
    assert_eq!((core(EnneagramType::Type9w1), wing(EnneagramType::Type9w1)), (9, 1));
}

#[test]
fn neighbours_wrap_around_the_circle() {
    assert_eq!(clockwise(9), 1);
    assert_eq!(counter_clockwise(1), 9);
    assert_eq!((counter_clockwise(5), clockwise(5)), (4, 6));
    assert!(is_valid_wing(1, 9) && is_valid_wing(1, 2));
    assert!(!is_valid_wing(1, 1) && !is_valid_wing(5, 7) && !is_valid_wing(3, 9));
}

#[test]
fn each_core_offers_exactly_its_two_valid_wings() {
    for point in 1..=POINTS {
        let options = variants(point);
        let wings: HashSet<u8> = options.iter().map(|ty| wing(*ty)).collect();
        assert_eq!(options.len(), 2, "core {point}");
        assert_eq!(wings, HashSet::from([clockwise(point), counter_clockwise(point)]), "core {point}");
    }
}

#[test]
fn moving_the_core_keeps_the_wing_on_the_same_side() {
    assert_eq!(with_core(EnneagramType::Type5w4, 6), EnneagramType::Type6w5);
    assert_eq!(with_core(EnneagramType::Type5w6, 6), EnneagramType::Type6w7);
    assert_eq!(with_core(EnneagramType::Type1w9, 9), EnneagramType::Type9w8);
    assert_eq!(with_core(EnneagramType::Type8w9, 9), EnneagramType::Type9w1);
    for ty in EnneagramType::ALL {
        assert_eq!(with_core(ty, core(ty)), ty, "re-picking the same core keeps {ty:?}");
    }
}

#[test]
fn the_wheel_puts_nine_on_top_and_runs_clockwise() {
    let (x, y) = wheel_offset(9);
    assert!(x.abs() < EPSILON && (y + 1.0).abs() < EPSILON);
    let (x3, y3) = wheel_offset(3);
    let (x6, y6) = wheel_offset(6);
    assert!(x3 > 0.0 && y3 > 0.0, "3 sits lower right");
    assert!(x6 < 0.0 && y6 > 0.0, "6 sits lower left");
    for point in 1..=POINTS {
        let (x, y) = wheel_offset(point);
        assert!((x.hypot(y) - 1.0).abs() < EPSILON, "{point} lies on the unit circle");
    }
    assert!(wheel_offset(1).0 > 0.0 && wheel_offset(8).0 < 0.0);
}

#[test]
fn the_figure_visits_every_point_but_nine_in_the_hexad() {
    let hexad: HashSet<u8> = HEXAD.into_iter().collect();
    let triangle: HashSet<u8> = TRIANGLE.into_iter().collect();
    assert_eq!(hexad, HashSet::from([1, 2, 4, 5, 7, 8]));
    assert_eq!(triangle, HashSet::from([3, 6, 9]));
    assert_eq!(HEXAD.first(), HEXAD.last());
    assert_eq!(TRIANGLE.first(), TRIANGLE.last());
}

#[test]
fn temperament_families_split_the_sixteen_types_four_ways() {
    let mut seen = HashSet::new();
    for group in MbtiGroup::ALL {
        let members = group.members();
        assert_eq!(members.len(), 4, "{group:?}");
        seen.extend(members);
    }
    assert_eq!(seen.len(), MbtiType::ALL.len());
    assert_eq!(MbtiGroup::of(MbtiType::Intj), MbtiGroup::Analysts);
    assert_eq!(MbtiGroup::of(MbtiType::Enfp), MbtiGroup::Diplomats);
    assert_eq!(MbtiGroup::of(MbtiType::Esfj), MbtiGroup::Sentinels);
    assert_eq!(MbtiGroup::of(MbtiType::Istp), MbtiGroup::Explorers);
}

#[test]
fn the_page_renders_every_birthsign_field() {
    use crate::state::forge::{fields_for_step, ForgeStep, TextField, TraitField};
    let rendered = [TraitField::Mbti, TraitField::IdentityVariant, TraitField::Enneagram, TraitField::InstinctualVariant];
    let mut shown: Vec<&str> = rendered.iter().map(|field| field.path()).collect();
    shown.push(TextField::Tritype.path());
    let mut owned = fields_for_step(ForgeStep::Birthsign);
    shown.sort_unstable();
    owned.sort_unstable();
    assert_eq!(shown, owned);
}

#[test]
fn every_family_is_named_in_english_and_indonesian() {
    for group in MbtiGroup::ALL {
        for key in [group.label_key(), group.blurb_key()] {
            for locale in ["en", "id"] {
                assert!(crate::_rust_i18n_try_translate(locale, &key).is_some(), "{locale} lacks {key}");
            }
        }
    }
}
