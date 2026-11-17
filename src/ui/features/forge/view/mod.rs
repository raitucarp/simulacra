//! [`ForgeView`]: the full-screen character creator entity. It owns the working
//! [`AgentDraft`] and every piece of forge UI state; the host (`AppView`) persists what the
//! forge asks for through [`ForgeEvent`]s and reports back with `committed` /
//! `apply_avatar` / `set_notice`.

mod actions;
mod begin;
mod describe;
mod dismiss;
mod handle;
mod hear;
mod keys;
mod lifecycle;
mod state;

pub use handle::ForgeHandle;
pub use hear::HearStatus;
pub use keys::{init_key_bindings, Dismiss, Redo, Undo, FORGE_KEY_CONTEXT};
pub use state::{DescribeStatus, ForgeContext, ForgeOpen, ForgeScreen, GuideState, Notice, StartState};

use crate::domain::portrait::{FormSlot, PortraitSlot};
use crate::domain::{Palette, Profile};
use crate::state::forge::{draft_palette, AgentDraft, ValidationIssue};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::AutosavePolicy;
use gpui_kit::*;
use std::path::PathBuf;

/// What the forge asks its host to do.
#[derive(Debug, Clone)]
pub enum ForgeEvent {
    /// Leave the forge (the draft was already autosaved or cleared).
    Close,
    /// Store this snapshot so an interrupted session can resume.
    Autosave(Box<AgentDraft>),
    /// Forget the stored snapshot.
    ClearAutosave,
    /// Persist this finished profile, then call [`ForgeView::committed`].
    Commit { profile: Box<Profile>, bring_to_life: bool, intro_post: bool },
    /// Persist whether Mnemo is hidden.
    GuideHidden(bool),
    /// Import this image as the draft's photo avatar, then call [`ForgeView::apply_avatar`].
    ImportPhoto(PathBuf),
}

/// The Agent Forge entity.
pub struct ForgeView {
    pub(crate) theme: ThemeTokens,
    pub(crate) draft: AgentDraft,
    pub(crate) context: ForgeContext,
    pub(crate) screen: ForgeScreen,
    /// Validation of the current draft against the other profiles.
    pub(crate) issues: Vec<ValidationIssue>,
    pub(crate) autosave: AutosavePolicy,
    pub(crate) guide: GuideState,
    pub(crate) start: StartState,
    /// "Write an introduction post" on the Review step.
    pub(crate) intro_post: bool,
    /// A transient message (photo import failure, describe result).
    pub(crate) notice: Option<Notice>,
    /// The portrait part whose options the Appearance step shows.
    pub(crate) portrait_slot: PortraitSlot,
    /// The form part whose options the Appearance step shows for non-human bodies.
    pub(crate) form_slot: FormSlot,
    /// "Hear them speak" on the Voice step.
    pub(crate) hear: HearStatus,
    /// Whether the Grimoire's advanced settings are unfolded.
    pub(crate) grimoire_open: bool,
    /// The step palette hovered in the current step's palette strip (its changes preview).
    pub(crate) strip_preview: Option<&'static Palette>,
    pub(crate) focus: FocusHandle,
}

impl EventEmitter<ForgeEvent> for ForgeView {}

impl ForgeView {
    /// The full palette (or class) the draft was started from, shown on the nameplate.
    pub(crate) fn palette(&self) -> Option<&'static Palette> {
        draft_palette(&self.draft)
    }
}

impl Focusable for ForgeView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}
