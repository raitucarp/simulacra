//! Creating a new network (thought-space) from a user-typed name.

use super::AppState;
use crate::domain::{Network, NetworkId};
use uuid::Uuid;

/// Longest accepted network name, in characters.
pub const MAX_NETWORK_NAME_CHARS: usize = 48;
/// Prefix of every generated network identifier.
const NETWORK_ID_PREFIX: &str = "net_";
/// Separator replacing runs of non-alphanumeric characters in a slug.
const SLUG_SEPARATOR: char = '-';

/// Why a typed network name cannot be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkNameError {
    /// Nothing but whitespace was typed.
    Empty,
    /// The trimmed name is longer than [`MAX_NETWORK_NAME_CHARS`].
    TooLong,
    /// Another network already uses this name (case-insensitive).
    Duplicate(String),
    /// The network could not be persisted.
    Storage(String),
}

/// Trims `raw` and checks it is non-empty, short enough and unused by `existing`.
pub fn validate_network_name(raw: &str, existing: &[Network]) -> Result<String, NetworkNameError> {
    let name = raw.trim();
    if name.is_empty() {
        return Err(NetworkNameError::Empty);
    }
    if name.chars().count() > MAX_NETWORK_NAME_CHARS {
        return Err(NetworkNameError::TooLong);
    }
    if existing.iter().any(|n| n.name.trim().to_lowercase() == name.to_lowercase()) {
        return Err(NetworkNameError::Duplicate(name.to_string()));
    }
    Ok(name.to_string())
}

/// Lowercase, dash-separated slug of `name` (e.g. "Stoa & Garden" -> "stoa-garden").
pub fn network_slug(name: &str) -> String {
    let lowered: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { SLUG_SEPARATOR })
        .collect();
    lowered
        .split(SLUG_SEPARATOR)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(&SLUG_SEPARATOR.to_string())
}

impl AppState {
    /// Validates `raw_name`, persists a new empty network and makes it the active one.
    pub fn create_network(&mut self, raw_name: &str) -> Result<NetworkId, NetworkNameError> {
        let name = validate_network_name(raw_name, &self.networks)?;
        let id = NetworkId::new(format!("{NETWORK_ID_PREFIX}{}", Uuid::new_v4().simple()));
        let network = Network::new(id.clone(), name.as_str(), network_slug(&name), String::new());
        if let Some(ref db) = self.db {
            db.save_network(&network).map_err(|e| NetworkNameError::Storage(e.to_string()))?;
        }
        self.networks.push(network);
        self.switch_network(&id).map_err(|e| NetworkNameError::Storage(e.to_string()))?;
        Ok(id)
    }
}
