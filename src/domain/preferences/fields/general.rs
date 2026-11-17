//! Fields of the General and Appearance categories.

use super::macros::category_fields;
use super::value::{choice, choice_value, choices, text, toggle, FieldKind, SettingValue};
use crate::domain::preferences::{AppearancePrefs, GeneralPrefs, ReadingWidth};
use crate::domain::preferences::{DateFormat, FirstWeekday, NumberFormat, StartupScreen, TimeFormat};
use crate::i18n::Language;

category_fields! {
    /// A General preference.
    GeneralField {
        Language => "language", DefaultPersona => "default_persona", StartupScreen => "startup_screen",
        DateFormat => "date_format", TimeFormat => "time_format", NumberFormat => "number_format",
        FirstWeekday => "first_weekday",
    }
}

impl GeneralField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::Language => choices::<Language>(),
            Self::DefaultPersona => FieldKind::Persona,
            Self::StartupScreen => choices::<StartupScreen>(),
            Self::DateFormat => choices::<DateFormat>(),
            Self::TimeFormat => choices::<TimeFormat>(),
            Self::NumberFormat => choices::<NumberFormat>(),
            Self::FirstWeekday => choices::<FirstWeekday>(),
        }
    }

    pub fn get(self, prefs: &GeneralPrefs) -> SettingValue {
        match self {
            Self::Language => choice_value(prefs.language),
            Self::DefaultPersona => SettingValue::Text(prefs.default_persona.clone()),
            Self::StartupScreen => choice_value(prefs.startup_screen),
            Self::DateFormat => choice_value(prefs.date_format),
            Self::TimeFormat => choice_value(prefs.time_format),
            Self::NumberFormat => choice_value(prefs.number_format),
            Self::FirstWeekday => choice_value(prefs.first_weekday),
        }
    }

    pub fn set(self, prefs: GeneralPrefs, value: &SettingValue) -> GeneralPrefs {
        match self {
            Self::Language => GeneralPrefs { language: choice(value, prefs.language), ..prefs },
            Self::DefaultPersona => GeneralPrefs { default_persona: text(value, prefs.default_persona.clone()), ..prefs },
            Self::StartupScreen => GeneralPrefs { startup_screen: choice(value, prefs.startup_screen), ..prefs },
            Self::DateFormat => GeneralPrefs { date_format: choice(value, prefs.date_format), ..prefs },
            Self::TimeFormat => GeneralPrefs { time_format: choice(value, prefs.time_format), ..prefs },
            Self::NumberFormat => GeneralPrefs { number_format: choice(value, prefs.number_format), ..prefs },
            Self::FirstWeekday => GeneralPrefs { first_weekday: choice(value, prefs.first_weekday), ..prefs },
        }
    }
}

category_fields! {
    /// An Appearance preference (density and text size live in the theme settings).
    AppearanceField { ReadingWidth => "reading_width", ReducedMotion => "reduced_motion" }
}

impl AppearanceField {
    pub fn kind(self) -> FieldKind {
        match self {
            Self::ReadingWidth => choices::<ReadingWidth>(),
            Self::ReducedMotion => FieldKind::Toggle,
        }
    }

    pub fn get(self, prefs: &AppearancePrefs) -> SettingValue {
        match self {
            Self::ReadingWidth => choice_value(prefs.reading_width),
            Self::ReducedMotion => SettingValue::Toggle(prefs.reduced_motion),
        }
    }

    pub fn set(self, prefs: AppearancePrefs, value: &SettingValue) -> AppearancePrefs {
        match self {
            Self::ReadingWidth => AppearancePrefs { reading_width: choice(value, prefs.reading_width), ..prefs },
            Self::ReducedMotion => AppearancePrefs { reduced_motion: toggle(value, prefs.reduced_motion), ..prefs },
        }
    }
}
