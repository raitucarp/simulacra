//! The masked identities posts can go out under: the stable anonymous account and
//! pseudonyms created on the fly from a typed name (reused when the name exists).

use super::{AppState, PERSONA_ID_PREFIX};
use crate::domain::{Profile, ProfileId, ProfileKind, ANONYMOUS_PROFILE_ID};
use crate::error::{AppError, Result};
use crate::import::plan::{pseudonym_handle, AuthorChoice};
use uuid::Uuid;

/// Separator before the number that makes a taken pseudonym handle unique.
const HANDLE_SUFFIX_SEPARATOR: char = '_';

impl AppState {
    /// The profile id `choice` publishes as, creating the anonymous account or the pseudonym
    /// when it does not exist yet.
    pub fn ensure_author(&mut self, choice: &AuthorChoice) -> Result<ProfileId> {
        match choice {
            AuthorChoice::Yourself => self.current_profile_or_err().map(|profile| profile.id),
            AuthorChoice::Anonymous => self.ensure_profile(|p| p.id.as_str() == ANONYMOUS_PROFILE_ID, Profile::new_anonymous),
            AuthorChoice::Pseudonym(name) if name.trim().is_empty() => Err(AppError::EmptyContent),
            AuthorChoice::Pseudonym(name) => {
                let wanted = name.trim().to_lowercase();
                let handle = self.unique_handle(&pseudonym_handle(name));
                let id = ProfileId::new(format!("{PERSONA_ID_PREFIX}{}", Uuid::new_v4().simple()));
                self.ensure_profile(
                    |p| p.kind == ProfileKind::Pseudonym && p.name.to_lowercase() == wanted,
                    || Profile::new_pseudonym(id, name, &handle),
                )
            }
        }
    }

    /// The id of the first profile matching `existing`, else of `create()`, stored and kept.
    fn ensure_profile(&mut self, existing: impl Fn(&Profile) -> bool, create: impl FnOnce() -> Profile) -> Result<ProfileId> {
        if let Some(found) = self.user_profiles.iter().find(|profile| existing(profile)) {
            return Ok(found.id.clone());
        }
        let profile = create();
        if let Some(ref db) = self.db {
            db.save_profile(&profile)?;
        }
        let id = profile.id.clone();
        self.user_profiles.push(profile);
        self.index.listing_changed();
        Ok(id)
    }

    /// `handle`, or `handle_2`, `handle_3`, ... when it is taken.
    fn unique_handle(&self, handle: &str) -> String {
        let taken = |candidate: &str| self.user_profiles.iter().any(|p| p.handle.eq_ignore_ascii_case(candidate));
        (1..)
            .map(|n| if n == 1 { handle.to_string() } else { format!("{handle}{HANDLE_SUFFIX_SEPARATOR}{n}") })
            .find(|candidate| !taken(candidate))
            .unwrap_or_else(|| handle.to_string())
    }
}
