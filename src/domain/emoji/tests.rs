//! The checked-in emoji table (R3): complete, consistent and searchable by glyph.

use super::{find_emoji, EmojiGroup, SkinTone, EMOJI, UNICODE_EMOJI_VERSION};
use crate::domain::reaction::CLAP;
use std::collections::HashSet;

#[test]
fn the_table_covers_every_group_of_its_unicode_version() {
    assert_eq!(UNICODE_EMOJI_VERSION, "17.0");
    assert!(EMOJI.len() > 1800, "{} emoji", EMOJI.len());
    for group in EmojiGroup::ALL {
        assert!(EMOJI.iter().any(|emoji| emoji.group == group), "{group:?} has emoji");
    }
    let first_of = |group: EmojiGroup| EMOJI.iter().position(|emoji| emoji.group == group).unwrap();
    let starts: Vec<usize> = EmojiGroup::ALL.into_iter().map(first_of).collect();
    assert!(starts.windows(2).all(|pair| pair[0] < pair[1]), "groups follow CLDR order");
}

#[test]
fn glyphs_are_unique_and_tones_come_in_fives() {
    let mut seen = HashSet::new();
    for emoji in EMOJI {
        assert!(seen.insert(emoji.glyph), "{} twice", emoji.name);
        assert!(!emoji.name.is_empty());
        assert!(emoji.tones.is_empty() || emoji.tones.len() == SkinTone::ALL.len(), "{}", emoji.name);
        for tone in emoji.tones {
            assert!(seen.insert(tone), "{} tone twice", emoji.name);
        }
    }
}

#[test]
fn skin_tones_apply_and_toned_glyphs_find_their_base() {
    let wave = EMOJI.iter().find(|emoji| emoji.name == "waving hand").expect("waving hand");
    assert!(wave.has_tones());
    assert_eq!(wave.in_tone(Some(SkinTone::Dark)), "\u{1F44B}\u{1F3FF}");
    assert_eq!(wave.in_tone(None), "\u{1F44B}");
    assert_eq!(find_emoji("\u{1F44B}\u{1F3FD}").map(|emoji| emoji.name), Some("waving hand"));
    let robot = find_emoji("\u{1F916}").expect("robot");
    assert_eq!(robot.in_tone(Some(SkinTone::Light)), "\u{1F916}", "an emoji without tones stays itself");
    assert_eq!(find_emoji(CLAP).map(|emoji| emoji.name), Some("clapping hands"));
    assert!(find_emoji("lol").is_none());
}

#[test]
fn skin_tone_keys_round_trip() {
    for tone in SkinTone::ALL {
        assert_eq!(SkinTone::from_key(tone.key()), Some(tone));
        assert_eq!(SkinTone::from_unicode_name(tone.unicode_name()), Some(tone));
    }
    assert_eq!(SkinTone::from_key("purple"), None);
}
