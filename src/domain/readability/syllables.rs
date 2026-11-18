//! Syllable estimates. Every formula of the Flesch family counts spoken syllables; text gives
//! only letters, so each language gets the usual orthographic approximation:
//!
//! - **English:** groups of vowels (`y` included) after dropping a silent ending (`-es` and
//!   `-e` after a consonant other than `l`, and `-ed`), with a leading `y` read as a consonant.
//!   This is the heuristic most readability tools use; it errs by about one syllable in ten
//!   words, which the averages absorb.
//! - **German:** vowel groups over `aeiouyäöü` (diphthongs such as `ei`, `au`, `eu` are one
//!   group).
//! - **French:** vowel groups over the accented vowels, after dropping a mute final `-e` or
//!   `-es` after a consonant.
//! - **Spanish and Portuguese:** vowel groups, split again at a hiatus (two strong vowels
//!   `a e o`, or an accented `í ú` next to a vowel, as in "po-e-ta" and "dí-a").
//! - **Russian:** every vowel letter is a syllable nucleus, so syllables are vowel letters.
//!
//! Every word has at least one syllable.

use crate::import::segment::SentenceLocale;

/// Which syllable rules apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyllableRules {
    #[default]
    English,
    German,
    French,
    Spanish,
    Portuguese,
    Russian,
    /// Vowel groups over the Latin vowels (used where no formula needs syllables).
    Generic,
}

const ENGLISH_VOWELS: &str = "aeiouy";
const GERMAN_VOWELS: &str = "aeiouyäöü";
const FRENCH_VOWELS: &str = "aeiouyàâäéèêëîïôöûùüÿœæ";
const SPANISH_VOWELS: &str = "aeiouáéíóúüy";
const PORTUGUESE_VOWELS: &str = "aeiouáéíóúâêôãõàü";
const RUSSIAN_VOWELS: &str = "аеёиоуыэюя";
const GENERIC_VOWELS: &str = "aeiouyàáâãäåèéêëìíîïòóôõöùúûüýÿæœ";
/// Vowels that keep their own syllable next to another vowel (Spanish and Portuguese).
const STRONG_VOWELS: &str = "aeoáéóíú";
/// English endings that are usually silent, and the consonants that keep them voiced.
const ENGLISH_SILENT: [&str; 3] = ["es", "ed", "e"];
const ENGLISH_VOICED_BEFORE_E: &str = "laeiouy";
const FRENCH_MUTE: [&str; 2] = ["es", "e"];
/// Words this short are one syllable in English whatever their letters.
const ENGLISH_SHORT_WORD: usize = 3;

impl SyllableRules {
    /// The sentence splitter's rules for the same language.
    pub fn sentence_locale(self) -> SentenceLocale {
        match self {
            Self::English => SentenceLocale::English,
            Self::German => SentenceLocale::German,
            Self::French => SentenceLocale::French,
            Self::Spanish => SentenceLocale::Spanish,
            Self::Portuguese => SentenceLocale::Portuguese,
            Self::Russian => SentenceLocale::Russian,
            Self::Generic => SentenceLocale::Other,
        }
    }
}

/// Estimated syllables of `word` (at least one).
pub fn syllables(word: &str, rules: SyllableRules) -> usize {
    let lower: String = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    let count = match rules {
        SyllableRules::English => english(&lower),
        SyllableRules::German => vowel_groups(&lower, GERMAN_VOWELS),
        SyllableRules::French => vowel_groups(strip_mute(&lower, &FRENCH_MUTE, FRENCH_VOWELS), FRENCH_VOWELS),
        SyllableRules::Spanish => vowel_groups(&lower, SPANISH_VOWELS) + hiatuses(&lower, SPANISH_VOWELS),
        SyllableRules::Portuguese => vowel_groups(&lower, PORTUGUESE_VOWELS) + hiatuses(&lower, PORTUGUESE_VOWELS),
        SyllableRules::Russian => lower.chars().filter(|c| RUSSIAN_VOWELS.contains(*c)).count(),
        SyllableRules::Generic => vowel_groups(&lower, GENERIC_VOWELS),
    };
    count.max(1)
}

fn english(word: &str) -> usize {
    if word.chars().count() <= ENGLISH_SHORT_WORD {
        return 1;
    }
    let after_consonant = |stem: &&str| stem.chars().last().is_some_and(|c| !ENGLISH_VOICED_BEFORE_E.contains(c));
    let [es, ed, e] = ENGLISH_SILENT;
    let stem = word
        .strip_suffix(es)
        .filter(after_consonant)
        .or_else(|| word.strip_suffix(ed))
        .or_else(|| word.strip_suffix(e).filter(after_consonant))
        .unwrap_or(word);
    vowel_groups(stem.strip_prefix('y').unwrap_or(stem), ENGLISH_VOWELS)
}

/// `word` without the first of `endings` it ends with, when the letter before it is a
/// consonant of `vowels`' language and something is left.
fn strip_mute<'a>(word: &'a str, endings: &[&str], vowels: &str) -> &'a str {
    endings
        .iter()
        .filter_map(|ending| word.strip_suffix(ending))
        .find(|stem| stem.chars().count() > 1 && stem.chars().last().is_some_and(|c| !vowels.contains(c)))
        .unwrap_or(word)
}

/// Runs of letters from `vowels`.
fn vowel_groups(word: &str, vowels: &str) -> usize {
    let mut previous = false;
    word.chars().filter(|c| {
        let vowel = vowels.contains(*c);
        let starts = vowel && !previous;
        previous = vowel;
        starts
    }).count()
}

/// Adjacent vowel pairs that belong to two syllables (both strong, or an accented weak one).
fn hiatuses(word: &str, vowels: &str) -> usize {
    let letters: Vec<char> = word.chars().collect();
    letters.windows(2).filter(|pair| pair.iter().all(|c| vowels.contains(*c) && STRONG_VOWELS.contains(*c))).count()
}
