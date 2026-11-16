//! Turning the working copy into a finished, normalized profile.

use super::AgentDraft;
use crate::domain::profile::HANDLE_PREFIX;
use crate::domain::Profile;
use crate::state::forge::fields::TextField;

/// Short single-line fields whose surrounding whitespace is dropped on commit.
const TRIMMED_FIELDS: [TextField; 7] = [
    TextField::Name,
    TextField::Description,
    TextField::Pronouns,
    TextField::LocationCulture,
    TextField::Occupation,
    TextField::PrimaryLanguage,
    TextField::AvatarInitial,
];

/// `@handle` form of a typed handle (one leading `@` is optional), or `""` when blank.
pub fn normalize_handle(raw: &str) -> String {
    let trimmed = raw.trim();
    let body = trimmed.strip_prefix(HANDLE_PREFIX).unwrap_or(trimmed);
    if body.is_empty() {
        String::new()
    } else {
        format!("{HANDLE_PREFIX}{body}")
    }
}

/// Upper-cased first character of `name`.
fn initial_of(name: &str) -> String {
    name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default()
}

impl AgentDraft {
    /// The working copy with whitespace trimmed, the handle `@`-prefixed and derived fields synced.
    pub fn normalized(&self) -> Profile {
        let mut profile = self.profile.clone();
        for field in TRIMMED_FIELDS {
            let value = field.get_mut(&mut profile);
            *value = value.trim().to_string();
        }
        profile.handle = normalize_handle(&profile.handle);
        let initial_untouched = profile.avatar_initial == self.original.avatar_initial;
        if initial_untouched || profile.avatar_initial.is_empty() {
            let derived = initial_of(&profile.name);
            if !derived.is_empty() {
                profile.avatar_initial = derived;
            }
        }
        profile.sync_derived_fields();
        profile
    }

    /// Finalizes the draft: returns the normalized profile and makes it the new original.
    ///
    /// The caller persists it (e.g. `AppState::update_profile`); validate first.
    pub fn commit(&mut self) -> Profile {
        let finished = self.normalized();
        self.profile = finished.clone();
        self.original = finished.clone();
        self.history.clear();
        finished
    }
}
