//! Catalog coverage, search matching and highlighting, and the palette section list.

use super::palette::split_keywords;
use super::*;
use crate::domain::preferences::{AgentField, Preferences, SettingField};
use crate::ui::features::settings::tabs::{SettingsTab, DESCRIPTION, KEYWORDS, TITLE};
use rust_i18n::t;
use std::collections::HashSet;

const LOCALES: [&str; 10] = ["en", "id", "de", "fr", "es", "pt", "ru", "ar", "ja", "zh-CN"];

fn entry(title: &str, description: &str, keywords: &str) -> SearchEntry {
    SearchEntry {
        tab: SettingsTab::Agents,
        row: SettingRow::Field(SettingField::Agents(AgentField::QuietHours)),
        title: title.into(),
        description: description.into(),
        keywords: keywords.into(),
        context: "Agents & Simulation".into(),
    }
}

#[test]
fn every_preference_field_is_listed_exactly_once() {
    let rows = all_rows();
    let listed: Vec<SettingField> = rows.iter().filter_map(|(_, row)| match row { SettingRow::Field(f) => Some(*f), _ => None }).collect();
    let unique: HashSet<_> = listed.iter().collect();
    assert_eq!(unique.len(), listed.len(), "a field is listed twice");
    for field in SettingField::all() {
        assert!(unique.contains(&field), "{} is not in the settings window", field.id());
    }
    for command in SettingsCommand::ALL {
        let reachable = rows.iter().any(|(_, row)| *row == SettingRow::Action(command) || matches!(row, SettingRow::Path(folder) if folder.open_command() == command));
        assert!(reachable, "{} has no button", command.id());
        assert_eq!(SettingsCommand::from_id(command.id()), Some(command));
    }
    let ids: HashSet<String> = rows.iter().map(|(_, row)| row.id()).collect();
    assert_eq!(ids.len(), rows.len(), "row ids are unique");
}

#[test]
fn every_category_has_rows_text_and_art() {
    for tab in SettingsTab::ALL {
        assert!(!groups(tab).is_empty(), "{} is empty", tab.id());
        assert_eq!(SettingsTab::from_id(tab.id()), Some(tab));
        let art = std::path::Path::new("assets").join(tab.illustration());
        assert!(art.is_file(), "missing header art {}", art.display());
    }
    assert_eq!(SettingsTab::General.step(-1), SettingsTab::Advanced);
    assert_eq!(SettingsTab::Advanced.step(1), SettingsTab::General);
}

#[test]
fn every_text_key_resolves_in_every_locale() {
    let mut keys: Vec<String> = SettingsTab::ALL.iter().flat_map(|tab| [TITLE, DESCRIPTION, KEYWORDS].map(|part| tab.text_key(part))).collect();
    keys.extend(all_rows().iter().flat_map(|(_, row)| [TITLE, DESCRIPTION, KEYWORDS].map(|part| row.text_key(part))));
    keys.extend(SettingsTab::ALL.iter().flat_map(|tab| groups(*tab).iter().map(|group| group.title_key(*tab))));
    for locale in LOCALES {
        for key in &keys {
            let text = t!(key.as_str(), locale = locale).to_string();
            assert!(!text.is_empty() && !text.contains("prefs."), "{locale}: unresolved {key}");
        }
    }
}

#[test]
fn search_requires_every_term_and_ignores_case() {
    let quiet = entry("Quiet hours", "No autonomous posts at night.", "sleep, night, pause");
    let terms = query_terms("  QUIET   Night ");
    assert_eq!(terms, vec!["quiet".to_string(), "night".to_string()]);
    let hit = match_entry(&quiet, &terms).expect("both terms appear");
    assert_eq!(hit.title_ranges, vec![0..5]);
    assert_eq!(hit.description_ranges, vec![23..28]);
    assert!(match_entry(&quiet, &query_terms("quiet budget")).is_none(), "every term must appear");
    assert!(match_entry(&quiet, &query_terms("sleep")).is_some(), "keywords match");
    assert!(match_entry(&quiet, &query_terms("simulation")).is_some(), "the category matches");
    assert!(match_entry(&quiet, &query_terms("   ")).is_none(), "a blank query matches nothing");
}

#[test]
fn ranges_are_char_boundaries_and_merge() {
    assert_eq!(find_ranges("Größe GRÖSSE größe", "größe"), vec![0..7, 16..23]);
    assert_eq!(find_ranges("日本語の設定", "設定"), vec![12..18]);
    assert_eq!(find_ranges("aaaa", "aa"), vec![0..2, 2..4], "occurrences do not overlap");
    assert_eq!(find_ranges("short", "much longer"), Vec::<std::ops::Range<usize>>::new());
    assert_eq!(merge_ranges(vec![5..8, 0..2, 1..4, 8..9]), vec![0..4, 5..9]);
}

#[test]
fn search_over_the_real_catalog_finds_rows_in_order() {
    rust_i18n::set_locale("en");
    let entries = search_entries();
    let hits = search("timeout", &entries);
    assert!(hits.iter().any(|hit| hit.tab == SettingsTab::AiIntelligence), "the request timeout is found");
    assert!(search("", &entries).is_empty());
    let all = search("a", &entries);
    let order: Vec<usize> = all.iter().map(|hit| entries.iter().position(|e| e.row == hit.row).unwrap()).collect();
    assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "hits keep window order");
    assert_eq!(hits_in(&hits, SettingsTab::AiIntelligence), hits.iter().filter(|h| h.tab == SettingsTab::AiIntelligence).count());
}

#[test]
fn dependent_rows_follow_their_switch() {
    let quiet_start = SettingRow::Field(SettingField::Agents(AgentField::QuietStart));
    let mut prefs = Preferences::default();
    assert!(!is_enabled(quiet_start, &prefs));
    prefs.agents.quiet_hours = true;
    assert!(is_enabled(quiet_start, &prefs));
    assert!(is_enabled(SettingRow::Field(SettingField::Privacy(crate::domain::preferences::PrivacyField::Telemetry)), &prefs), "a locked badge is not dimmed");
}

#[test]
fn palette_sections_are_translated_with_keywords() {
    rust_i18n::set_locale("en");
    let sections = settings_sections();
    assert_eq!(sections.len(), SettingsTab::ALL.len());
    assert!(sections.iter().all(|section| !section.title.is_empty() && !section.keywords.is_empty()));
    assert_eq!(split_keywords(" a, ,b ,"), vec!["a".to_string(), "b".to_string()]);
}
