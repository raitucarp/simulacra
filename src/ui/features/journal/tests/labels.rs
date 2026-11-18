use crate::domain::journal::activity::ActivityKind;
use crate::domain::journal::focus::Outcome;
use crate::domain::journal::mood::correlate::MoodMeasure;
use crate::domain::journal::mood::AffectRegion;
use crate::domain::journal::prompts::catalogue;
use crate::domain::journal::templates::Template;
use crate::ui::features::journal::JournalTab;
use rust_i18n::t;

/// Whether `key` has an English translation (a missing key comes back as itself).
fn translated(key: &str) -> bool {
    t!(key, locale = "en").as_ref() != key
}

#[test]
fn every_label_of_the_page_is_translated() {
    let mut keys: Vec<String> = catalogue().iter().flat_map(|p| [p.text_key(), p.kind_key()]).collect();
    keys.extend(JournalTab::ALL.iter().map(|tab| tab.label_key()));
    keys.extend(Template::ALL.iter().flat_map(|t| t.section_keys().into_iter().chain([format!("journal.template.{}", t.key())])));
    keys.extend(AffectRegion::AROUND.iter().chain([&AffectRegion::Neutral]).map(|r| format!("journal.region.{}", r.key())));
    keys.extend(Outcome::ALL.iter().map(|o| format!("journal.outcome.{}", o.key())));
    keys.extend(ActivityKind::ALL.iter().map(|a| format!("journal.activity.{}", a.key())));
    keys.extend(MoodMeasure::ALL.iter().map(|m| format!("journal.measure.{}", m.key())));
    keys.extend((0..7).flat_map(|d| [format!("journal.weekday.{d}"), format!("journal.weekday_short.{d}")]));
    let missing: Vec<&String> = keys.iter().filter(|k| !translated(k)).collect();
    assert!(missing.is_empty(), "untranslated: {missing:?}");
}

#[test]
fn tabs_have_distinct_keys_and_icons() {
    let keys: std::collections::HashSet<_> = JournalTab::ALL.iter().map(|t| t.key()).collect();
    let icons: std::collections::HashSet<_> = JournalTab::ALL.iter().map(|t| t.icon()).collect();
    assert_eq!((keys.len(), icons.len()), (JournalTab::ALL.len(), JournalTab::ALL.len()));
    assert_eq!(JournalTab::default(), JournalTab::Today);
}
