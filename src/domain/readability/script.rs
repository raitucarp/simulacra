//! Which formula a text gets. The script decides first (a text mostly in Chinese characters
//! or kana is measured in characters per sentence; Cyrillic gets Oborneva's Russian formula;
//! Arabic script gets LIX). A Latin-script text gets the formula of the reading language
//! when one was published for it, and LIX otherwise (Indonesian, or a Latin text read in a
//! language of another script).

use super::formulas::Formula;

/// Share of the letters in CJK characters above which a text is measured in characters.
const CJK_SHARE: f64 = 0.3;
const HALF: f64 = 0.5;

/// The reader's language, as far as the formulas care.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReadingLanguage {
    #[default]
    English,
    German,
    French,
    Spanish,
    Portuguese,
    Russian,
    /// Languages without a formula of their own (Indonesian, Arabic, Japanese, Chinese, …).
    Other,
}

impl ReadingLanguage {
    /// The language of a tag such as `en`, `pt-BR` or `zh-CN`.
    pub fn from_code(code: &str) -> Self {
        match code.split(['-', '_']).next().unwrap_or_default().to_ascii_lowercase().as_str() {
            "en" => Self::English,
            "de" => Self::German,
            "fr" => Self::French,
            "es" => Self::Spanish,
            "pt" => Self::Portuguese,
            "ru" => Self::Russian,
            _ => Self::Other,
        }
    }
}

/// Whether `c` is a Chinese character or kana.
pub fn is_cjk(c: char) -> bool {
    matches!(c, '\u{3040}'..='\u{30FF}' | '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{F900}'..='\u{FAFF}')
}

fn is_cyrillic(c: char) -> bool {
    matches!(c, '\u{0400}'..='\u{04FF}')
}

fn is_arabic(c: char) -> bool {
    matches!(c, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}')
}

/// The formula for `text` read by a reader of `language`.
pub fn formula_for(text: &str, language: ReadingLanguage) -> Formula {
    let letters: Vec<char> = text.chars().filter(|c| c.is_alphabetic()).collect();
    let share = |test: fn(char) -> bool| if letters.is_empty() { 0.0 } else { letters.iter().filter(|c| test(**c)).count() as f64 / letters.len() as f64 };
    if share(is_cjk) >= CJK_SHARE {
        return Formula::CjkSentenceLength;
    }
    if share(is_cyrillic) > HALF {
        return Formula::Oborneva;
    }
    if share(is_arabic) > HALF {
        return Formula::Lix;
    }
    match language {
        ReadingLanguage::English => Formula::FleschEnglish,
        ReadingLanguage::German => Formula::Amstad,
        ReadingLanguage::French => Formula::KandelMoles,
        ReadingLanguage::Spanish => Formula::FernandezHuerta,
        ReadingLanguage::Portuguese => Formula::FleschPortuguese,
        ReadingLanguage::Russian | ReadingLanguage::Other => Formula::Lix,
    }
}
