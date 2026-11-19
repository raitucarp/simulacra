//! Display texts of control values: choice option labels and number readouts.

use super::super::consts::HOUR_SUFFIX;
use crate::domain::preferences::{AdvancedField, Choice, EmbeddingField, GeneralField, ImageField, ImportField, LearningField};
use crate::domain::preferences::{group_thousands, AppearanceField, NumberSpec, NumberUnit, Preferences, SettingField, WritingField};
use crate::i18n::Language;
use super::super::text::tr;
use rust_i18n::t;

/// Prefix of the option label keys (`prefs.option.<group>.<key>`).
const OPTION_PREFIX: &str = "prefs.option";
/// Prefix of the unit readout keys (`prefs.unit.<unit>`).
const UNIT_PREFIX: &str = "prefs.unit";
const UNLIMITED_KEY: &str = "prefs.unit.unlimited";

/// The option-label group of a choice field (`None` for the language, shown natively).
fn option_group(field: SettingField) -> Option<&'static str> {
    use SettingField as F;
    Some(match field {
        F::General(GeneralField::Language) => return None,
        F::General(GeneralField::StartupScreen) => "startup_screen",
        F::General(GeneralField::DateFormat) => "date_format",
        F::General(GeneralField::TimeFormat) => "time_format",
        F::General(GeneralField::NumberFormat) => "number_format",
        F::General(GeneralField::FirstWeekday) => "first_weekday",
        F::Learning(LearningField::WeeklyDay) => "weekly_review_day",
        F::Appearance(AppearanceField::ReadingWidth) => "reading_width",
        F::Writing(WritingField::DefaultIdentity) | F::Import(ImportField::Identity) => "identity",
        F::Writing(WritingField::WikilinkClick) => "wikilink_click",
        F::Writing(WritingField::HashtagClick) => "hashtag_click",
        F::Import(ImportField::Segmentation) => "segmentation",
        F::Embeddings(EmbeddingField::Provider) => "embedding_provider",
        F::Images(ImageField::DefaultEngine) => "image_engine",
        F::Images(ImageField::Size) => "image_size",
        F::Advanced(AdvancedField::LogLevel) => "log_level",
        _ => return None,
    })
}

/// The translated label of option `key` of `field`.
pub(super) fn option_label(field: SettingField, key: &str) -> String {
    match option_group(field) {
        Some(group) => tr(&format!("{OPTION_PREFIX}.{group}.{key}")),
        None => Language::from_key(key).map_or_else(|| key.to_string(), |language| language.native_label().to_string()),
    }
}

/// The readout of number `value` (`30 s`, `22:00`, `No limit`), grouped by the user's
/// number format.
pub(super) fn number_readout(spec: NumberSpec, value: i64, prefs: &Preferences) -> String {
    if spec.zero_is_unlimited && value == 0 {
        return t!(UNLIMITED_KEY).to_string();
    }
    let general = &prefs.general;
    let count = group_thousands(value, general.number_format.thousands_separator(general.language));
    match spec.unit {
        NumberUnit::Hour => format!("{value:02}{HOUR_SUFFIX}"),
        unit => {
            let key = format!("{UNIT_PREFIX}.{}", unit.key());
            t!(key.as_str(), count = count).to_string()
        }
    }
}
