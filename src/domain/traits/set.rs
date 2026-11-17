//! [`OptionSet`]: a multi-select of one trait enum (rhetorical devices, lexical fields, active
//! weekdays), kept in display order without duplicates.
//!
//! The wire format is a JSON array of variant identifiers. Unknown entries are skipped when
//! reading, so a renamed or removed option never resets the whole persisted config.

use crate::domain::TraitOption;
use serde::de::{Deserialize, Deserializer, IgnoredAny};
use serde::Serialize;

/// Several options of one trait enum, in `T::ALL` order, each at most once.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct OptionSet<T>(Vec<T>);

impl<T> Default for OptionSet<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// Position of `option` in `T::ALL` (compared by key).
fn rank<T: TraitOption>(option: T) -> usize {
    T::ALL.iter().position(|o| o.key() == option.key()).unwrap_or(usize::MAX)
}

impl<T: TraitOption> OptionSet<T> {
    /// The set of `options`, sorted into display order and deduplicated.
    pub fn new(options: impl IntoIterator<Item = T>) -> Self {
        let mut list: Vec<T> = options.into_iter().collect();
        list.sort_by_key(|o| rank(*o));
        list.dedup_by_key(|o| rank(*o));
        Self(list)
    }

    /// Every option of `T`.
    pub fn all() -> Self {
        Self(T::ALL.to_vec())
    }

    /// Whether `option` is chosen.
    pub fn contains(&self, option: T) -> bool {
        self.0.iter().any(|o| o.key() == option.key())
    }

    /// A copy with `option` added (`on`) or removed.
    #[must_use]
    pub fn with(&self, option: T, on: bool) -> Self {
        let kept = self.0.iter().copied().filter(|o| o.key() != option.key());
        if on {
            Self::new(kept.chain(std::iter::once(option)))
        } else {
            Self(kept.collect())
        }
    }

    /// A copy with `option` flipped.
    #[must_use]
    pub fn toggled(&self, option: T) -> Self {
        self.with(option, !self.contains(option))
    }

    /// The chosen options, in display order.
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.0.iter().copied()
    }

    /// Stable keys of the chosen options.
    pub fn keys(&self) -> Vec<&'static str> {
        self.iter().map(TraitOption::key).collect()
    }

    /// The chosen options' English labels joined by `separator` (or `empty` when none).
    pub fn labels(&self, separator: &str, empty: &str) -> String {
        if self.0.is_empty() {
            return empty.to_string();
        }
        self.iter().map(TraitOption::label).collect::<Vec<_>>().join(separator)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: TraitOption> FromIterator<T> for OptionSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter)
    }
}

/// One array entry: a known option, or anything else (skipped).
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Entry<T> {
    Known(T),
    Unknown(IgnoredAny),
}

impl<'de, T: TraitOption + Deserialize<'de>> Deserialize<'de> for OptionSet<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let entries: Vec<Entry<T>> = Vec::deserialize(deserializer)?;
        Ok(Self::new(entries.into_iter().filter_map(|entry| match entry {
            Entry::Known(option) => Some(option),
            Entry::Unknown(_) => None,
        })))
    }
}
