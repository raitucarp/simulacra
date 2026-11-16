use super::disposition::{BONDS, VALUES, VALUES_TEXT, WEATHER, WEATHER_TEXT};
use super::ladder::LadderStep;
use super::mind::REASONING;
use crate::state::forge::attributes::INTELLIGENCE_FIELDS;
use crate::state::forge::{fields_for_step, ForgeStep, QuotientLevel, TraitField};

const LOCALES: [&str; 2] = ["en", "id"];
/// The steps whose pages these tests cover.
const STEPS: [ForgeStep; 4] = [ForgeStep::Birthsign, ForgeStep::Attributes, ForgeStep::Disposition, ForgeStep::Mind];

fn sorted(mut paths: Vec<&'static str>) -> Vec<&'static str> {
    paths.sort_unstable();
    paths
}

fn translated(locale: &str, key: &str) -> bool {
    crate::_rust_i18n_try_translate(locale, key).is_some()
}

#[test]
fn disposition_page_edits_every_disposition_field() {
    let mut shown: Vec<&str> = BONDS.iter().chain(&VALUES).chain(&WEATHER).map(|field| field.path()).collect();
    shown.extend([VALUES_TEXT.path(), WEATHER_TEXT.path()]);
    assert_eq!(sorted(shown), sorted(fields_for_step(ForgeStep::Disposition)));
}

#[test]
fn mind_page_edits_every_mind_field() {
    let mut shown: Vec<&str> = REASONING.iter().map(|field| field.path()).collect();
    shown.extend(INTELLIGENCE_FIELDS.iter().map(|field| field.path()));
    assert_eq!(sorted(shown), sorted(fields_for_step(ForgeStep::Mind)));
}

#[test]
fn the_ladder_counts_points_to_the_next_level() {
    let start = LadderStep::of(0);
    assert_eq!(start.level, QuotientLevel::Novice);
    assert_eq!(start.next, Some((QuotientLevel::Apprentice, QuotientLevel::Apprentice.threshold())));
    let adept = LadderStep::of(QuotientLevel::Expert.threshold() - 1);
    assert_eq!(adept.level, QuotientLevel::Adept);
    assert_eq!(adept.next, Some((QuotientLevel::Expert, 1)));
    assert!(adept.lit(QuotientLevel::Novice) && adept.lit(QuotientLevel::Adept));
    assert!(!adept.lit(QuotientLevel::Expert));
    let top = LadderStep::of(QuotientLevel::Legendary.threshold());
    assert_eq!((top.level, top.next), (QuotientLevel::Legendary, None));
}

#[test]
fn every_option_of_these_steps_has_a_label_and_a_description() {
    let fields = TraitField::ALL.iter().filter(|field| STEPS.contains(&field.step()));
    for field in fields {
        for key in field.option_keys() {
            let label = field.parse(key).map(|value| value.label_key()).unwrap_or_default();
            for locale in LOCALES {
                assert!(translated(locale, &label), "{locale} lacks {label}");
                assert!(translated(locale, &format!("{label}.desc")), "{locale} lacks {label}.desc");
            }
        }
    }
}

#[test]
fn every_field_of_these_steps_has_an_explainer() {
    for path in STEPS.into_iter().flat_map(fields_for_step) {
        for part in ["what", "effect", "example"] {
            for locale in LOCALES {
                let key = format!("forge.explain.{path}.{part}");
                assert!(translated(locale, &key), "{locale} lacks {key}");
            }
        }
    }
}

#[test]
fn translations_differ_between_english_and_indonesian() {
    let key = TraitField::Mbti.parse("intj").map(|value| value.label_key()).unwrap_or_default();
    let desc = format!("{key}.desc");
    assert_ne!(crate::_rust_i18n_try_translate("en", &desc), crate::_rust_i18n_try_translate("id", &desc));
}
