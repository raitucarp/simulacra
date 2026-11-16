//! JSON snapshot of an in-progress draft for the `settings` table (`SettingKey::ForgeAutosave`).
//!
//! Undo history is deliberately not persisted: a restored draft starts a fresh history.

use super::AgentDraft;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Format version written by [`AgentDraft::to_autosave`].
pub const AUTOSAVE_VERSION: u32 = 1;

/// Why an autosave could not be restored.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AutosaveError {
    #[error("autosave is not valid draft JSON: {0}")]
    Malformed(String),
    #[error("autosave version {0} is not supported")]
    UnsupportedVersion(u32),
}

#[derive(Serialize)]
struct EnvelopeRef<'a> {
    version: u32,
    draft: &'a AgentDraft,
}

#[derive(Deserialize)]
struct Envelope {
    version: u32,
    draft: AgentDraft,
}

impl AgentDraft {
    /// Serializes the draft (working copy, original, kind, step and progress) to JSON.
    pub fn to_autosave(&self) -> String {
        let envelope = EnvelopeRef { version: AUTOSAVE_VERSION, draft: self };
        // Every field is plain data (strings, numbers, unit enums), so this cannot fail.
        serde_json::to_string(&envelope).unwrap_or_default()
    }

    /// Restores a draft written by [`AgentDraft::to_autosave`].
    pub fn from_autosave(json: &str) -> Result<Self, AutosaveError> {
        let envelope: Envelope =
            serde_json::from_str(json).map_err(|e| AutosaveError::Malformed(e.to_string()))?;
        if envelope.version != AUTOSAVE_VERSION {
            return Err(AutosaveError::UnsupportedVersion(envelope.version));
        }
        let mut draft = envelope.draft;
        draft.normalize_navigation();
        Ok(draft)
    }
}
