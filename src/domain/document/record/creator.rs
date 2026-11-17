//! Creators (authors, editors, translators) and how a personal name splits into parts.

use crate::domain::document::keyed::keyed_enum;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// The part a person played in making the work.
    CreatorRole {
        Author => "author",
        Editor => "editor",
        Translator => "translator",
        Contributor => "contributor",
    }
}

impl Default for CreatorRole {
    fn default() -> Self {
        Self::Author
    }
}

/// Separator of an inverted name ("Spinoza, Baruch").
const INVERTED_SEPARATOR: char = ',';

/// One creator of a work.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Creator {
    /// The name as written ("Baruch Spinoza" or "Spinoza, Baruch").
    pub name: String,
    pub role: CreatorRole,
}

impl Creator {
    pub fn new(name: impl Into<String>, role: CreatorRole) -> Self {
        Self { name: name.into().trim().to_string(), role }
    }

    pub fn author(name: impl Into<String>) -> Self {
        Self::new(name, CreatorRole::Author)
    }

    /// The name split into family and given parts.
    pub fn person_name(&self) -> PersonName {
        PersonName::parse(&self.name)
    }
}

/// A personal name split for citation: "Ludwig Wittgenstein" → family "Wittgenstein",
/// given "Ludwig". A single word (a mononym or an organisation) has no given part.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PersonName {
    pub family: String,
    pub given: String,
}

impl PersonName {
    /// Splits `raw`: an inverted name at its comma, otherwise at the last word.
    pub fn parse(raw: &str) -> Self {
        let raw = raw.trim();
        if let Some((family, given)) = raw.split_once(INVERTED_SEPARATOR) {
            return Self { family: family.trim().to_string(), given: given.trim().to_string() };
        }
        match raw.rsplit_once(char::is_whitespace) {
            Some((given, family)) => Self { family: family.to_string(), given: given.trim().to_string() },
            None => Self { family: raw.to_string(), given: String::new() },
        }
    }

    /// Initials of the given names, each followed by a full stop ("Ludwig Josef" → "L. J.").
    pub fn initials(&self) -> String {
        self.given
            .split(|c: char| c.is_whitespace() || c == '-')
            .filter_map(|part| part.chars().next())
            .map(|initial| format!("{initial}."))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// "Family, Given" (or the family name alone).
    pub fn inverted(&self) -> String {
        if self.given.is_empty() {
            self.family.clone()
        } else {
            format!("{}, {}", self.family, self.given)
        }
    }

    /// "Given Family" (or the family name alone).
    pub fn natural(&self) -> String {
        if self.given.is_empty() {
            self.family.clone()
        } else {
            format!("{} {}", self.given, self.family)
        }
    }
}

/// Parses an editor line of creators: one per line or separated by semicolons, an optional
/// "(role)" suffix naming the role key ("Elwes, R. H. M. (translator)").
pub fn parse_creators(text: &str) -> Vec<Creator> {
    text.split(['\n', ';'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(parse_creator)
        .collect()
}

fn parse_creator(entry: &str) -> Creator {
    let role = entry
        .strip_suffix(')')
        .and_then(|head| head.rsplit_once('('))
        .and_then(|(name, role)| CreatorRole::from_key(role.trim()).map(|role| (name, role)));
    match role {
        Some((name, role)) => Creator::new(name, role),
        None => Creator::author(entry),
    }
}

/// The editor line of `creators` (inverse of [`parse_creators`]).
pub fn format_creators(creators: &[Creator]) -> String {
    creators
        .iter()
        .map(|creator| match creator.role {
            CreatorRole::Author => creator.name.clone(),
            role => format!("{} ({})", creator.name, role.key()),
        })
        .collect::<Vec<_>>()
        .join("; ")
}
