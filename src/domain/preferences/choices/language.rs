//! The interface language as a [`Choice`] keyed by its locale code (`"en"`, `"zh-CN"`).

use super::super::Choice;
use crate::i18n::Language;

/// The locale codes of every supported language, in menu order.
const LANGUAGE_CODES: [&str; 10] = {
    let all = Language::ALL;
    [
        all[0].code(), all[1].code(), all[2].code(), all[3].code(), all[4].code(),
        all[5].code(), all[6].code(), all[7].code(), all[8].code(), all[9].code(),
    ]
};

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Choice for Language {
    const KEYS: &'static [&'static str] = &LANGUAGE_CODES;

    fn key(self) -> &'static str {
        self.code()
    }

    fn from_key(key: &str) -> Option<Self> {
        Language::ALL.into_iter().find(|language| language.code() == key)
    }
}

/// Serde by locale code; an unknown code reads as English.
pub mod language_code {
    use super::{Choice, Language};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(language: &Language, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(language.code())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Language, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Ok(Language::from_key(&raw).unwrap_or_default())
    }
}
