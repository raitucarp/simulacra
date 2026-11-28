//! Whether the keys an engine references exist: the readiness badges of the Engine step and
//! the credential rules of its validation.

use crate::agent::credentials::{CredentialKind, CredentialStore};
use crate::domain::engine::{ImageEngine, SearchBackend};

/// Where one credential reference stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialStatus {
    /// A key is stored; `label` names the credential that will be used.
    Ready { label: String, chosen: bool },
    /// No key of the needed kind is stored (or the chosen one is blank).
    Missing,
    /// The chosen credential was deleted.
    Gone,
    /// Nothing is needed (or a key is optional and none is chosen).
    NotNeeded,
}

impl CredentialStatus {
    /// Whether requests can authenticate.
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready { .. } | Self::NotNeeded)
    }
}

/// The status of credential `id` for a service of `kind`. With `kind` = `None` a key is
/// optional: only a chosen credential is checked. Without a chosen one, the first stored
/// credential of `kind` is used.
pub fn credential_status(store: &CredentialStore, id: Option<&str>, kind: Option<CredentialKind>) -> CredentialStatus {
    if let Some(id) = id {
        return match store.get(id) {
            Some(cred) if cred.has_secret() || kind.is_none() => CredentialStatus::Ready { label: cred.shown_label(), chosen: true },
            Some(_) => CredentialStatus::Missing,
            None => CredentialStatus::Gone,
        };
    }
    let Some(kind) = kind else { return CredentialStatus::NotNeeded };
    match store.first_of(kind).filter(|cred| cred.has_secret()) {
        Some(cred) => CredentialStatus::Ready { label: cred.shown_label(), chosen: false },
        None => CredentialStatus::Missing,
    }
}

/// The credential kind an image engine needs (`None` = optional bearer token).
pub fn image_credential_kind(engine: &ImageEngine) -> Option<CredentialKind> {
    match engine {
        ImageEngine::OpenAi => Some(CredentialKind::OpenAi),
        ImageEngine::Google => Some(CredentialKind::Gemini),
        ImageEngine::None | ImageEngine::LocalSd { .. } | ImageEngine::Custom(_) => None,
    }
}

/// The credential kind a web-search backend needs (`None` = the provider's own search).
pub fn search_credential_kind(backend: SearchBackend) -> Option<CredentialKind> {
    match backend {
        SearchBackend::ProviderNative => None,
        SearchBackend::Brave => Some(CredentialKind::Brave),
        SearchBackend::Tavily => Some(CredentialKind::Tavily),
    }
}
