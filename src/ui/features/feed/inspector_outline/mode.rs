//! How the inspector shows a thread (cards or outline), what the outline needs to render,
//! and the commands the inspector sends to the shell.

use super::memory::{OutlineCommand, OutlineMemory};
use crate::domain::document::keyed::keyed_enum;
use crate::domain::PostId;
use gpui_kit::{App, FocusHandle, Window};
use std::rc::Rc;

keyed_enum! {
    /// The inspector's view of a thread; the key is stored in the settings table.
    InspectorMode {
        /// Post cards with avatars, media and actions.
        Cards => "cards",
        /// Nested, text-only bullets.
        Outline => "outline",
    }
}

impl Default for InspectorMode {
    fn default() -> Self {
        Self::Cards
    }
}

impl InspectorMode {
    /// The other mode.
    pub fn toggled(self) -> Self {
        match self {
            Self::Cards => Self::Outline,
            Self::Outline => Self::Cards,
        }
    }
}

/// The inspector's view state as the feed renders it.
#[derive(Clone)]
pub struct InspectorView {
    pub mode: InspectorMode,
    pub outlines: OutlineMemory,
    /// Keyboard focus of the outline.
    pub focus: FocusHandle,
}

/// Something done in the inspector's chrome or outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectorCommand {
    SetMode(InspectorMode),
    /// `command` in the outline of the thread rooted at `root`.
    Outline { root: PostId, command: OutlineCommand },
}

/// Sends an inspector command to the shell.
pub type InspectorCallback = Rc<dyn Fn(InspectorCommand, &mut Window, &mut App) + 'static>;
