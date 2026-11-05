//! Internationalization (i18n) definitions and supported locales.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Indonesian,
    Spanish,
    ChineseSimplified,
    Japanese,
    French,
    German,
    Portuguese,
    Russian,
    Arabic,
}

impl Language {
    pub const ALL: [Language; 10] = [
        Language::English,
        Language::Indonesian,
        Language::Spanish,
        Language::ChineseSimplified,
        Language::Japanese,
        Language::French,
        Language::German,
        Language::Portuguese,
        Language::Russian,
        Language::Arabic,
    ];

    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Indonesian => "id",
            Self::Spanish => "es",
            Self::ChineseSimplified => "zh-CN",
            Self::Japanese => "ja",
            Self::French => "fr",
            Self::German => "de",
            Self::Portuguese => "pt",
            Self::Russian => "ru",
            Self::Arabic => "ar",
        }
    }

    #[must_use]
    pub fn next(&self) -> Self {
        let all = Self::ALL;
        let idx = all.iter().position(|l| l == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }

    #[must_use]
    pub const fn native_label(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Indonesian => "Bahasa Indonesia",
            Self::Spanish => "Español",
            Self::ChineseSimplified => "简体中文",
            Self::Japanese => "日本語",
            Self::French => "Français",
            Self::German => "Deutsch",
            Self::Portuguese => "Português",
            Self::Russian => "Русский",
            Self::Arabic => "العربية",
        }
    }

    #[must_use]
    pub const fn english_label(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Indonesian => "Indonesian",
            Self::Spanish => "Spanish",
            Self::ChineseSimplified => "Chinese (Simplified)",
            Self::Japanese => "Japanese",
            Self::French => "French",
            Self::German => "German",
            Self::Portuguese => "Portuguese",
            Self::Russian => "Russian",
            Self::Arabic => "Arabic",
        }
    }
}

pub fn set_locale(lang: Language) {
    rust_i18n::set_locale(lang.code());
}
