//! What the emoji picker offers (R5): the version cap, search, groups, recent and tones.

use crate::domain::emoji::{
    group_emoji, is_pickable, pickable, search_emoji, EmojiGroup, PickerMemory, SkinTone, EMOJI, PICKER_MAX_EMOJI_VERSION, RECENT_LIMIT,
};

const WAVE: &str = "\u{1F44B}";
const FIRE: &str = "\u{1F525}";

#[test]
fn the_picker_offers_nothing_newer_than_its_cap() {
    assert!(pickable().all(|emoji| emoji.since <= PICKER_MAX_EMOJI_VERSION));
    let newer = EMOJI.iter().find(|emoji| emoji.since > PICKER_MAX_EMOJI_VERSION).expect("the table has newer emoji");
    assert!(!is_pickable(newer.glyph), "{} is kept in the table but not offered", newer.name);
    assert!(is_pickable(FIRE) && is_pickable("\u{1F44B}\u{1F3FD}"), "glyphs and their tones are offered");
    for group in EmojiGroup::ALL {
        assert!(!group_emoji(group).is_empty(), "{group:?}");
    }
}

#[test]
fn search_matches_word_starts_and_puts_name_starts_first() {
    let names = |query: &str| search_emoji(query).into_iter().map(|emoji| emoji.name).collect::<Vec<_>>();
    assert!(names("").is_empty() && names("   ").is_empty());
    let fire = names("fire");
    assert_eq!(fire.first(), Some(&"fire"), "{fire:?}");
    assert!(fire.contains(&"fire engine") && fire.contains(&"firecracker"));
    assert!(names("FIRE ENG").contains(&"fire engine"), "case does not matter; every term must match");
    assert!(!names("ire").contains(&"fire"), "terms match the start of a word");
    let smiling = names("smiling");
    let first_other = smiling.iter().position(|name| !name.starts_with("smiling")).unwrap_or(smiling.len());
    assert!(smiling[first_other..].iter().all(|name| !name.starts_with("smiling")), "names starting with the query come first");
    assert!(names("qqqzzz").is_empty());
}

#[test]
fn recent_emoji_move_to_the_front_without_repeats_and_are_capped() {
    let mut memory = PickerMemory::default();
    memory.used(FIRE);
    memory.used(WAVE);
    memory.used(FIRE);
    assert_eq!(memory.recent, vec![FIRE.to_string(), WAVE.to_string()]);
    for emoji in pickable().take(RECENT_LIMIT + 5) {
        memory.used(emoji.glyph);
    }
    assert_eq!(memory.recent.len(), RECENT_LIMIT);
}

#[test]
fn read_back_memory_drops_unknown_and_repeated_emoji() {
    let saved = PickerMemory { recent: vec![FIRE.into(), "lol".into(), FIRE.into(), WAVE.into()], tone: Some(SkinTone::Medium) };
    let json = serde_json::to_string(&saved).unwrap();
    assert!(json.contains("\"medium\""), "tones are saved by key: {json}");
    let read: PickerMemory = serde_json::from_str(&json).unwrap();
    assert_eq!(read.validated().recent, vec![FIRE.to_string(), WAVE.to_string()]);
    assert_eq!(serde_json::from_str::<PickerMemory>("{}").unwrap(), PickerMemory::default(), "missing fields default");
}

#[test]
fn the_chosen_tone_applies_to_emoji_that_have_tones() {
    let memory = PickerMemory { recent: Vec::new(), tone: Some(SkinTone::Dark) };
    let wave = EMOJI.iter().find(|emoji| emoji.glyph == WAVE).unwrap();
    let fire = EMOJI.iter().find(|emoji| emoji.glyph == FIRE).unwrap();
    assert_eq!(memory.toned(wave), "\u{1F44B}\u{1F3FF}");
    assert_eq!(memory.toned(fire), FIRE);
}
