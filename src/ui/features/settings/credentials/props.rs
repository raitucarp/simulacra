//! Data and callbacks of the credentials section (Settings › AI Intelligence).

use crate::agent::credentials::{Credential, CredentialKind};
use gpui_kit::component::input::InputState;
use gpui_kit::{App, Entity, Window};
use std::rc::Rc;

/// A no-argument callback.
pub type CredentialAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// A callback taking a credential id.
pub type CredentialIdAction = Rc<dyn Fn(String, &mut Window, &mut App) + 'static>;
/// A callback taking a credential kind.
pub type CredentialKindAction = Rc<dyn Fn(CredentialKind, &mut Window, &mut App) + 'static>;

/// One listed credential; the secret is already masked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialRowView {
    pub id: String,
    pub kind: CredentialKind,
    pub label: String,
    pub masked_secret: String,
    pub endpoint: String,
}

impl CredentialRowView {
    /// The display row of `credential` (never carries the secret itself).
    pub fn of(credential: &Credential) -> Self {
        Self {
            id: credential.id.clone(),
            kind: credential.kind,
            label: credential.shown_label(),
            masked_secret: credential.masked_secret(),
            endpoint: credential.endpoint.clone(),
        }
    }
}

/// The open add/edit form.
pub struct CredentialEditorView {
    /// `false` while editing an existing credential (a blank key keeps the stored one).
    pub is_new: bool,
    pub kind: CredentialKind,
    pub label: Entity<InputState>,
    pub secret: Entity<InputState>,
    pub endpoint: Entity<InputState>,
}

/// Everything the credentials section renders and the actions it raises.
pub struct CredentialsPaneProps {
    pub rows: Vec<CredentialRowView>,
    pub editor: Option<CredentialEditorView>,
    pub on_add: CredentialAction,
    pub on_edit: CredentialIdAction,
    pub on_remove: CredentialIdAction,
    pub on_pick_kind: CredentialKindAction,
    pub on_commit: CredentialAction,
    pub on_cancel: CredentialAction,
}
