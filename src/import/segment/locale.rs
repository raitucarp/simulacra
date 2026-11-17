//! The language-dependent parts of sentence splitting: which abbreviations never end a
//! sentence. Everything else (terminators, quotes, CJK punctuation) is shared.

/// A language family with its own abbreviation list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SentenceLocale {
    #[default]
    English,
    German,
    French,
    Spanish,
    Portuguese,
    Indonesian,
    Russian,
    /// Languages without a list of their own (the shared list still applies).
    Other,
}

/// Abbreviations (lower case, inner dots kept, final dot dropped) that precede a capitalised
/// word mid-sentence. "etc." and its kin are left out: before a capital they usually do end
/// the sentence, and before a lower-case word no abbreviation list is needed.
const SHARED: [&str; 12] = ["e.g", "i.e", "vs", "cf", "dr", "prof", "st", "nr", "fig", "approx", "ca", "vol"];
const ENGLISH: [&str; 18] = [
    "mr", "mrs", "ms", "jr", "sr", "mt", "gen", "col", "capt", "lt", "sgt", "rev", "hon", "inc", "ltd", "co", "p", "pp",
];
const GERMAN: [&str; 13] = ["z.b", "bzw", "vgl", "d.h", "u.a", "hr", "fr", "str", "s", "ggf", "evtl", "inkl", "z.t"];
const FRENCH: [&str; 8] = ["m", "mm", "mme", "mlle", "ste", "p.ex", "env", "av"];
const SPANISH: [&str; 9] = ["sr", "sra", "srta", "d", "dña", "ud", "uds", "p.ej", "pág"];
const PORTUGUESE: [&str; 6] = ["sr", "sra", "srª", "d", "p.ex", "pág"];
const INDONESIAN: [&str; 4] = ["bpk", "ibu", "yth", "sdr"];
const RUSSIAN: [&str; 8] = ["т.е", "т.д", "т.п", "г", "ул", "им", "др", "см"];

impl SentenceLocale {
    /// The locale for a language tag such as `en-GB`, `de` or `pt-BR` (English when unknown).
    pub fn from_language_tag(tag: Option<&str>) -> Self {
        let primary = tag.unwrap_or_default().split(['-', '_']).next().unwrap_or_default().to_ascii_lowercase();
        match primary.as_str() {
            "" | "en" => Self::English,
            "de" => Self::German,
            "fr" => Self::French,
            "es" => Self::Spanish,
            "pt" => Self::Portuguese,
            "id" | "ms" => Self::Indonesian,
            "ru" | "uk" | "be" => Self::Russian,
            _ => Self::Other,
        }
    }

    fn own(self) -> &'static [&'static str] {
        match self {
            Self::English => &ENGLISH,
            Self::German => &GERMAN,
            Self::French => &FRENCH,
            Self::Spanish => &SPANISH,
            Self::Portuguese => &PORTUGUESE,
            Self::Indonesian => &INDONESIAN,
            Self::Russian => &RUSSIAN,
            Self::Other => &[],
        }
    }

    /// Whether `word` (as written before its final dot) is an abbreviation here.
    pub fn is_abbreviation(self, word: &str) -> bool {
        let word = word.to_lowercase();
        SHARED.contains(&word.as_str()) || self.own().contains(&word.as_str())
    }
}
