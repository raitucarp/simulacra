//! Agent Forge tests, one file per concern.

mod attributes;
mod class;
mod describe;
mod draft;
mod history;
mod locales;
mod ontology;
mod palette;
mod roll;
mod steps;
mod validate;
mod voice;

use super::*;
use crate::domain::Profile;

pub(super) const NET: &str = "net_test";

pub(super) fn agent() -> AgentDraft {
    AgentDraft::new(ForgeKind::Agent, NET)
}

pub(super) fn persona() -> AgentDraft {
    AgentDraft::new(ForgeKind::Persona, NET)
}

/// A draft with a valid name and handle.
pub(super) fn named(mut draft: AgentDraft, name: &str, handle: &str) -> AgentDraft {
    draft.edit(DraftEdit::Text(TextField::Name, name.to_string()));
    draft.edit(DraftEdit::Text(TextField::Handle, handle.to_string()));
    draft.seal_history();
    draft
}

pub(super) fn existing(id: &str, name: &str, handle: &str) -> Profile {
    Profile::new_agent(id, name, handle, "Thinker")
}
