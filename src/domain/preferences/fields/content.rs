//! Fields of the Reading & Writing and Import categories.

use super::macros::category_fields;
use super::value::{choice, choice_value, choices, count, limit, number, number_value, toggle, FieldKind, NumberUnit, SettingValue};
use crate::domain::preferences::{ArchiveAfter, HashtagClick, ImportSegmentation, PostIdentity, WikilinkClick};
use crate::domain::preferences::{ImportPrefs, WritingPrefs, AUTOSAVE_SECONDS, FEED_BUDGET, MAX_FILE_MB, POST_LENGTH};

category_fields! {
    /// A Reading & Writing preference.
    WritingField {
        PostLengthLimit => "post_length_limit", AutoSplitThreads => "auto_split_threads",
        AutosaveSeconds => "autosave_seconds", Spellcheck => "spellcheck", DefaultIdentity => "default_identity",
        WikilinkClick => "wikilink_click", HashtagClick => "hashtag_click", AtomicHints => "atomic_hints",
        ReadabilityMeter => "readability_meter", AutoArchive => "auto_archive", SymbolShorthand => "symbol_shorthand", FeelingCheckins => "feeling_checkins", FeedBudget => "feed_budget",
    }
}

impl WritingField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::PostLengthLimit => number(POST_LENGTH, NumberUnit::Characters),
            Self::AutoSplitThreads | Self::Spellcheck | Self::AtomicHints | Self::ReadabilityMeter | Self::SymbolShorthand | Self::FeelingCheckins => FieldKind::Toggle,
            Self::AutosaveSeconds => number(AUTOSAVE_SECONDS, NumberUnit::Seconds),
            Self::DefaultIdentity => choices::<PostIdentity>(),
            Self::WikilinkClick => choices::<WikilinkClick>(),
            Self::HashtagClick => choices::<HashtagClick>(),
            Self::AutoArchive => choices::<ArchiveAfter>(),
            Self::FeedBudget => limit(FEED_BUDGET, NumberUnit::PostsPerDay), // N4
        }
    }

    pub fn get(self, prefs: &WritingPrefs) -> SettingValue {
        match self {
            Self::PostLengthLimit => number_value(prefs.post_length_limit),
            Self::AutoSplitThreads => SettingValue::Toggle(prefs.auto_split_threads),
            Self::AutosaveSeconds => number_value(prefs.autosave_seconds),
            Self::Spellcheck => SettingValue::Toggle(prefs.spellcheck),
            Self::DefaultIdentity => choice_value(prefs.default_identity),
            Self::WikilinkClick => choice_value(prefs.wikilink_click),
            Self::HashtagClick => choice_value(prefs.hashtag_click),
            Self::AtomicHints => SettingValue::Toggle(prefs.atomic_hints),
            Self::ReadabilityMeter => SettingValue::Toggle(prefs.readability_meter),
            Self::AutoArchive => choice_value(prefs.auto_archive),
            Self::SymbolShorthand => SettingValue::Toggle(prefs.symbol_shorthand),
            Self::FeelingCheckins => SettingValue::Toggle(prefs.feeling_checkins), // M6
            Self::FeedBudget => number_value(prefs.feed_budget),
        }
    }

    pub fn set(self, prefs: WritingPrefs, value: &SettingValue) -> WritingPrefs {
        match self {
            Self::PostLengthLimit => WritingPrefs { post_length_limit: count(value, prefs.post_length_limit), ..prefs },
            Self::AutoSplitThreads => WritingPrefs { auto_split_threads: toggle(value, prefs.auto_split_threads), ..prefs },
            Self::AutosaveSeconds => WritingPrefs { autosave_seconds: count(value, prefs.autosave_seconds), ..prefs },
            Self::Spellcheck => WritingPrefs { spellcheck: toggle(value, prefs.spellcheck), ..prefs },
            Self::DefaultIdentity => WritingPrefs { default_identity: choice(value, prefs.default_identity), ..prefs },
            Self::WikilinkClick => WritingPrefs { wikilink_click: choice(value, prefs.wikilink_click), ..prefs },
            Self::HashtagClick => WritingPrefs { hashtag_click: choice(value, prefs.hashtag_click), ..prefs },
            Self::AtomicHints => WritingPrefs { atomic_hints: toggle(value, prefs.atomic_hints), ..prefs },
            Self::ReadabilityMeter => WritingPrefs { readability_meter: toggle(value, prefs.readability_meter), ..prefs },
            Self::AutoArchive => WritingPrefs { auto_archive: choice(value, prefs.auto_archive), ..prefs },
            Self::SymbolShorthand => WritingPrefs { symbol_shorthand: toggle(value, prefs.symbol_shorthand), ..prefs },
            Self::FeelingCheckins => WritingPrefs { feeling_checkins: toggle(value, prefs.feeling_checkins), ..prefs },
            Self::FeedBudget => WritingPrefs { feed_budget: count(value, prefs.feed_budget), ..prefs },
        }
    }
}

category_fields! {
    /// An Import preference.
    ImportField {
        Segmentation => "segmentation", Identity => "identity", MaxFileMb => "max_file_mb", KeepOriginal => "keep_original",
    }
}

impl ImportField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::Segmentation => choices::<ImportSegmentation>(),
            Self::Identity => choices::<PostIdentity>(),
            Self::MaxFileMb => number(MAX_FILE_MB, NumberUnit::Megabytes),
            Self::KeepOriginal => FieldKind::Toggle,
        }
    }

    pub fn get(self, prefs: &ImportPrefs) -> SettingValue {
        match self {
            Self::Segmentation => choice_value(prefs.segmentation),
            Self::Identity => choice_value(prefs.identity),
            Self::MaxFileMb => number_value(prefs.max_file_mb),
            Self::KeepOriginal => SettingValue::Toggle(prefs.keep_original),
        }
    }

    pub fn set(self, prefs: ImportPrefs, value: &SettingValue) -> ImportPrefs {
        match self {
            Self::Segmentation => ImportPrefs { segmentation: choice(value, prefs.segmentation), ..prefs },
            Self::Identity => ImportPrefs { identity: choice(value, prefs.identity), ..prefs },
            Self::MaxFileMb => ImportPrefs { max_file_mb: count(value, prefs.max_file_mb), ..prefs },
            Self::KeepOriginal => ImportPrefs { keep_original: toggle(value, prefs.keep_original), ..prefs },
        }
    }
}
