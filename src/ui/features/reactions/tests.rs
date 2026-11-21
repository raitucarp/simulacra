//! The emoji picker (R5): what each tab and a search show, and its labels; the reaction bar
//! (R6): chips with counts, the acting persona's own highlighted, tooltips naming who reacted.

use super::grid::tone_keys;
use super::chips::{chip_views, names_line, TOOLTIP_NAMES};
use super::{picker_emoji, tab_label, PickerTab};
use crate::domain::reaction::{PostReactions, CLAP};
use crate::domain::{PostId, ProfileId};
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

fn at(minute: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, 20, 9, minute, 0)
}

fn name_of(id: &ProfileId) -> String {
    match id.as_str() {
        "me" => "Ada".into(),
        "soc" => "Socrates".into(),
        other => other.to_uppercase(),
    }
}

#[test]
fn chips_count_each_emoji_and_mark_the_acting_persona() {
    rust_i18n::set_locale("en");
    let (post, me, soc) = (PostId::new("p1"), ProfileId::new("me"), ProfileId::new("soc"));
    let mut set = PostReactions::default();
    set.add(&post, &soc, "\u{1F525}", at(1));
    set.add(&post, &me, "\u{1F525}", at(2));
    set.add(&post, &soc, "\u{1F914}", at(3));
    set.clap(&post, &me, 4, at(4));
    let chips = chip_views(&set, &me, &name_of);
    assert_eq!(chips.len(), 2, "claps have their own button");
    assert_eq!((chips[0].emoji.as_str(), chips[0].total, chips[0].mine), ("\u{1F525}", 2, true));
    assert_eq!((chips[1].total, chips[1].mine), (1, false));
    assert!(chips[0].tooltip.contains("Socrates") && chips[0].tooltip.contains("Ada (you)"), "{}", chips[0].tooltip);
    assert!(!chips[1].tooltip.contains("Ada"), "{}", chips[1].tooltip);
    assert!(chip_views(&PostReactions::default(), &me, &name_of).is_empty());
    assert!(!chips.iter().any(|chip| chip.emoji == CLAP));
}

#[test]
fn long_name_lists_end_with_how_many_more() {
    rust_i18n::set_locale("en");
    let names: Vec<String> = (0..TOOLTIP_NAMES + 3).map(|i| format!("agent {i}")).collect();
    let line = names_line(&names);
    assert!(line.contains("agent 0") && !line.contains(&format!("agent {}", TOOLTIP_NAMES)), "{line}");
    assert!(line.contains('3'), "{line}");
    assert_eq!(names_line(&names[..2]), "agent 0, agent 1");
}
