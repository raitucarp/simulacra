//! The credential store: API keys and endpoints for every provider and service, kept in the
//! settings table (never inside agent JSON). Agents reference credentials by id.
//!
//! - [`kind`]: what a credential authenticates against.
//! - [`persist`]: load/save as one JSON settings row, and the one-time migration of the
//!   legacy global API key into the default credential.

mod kind;
mod persist;

#[cfg(test)]
mod tests;

pub use kind::CredentialKind;
pub use persist::DEFAULT_CREDENTIAL_ID;

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Prefix of generated credential ids (`cred_<uuid>`).
const CREDENTIAL_ID_PREFIX: &str = "cred_";
/// Characters of a secret left visible by [`Credential::masked_secret`].
const VISIBLE_SECRET_CHARS: usize = 4;
/// Mask shown in place of the hidden part of a secret.
const SECRET_MASK: &str = "••••";

/// One stored key. `Debug` never prints the secret.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Credential {
    pub id: String,
    pub kind: CredentialKind,
    pub label: String,
    pub secret: String,
    /// Optional base URL override (required for `Custom`).
    pub endpoint: String,
}

impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credential")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("label", &self.label)
            .field("secret", &self.masked_secret())
            .field("endpoint", &self.endpoint)
            .finish()
    }
}

impl Credential {
    /// A new credential with a fresh id.
    pub fn new(kind: CredentialKind, label: impl Into<String>, secret: impl Into<String>, endpoint: impl Into<String>) -> Self {
        let id = format!("{CREDENTIAL_ID_PREFIX}{}", Uuid::new_v4().simple());
        Self { id, kind, label: label.into(), secret: secret.into(), endpoint: endpoint.into() }
    }

    /// Whether a non-blank secret is stored.
    pub fn has_secret(&self) -> bool {
        !self.secret.trim().is_empty()
    }

    /// The secret with all but its last few characters hidden (`••••abcd`).
    pub fn masked_secret(&self) -> String {
        let chars: Vec<char> = self.secret.trim().chars().collect();
        if chars.len() <= VISIBLE_SECRET_CHARS * 2 {
            return SECRET_MASK.to_string();
        }
        let tail: String = chars[chars.len() - VISIBLE_SECRET_CHARS..].iter().collect();
        format!("{SECRET_MASK}{tail}")
    }
}

/// Every stored credential, in the order the user added them.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CredentialStore {
    pub items: Vec<Credential>,
}

impl CredentialStore {
    /// The credential with `id`.
    pub fn get(&self, id: &str) -> Option<&Credential> {
        self.items.iter().find(|c| c.id == id)
    }

    /// The first credential of `kind` that has a secret (else the first of `kind`).
    pub fn first_of(&self, kind: CredentialKind) -> Option<&Credential> {
        let of_kind = || self.items.iter().filter(move |c| c.kind == kind);
        of_kind().find(|c| c.has_secret()).or_else(|| of_kind().next())
    }

    /// `id` when it exists, else the first credential of `kind`.
    pub fn resolve(&self, id: Option<&str>, kind: CredentialKind) -> Option<&Credential> {
        id.and_then(|id| self.get(id)).or_else(|| self.first_of(kind))
    }

    /// Inserts `credential`, or replaces the one with the same id in place.
    pub fn upsert(&mut self, credential: Credential) {
        match self.items.iter_mut().find(|c| c.id == credential.id) {
            Some(slot) => *slot = credential,
            None => self.items.push(credential),
        }
    }

    /// Removes the credential with `id`; `true` when one was removed.
    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.items.len();
        self.items.retain(|c| c.id != id);
        self.items.len() != before
    }

    /// Every non-blank secret (for redacting error messages and tool output).
    pub fn secrets(&self) -> Vec<String> {
        self.items.iter().filter(|c| c.has_secret()).map(|c| c.secret.trim().to_string()).collect()
    }

    /// Whether any credential holds a secret.
    pub fn any_secret(&self) -> bool {
        self.items.iter().any(Credential::has_secret)
    }
}
