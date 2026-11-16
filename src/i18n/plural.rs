//! Count-aware labels: picks the `<base>_one` or `<base>_other` translation of a key for a
//! number, following each shipped locale's plural rule.
//!
//! Every locale defines both variants. Languages with more categories (Russian, Arabic) word
//! their `_other` form so it reads correctly for every remaining number ("Ответов: 5").

use super::Language;
use rust_i18n::t;

/// Joins several count labels in one caption ("3 docs · 2 members").
pub const COUNT_SEPARATOR: &str = " · ";
/// Key suffix of the singular variant.
const ONE_SUFFIX: &str = "_one";
/// Key suffix of the variant used for every other number.
const OTHER_SUFFIX: &str = "_other";
/// Russian "one" rule: ends in 1 but not in 11 (1, 21, 101 ...).
const DECIMAL_BASE: usize = 10;
const HUNDRED_BASE: usize = 100;
const RU_ONE_UNIT: usize = 1;
const RU_ONE_EXCEPTION: usize = 11;

/// Plural category of a number in a locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralCategory {
    One,
    Other,
}

impl PluralCategory {
    /// The category of `count` in the locale with code `locale` (see [`super::Language::code`]).
    #[must_use]
    pub fn of(locale: &str, count: usize) -> Self {
        let language = Language::ALL.into_iter().find(|language| language.code() == locale);
        let is_one = match language {
            Some(Language::French | Language::Portuguese) => count <= 1,
            Some(Language::Russian) => {
                count % DECIMAL_BASE == RU_ONE_UNIT && count % HUNDRED_BASE != RU_ONE_EXCEPTION
            }
            Some(Language::Japanese | Language::ChineseSimplified) => false,
            _ => count == 1,
        };
        if is_one { Self::One } else { Self::Other }
    }

    const fn suffix(self) -> &'static str {
        match self {
            Self::One => ONE_SUFFIX,
            Self::Other => OTHER_SUFFIX,
        }
    }
}

/// The variant of `base` for `count` in `locale`.
#[must_use]
pub fn plural_key_in(locale: &str, base: &str, count: usize) -> String {
    format!("{base}{}", PluralCategory::of(locale, count).suffix())
}

/// The variant of `base` for `count` in the active locale.
#[must_use]
pub fn plural_key(base: &str, count: usize) -> String {
    plural_key_in(&rust_i18n::locale(), base, count)
}

/// The translation of `base` for `count` with `%{count}` filled in.
#[must_use]
pub fn t_count(base: &str, count: usize) -> String {
    let key = plural_key(base, count);
    t!(key.as_str(), count = count).to_string()
}
