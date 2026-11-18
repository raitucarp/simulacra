//! [`SuggestContext`]: what suggestions may depend on — the draft profile so far (body kind,
//! type, voice, engine…) and the palette it was started from.

use crate::domain::agent_config::AgentConfig;
use crate::domain::{Palette, Profile};
use crate::state::forge::{draft_palette, AgentDraft, ForgeKind};

/// The draft as the catalog sees it.
#[derive(Debug, Clone, Copy)]
pub struct SuggestContext<'a> {
    pub profile: &'a Profile,
    pub palette: Option<&'static Palette>,
    pub kind: ForgeKind,
}

impl<'a> SuggestContext<'a> {
    /// The context of a draft in the forge.
    pub fn of(draft: &'a AgentDraft) -> Self {
        Self { profile: draft.profile(), palette: draft_palette(draft), kind: draft.kind() }
    }

    /// A context for a bare profile (no palette).
    pub fn profile(profile: &'a Profile) -> Self {
        Self { profile, palette: None, kind: ForgeKind::of(profile) }
    }

    pub fn config(&self) -> &'a AgentConfig {
        &self.profile.config
    }
}
