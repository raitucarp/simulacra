//! General: interface language, default persona, startup screen and regional formats.

use super::super::choices::{language_code, DateFormat, FirstWeekday, NumberFormat, StartupScreen, TimeFormat};
use crate::i18n::Language;
use serde::{Deserialize, Serialize};

/// General preferences.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralPrefs {
    /// Interface language (also switched from the title bar's globe menu).
    #[serde(with = "language_code")]
    pub language: Language,
    /// Profile id the app starts authoring as; empty keeps the last used persona.
    pub default_persona: String,
    /// Screen the window opens on.
    pub startup_screen: StartupScreen,
    /// Consumed by a later wave.
    pub date_format: DateFormat,
    /// Consumed by a later wave.
    pub time_format: TimeFormat,
    /// Digit grouping of numbers (the settings readouts use it; other screens adopt it in
    /// a later wave).
    pub number_format: NumberFormat,
    /// Consumed by a later wave.
    pub first_weekday: FirstWeekday,
}

impl GeneralPrefs {
    /// Trims the persona id (a blank id means "last used").
    pub fn validated(self) -> Self {
        Self { default_persona: self.default_persona.trim().to_string(), ..self }
    }
}
