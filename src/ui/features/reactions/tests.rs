//! The emoji picker's choices (R5): what each tab and a search show, and its labels.

use super::grid::tone_keys;
use super::{picker_emoji, tab_label, PickerTab};
use crate::domain::emoji::{group_emoji, EmojiGroup, PickerMemory, SkinTone};

#[test]
fn tabs_show_recent_or_group_emoji_and_a_query_shows_results() {
    let memory = PickerMemory { recent: vec!["\u{1F525}".into()], tone: Some(SkinTone::Light) };
    assert_eq!(picker_emoji(PickerTab::Recent, "", &memory), vec!["\u{1F525}".to_string()]);
    let people = picker_emoji(PickerTab::Group(EmojiGroup::PeopleBody), "", &memory);
    assert_eq!(people.len(), group_emoji(EmojiGroup::PeopleBody).len());
    assert!(people.contains(&"\u{1F44B}\u{1F3FB}".to_string()), "the chosen tone is applied");
    let found = picker_emoji(PickerTab::Recent, "waving", &memory);
    assert_eq!(found.first().map(String::as_str), Some("\u{1F44B}\u{1F3FB}"), "a query searches every group");
    assert!(picker_emoji(PickerTab::Recent, "", &PickerMemory::default()).is_empty());
}

#[test]
fn every_tab_and_tone_has_a_label_in_english() {
    assert_eq!(PickerTab::all().count(), EmojiGroup::ALL.len() + 1);
    for tab in PickerTab::all() {
        assert!(!tab_label(tab).starts_with("reactions."), "{tab:?}");
    }
    let keys = ["reactions.add", "reactions.picker.search_placeholder", "reactions.picker.results", "reactions.picker.no_results", "reactions.picker.no_recent", "reactions.picker.skin_tone"];
    for key in tone_keys().into_iter().chain(keys) {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}
