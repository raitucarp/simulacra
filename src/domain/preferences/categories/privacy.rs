//! Privacy & Data: Simulacra sends no telemetry, and the stored flag can never turn it on.
//! Link previews (sb C3) are fetched from the linked sites unless switched off. Journal entries
//! (sb E3) stay out of every search unless the user lets the palette find them.

use serde::{Deserialize, Serialize};

/// Privacy preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrivacyPrefs {
    /// Always `false`: shown read-only and forced off by [`Self::validated`].
    pub telemetry: bool,
    /// Whether links in posts are fetched for a preview card (title, image, site); off shows
    /// plain links and nothing is requested.
    pub link_previews: bool,
    /// Whether the command palette's word search includes journal entries (sb E3, off).
    pub journal_in_search: bool,
}

impl Default for PrivacyPrefs {
    fn default() -> Self {
        Self { telemetry: false, link_previews: true, journal_in_search: false }
    }
}

impl PrivacyPrefs {
    /// Telemetry stays off whatever was stored.
    pub fn validated(self) -> Self {
        Self { telemetry: false, ..self }
    }
}
