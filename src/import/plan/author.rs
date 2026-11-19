//! Who imported posts are published as.

use crate::domain::preferences::PostIdentity;
use crate::domain::profile::HANDLE_PREFIX;

/// Handle stem when a pseudonym has no usable characters.
const FALLBACK_HANDLE: &str = "pen_name";
const HANDLE_JOINER: char = '_';

/// The author of imported or captured posts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AuthorChoice {
    /// The active profile.
    #[default]
    Yourself,
    /// The stable anonymous account.
    Anonymous,
    /// A pseudonym with this (trimmed) name, created when it does not exist yet.
    Pseudonym(String),
}

impl AuthorChoice {
    /// The choice for an identity preference; a pseudonym needs the name typed for it.
    pub fn from_identity(identity: PostIdentity, pseudonym: &str) -> Self {
        match identity {
            PostIdentity::Yourself => Self::Yourself,
            PostIdentity::Anonymous => Self::Anonymous,
            PostIdentity::Pseudonym => Self::Pseudonym(pseudonym.trim().to_string()),
        }
    }

    /// The identity preference this choice belongs to.
    pub fn identity(&self) -> PostIdentity {
        match self {
            Self::Yourself => PostIdentity::Yourself,
            Self::Anonymous => PostIdentity::Anonymous,
            Self::Pseudonym(_) => PostIdentity::Pseudonym,
        }
    }

    /// Whether the choice can be applied (a pseudonym needs a name).
    pub fn is_complete(&self) -> bool {
        !matches!(self, Self::Pseudonym(name) if name.trim().is_empty())
    }
}

/// `@lower_case_handle` for a pseudonym name (letters and digits of any script kept).
pub fn pseudonym_handle(name: &str) -> String {
    let words: Vec<String> = name
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect();
    let stem = if words.is_empty() { FALLBACK_HANDLE.to_string() } else { words.join(&HANDLE_JOINER.to_string()) };
    format!("{HANDLE_PREFIX}{stem}")
}
