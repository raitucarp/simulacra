//! Catalog coverage and behaviour tests (sb D1), one file per concern.

mod ai_suggest;
mod context;
mod coverage;
mod fill;
mod runner;

use super::*;
use crate::domain::traits::BodyKind;
use crate::domain::TraitOption;
use crate::state::forge::{AgentDraft, DraftEdit, ForgeKind, TextField, TraitValue};

pub(super) const NET: &str = "net_test";

pub(super) fn agent() -> AgentDraft {
    AgentDraft::new(ForgeKind::Agent, NET)
}

pub(super) fn persona() -> AgentDraft {
    AgentDraft::new(ForgeKind::Persona, NET)
}

/// Whether the English locale (or its fallback) defines `key`.
pub(super) fn has_key(key: &str) -> bool {
    crate::_rust_i18n_try_translate("en", key).is_some()
}

/// A phrase that resolves: a defined key or a non-blank literal.
pub(super) fn resolves(phrase: &Phrase) -> bool {
    match phrase {
        Phrase::Key(key) => has_key(key),
        Phrase::Literal(text) => !text.trim().is_empty(),
    }
}

/// Resolves keys to themselves (tests compare against the catalog, not translations).
pub(super) fn keys_as_text(phrase: &Phrase) -> String {
    phrase.resolve(|key| key.to_string())
}

/// Drafts covering every body kind, both kinds of profile, with and without a name.
pub(super) fn contexts() -> Vec<AgentDraft> {
    let mut drafts = Vec::new();
    for base in [agent(), persona()] {
        for kind in BodyKind::ALL {
            for name in ["", "Ada Lovelace"] {
                let mut draft = base.clone();
                draft.edit(DraftEdit::Trait(TraitValue::BodyKind(kind)));
                draft.edit(DraftEdit::Text(TextField::Name, name.to_string()));
                drafts.push(draft);
            }
        }
    }
    drafts
}
