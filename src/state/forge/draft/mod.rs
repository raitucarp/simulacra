//! The working copy of a profile while it is being forged: edits, undo / redo, dirty state.

mod autosave;
mod commit;
mod editing;
mod history;
mod nav;

pub use autosave::{AutosaveError, AUTOSAVE_VERSION};
pub use commit::normalize_handle;
pub use history::HISTORY_LIMIT;

use super::step::{ForgeKind, ForgeStep};
use crate::domain::agent_config::defaults::values::DESCRIPTION as AGENT_DESCRIPTION;
use crate::domain::{NetworkId, Profile, ProfileId};
use crate::state::store::{AGENT_ID_PREFIX, PERSONA_ID_PREFIX};
use history::History;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Whether the user has opened a step, and whether it was last seen without errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StepProgress {
    pub visited: bool,
    pub complete: bool,
}

/// A profile being created or edited in the forge. Nothing touches the store until
/// [`AgentDraft::commit`] hands back the finished profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDraft {
    profile: Profile,
    original: Profile,
    kind: ForgeKind,
    step: ForgeStep,
    /// One entry per [`ForgeStep::ALL`] element.
    progress: Vec<StepProgress>,
    #[serde(skip)]
    history: History,
}

impl AgentDraft {
    /// A blank (unnamed) profile of `kind` scoped to `network_id`, with a fresh id.
    pub fn new(kind: ForgeKind, network_id: impl Into<NetworkId>) -> Self {
        let profile = match kind {
            ForgeKind::Agent => {
                let id = format!("{AGENT_ID_PREFIX}{}", Uuid::new_v4().simple());
                Profile::new_agent(id, String::new(), String::new(), AGENT_DESCRIPTION)
            }
            ForgeKind::Persona => {
                let id = format!("{PERSONA_ID_PREFIX}{}", Uuid::new_v4().simple());
                Profile::new_alt(id, String::new(), String::new())
            }
        };
        Self::from_parts(profile.with_network(network_id), kind)
    }

    /// A draft that edits an existing profile.
    pub fn edit_existing(profile: Profile) -> Self {
        let kind = ForgeKind::of(&profile);
        Self::from_parts(profile, kind)
    }

    fn from_parts(profile: Profile, kind: ForgeKind) -> Self {
        let mut draft = Self {
            original: profile.clone(),
            profile,
            kind,
            step: ForgeStep::Origin,
            progress: vec![StepProgress::default(); ForgeStep::ALL.len()],
            history: History::default(),
        };
        draft.go_to(ForgeStep::Origin);
        draft
    }

    /// The profile as currently edited.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// The profile as it was when the draft opened (or was last committed).
    pub fn original(&self) -> &Profile {
        &self.original
    }

    pub fn id(&self) -> &ProfileId {
        &self.profile.id
    }

    pub fn kind(&self) -> ForgeKind {
        self.kind
    }

    /// Whether the working copy differs from the original.
    pub fn is_dirty(&self) -> bool {
        self.profile != self.original
    }

    /// Throws away every change since the draft opened (or was last committed).
    pub fn discard(&mut self) {
        self.profile = self.original.clone();
        self.history.clear();
    }
}
