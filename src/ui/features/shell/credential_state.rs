//! The credentials editor of the AI settings pane: its inputs and what is being edited.

use crate::agent::credentials::{Credential, CredentialKind};
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;

/// The credential being added or edited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialEdit {
    /// `None` for a new credential.
    pub id: Option<String>,
    pub kind: CredentialKind,
}

/// Inputs of the credential form plus the open edit, if any.
pub struct CredentialInputs {
    pub label: Entity<InputState>,
    pub secret: Entity<InputState>,
    pub endpoint: Entity<InputState>,
    pub editing: Option<CredentialEdit>,
}

impl CredentialInputs {
    /// Empty inputs with their placeholders; the form starts closed.
    pub(super) fn new(window: &mut Window, cx: &mut App) -> Self {
        let mut input = |placeholder: String, masked: bool| cx.new(|cx| InputState::new(window, cx).masked(masked).placeholder(placeholder));
        Self {
            label: input(t!("settings.credentials_label_placeholder").to_string(), false),
            secret: input(t!("settings.api_key_placeholder").to_string(), true),
            endpoint: input(t!("settings.credentials_endpoint_placeholder").to_string(), false),
            editing: None,
        }
    }

    /// The typed `(label, secret, endpoint)`, trimmed.
    pub(super) fn values(&self, cx: &App) -> (String, String, String) {
        let read = |input: &Entity<InputState>| input.read(cx).value().trim().to_string();
        (read(&self.label), read(&self.secret), read(&self.endpoint))
    }

    /// Fills the inputs (the secret always starts blank: blank keeps the stored key).
    pub(super) fn fill(&self, label: &str, endpoint: &str, window: &mut Window, cx: &mut App) {
        let fields = [(&self.label, label), (&self.secret, ""), (&self.endpoint, endpoint)];
        for (input, value) in fields {
            let value = value.to_string();
            input.update(cx, |state, cx| state.set_value(value, window, cx));
        }
    }
}

/// The credential to store from the form. Editing keeps the stored secret when the key
/// input is blank; a new credential gets a fresh id.
pub fn credential_from_form(
    edit: &CredentialEdit,
    existing: Option<&Credential>,
    (label, secret, endpoint): (String, String, String),
) -> Credential {
    let label = if label.is_empty() { edit.kind.label() } else { label };
    let secret = match existing {
        Some(stored) if secret.is_empty() => stored.secret.clone(),
        _ => secret,
    };
    match &edit.id {
        Some(id) => Credential { id: id.clone(), kind: edit.kind, label, secret, endpoint },
        None => Credential::new(edit.kind, label, secret, endpoint),
    }
}
