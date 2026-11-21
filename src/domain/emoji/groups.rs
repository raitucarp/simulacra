//! Emoji groups (as `emoji-test.txt` names them) and the five skin tones.

use serde::{Deserialize, Serialize};

/// A group of the emoji list, in CLDR order (the Component group is not part of the table).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmojiGroup {
    SmileysEmotion,
    PeopleBody,
    AnimalsNature,
    FoodDrink,
    TravelPlaces,
    Activities,
    Objects,
    Symbols,
    Flags,
}

impl EmojiGroup {
    /// Every group, in CLDR order.
    pub const ALL: [EmojiGroup; 9] = [
        Self::SmileysEmotion, Self::PeopleBody, Self::AnimalsNature, Self::FoodDrink, Self::TravelPlaces,
        Self::Activities, Self::Objects, Self::Symbols, Self::Flags,
    ];

    /// The name of the group `emoji-test.txt` lists skin tones and hair styles under.
    pub const COMPONENT: &'static str = "Component";

    /// The group's name in `emoji-test.txt`.
    pub fn unicode_name(self) -> &'static str {
        match self {
            Self::SmileysEmotion => "Smileys & Emotion",
            Self::PeopleBody => "People & Body",
            Self::AnimalsNature => "Animals & Nature",
            Self::FoodDrink => "Food & Drink",
            Self::TravelPlaces => "Travel & Places",
            Self::Activities => "Activities",
            Self::Objects => "Objects",
            Self::Symbols => "Symbols",
            Self::Flags => "Flags",
        }
    }

    /// The variant's name in Rust source, as the generated table writes it.
    pub fn variant_name(self) -> &'static str {
        match self {
            Self::SmileysEmotion => "SmileysEmotion",
            Self::PeopleBody => "PeopleBody",
            Self::AnimalsNature => "AnimalsNature",
            Self::FoodDrink => "FoodDrink",
            Self::TravelPlaces => "TravelPlaces",
            Self::Activities => "Activities",
            Self::Objects => "Objects",
            Self::Symbols => "Symbols",
            Self::Flags => "Flags",
        }
    }
}

/// A skin tone (the Fitzpatrick modifiers U+1F3FB to U+1F3FF), lightest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkinTone {
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

impl SkinTone {
    /// Every tone, lightest first (the order of [`super::Emoji::tones`]).
    pub const ALL: [SkinTone; 5] = [Self::Light, Self::MediumLight, Self::Medium, Self::MediumDark, Self::Dark];

    /// The tone's place in [`Self::ALL`].
    pub fn index(self) -> usize {
        self as usize
    }

    /// The tone's name in `emoji-test.txt` (`medium-light skin tone`).
    pub fn unicode_name(self) -> &'static str {
        match self {
            Self::Light => "light skin tone",
            Self::MediumLight => "medium-light skin tone",
            Self::Medium => "medium skin tone",
            Self::MediumDark => "medium-dark skin tone",
            Self::Dark => "dark skin tone",
        }
    }

    pub fn from_unicode_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tone| tone.unicode_name() == name)
    }

    /// A stable key for preferences (`light`, `medium_light`, ...).
    pub fn key(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::MediumLight => "medium_light",
            Self::Medium => "medium",
            Self::MediumDark => "medium_dark",
            Self::Dark => "dark",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tone| tone.key() == key)
    }
}
