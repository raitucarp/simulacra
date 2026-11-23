//! How the inspector shows a thread (a view from the registry, epic W), what the outline needs
//! to render, and the commands the inspector sends to the shell.

use super::memory::{OutlineCommand, OutlineMemory};
use crate::domain::views::calendar::YearMonth;
use crate::domain::views::PostView;
use crate::domain::PostId;
use gpui_kit::{App, FocusHandle, Window};
use std::rc::Rc;

/// The inspector's view state as the feed renders it.
#[derive(Clone)]
pub struct InspectorView {
    pub mode: PostView,
    pub outlines: OutlineMemory,
    /// Keyboard focus of the outline.
    pub focus: FocusHandle,
    /// sb E1: the Argument view (set by the shell).
    pub argument: Option<gpui_kit::Entity<crate::ui::features::thinking::argument::ArgumentPanel>>,
    /// W4: the calendar month chosen in the thread rooted at this post (this session).
    pub month: Option<(PostId, YearMonth)>,
}

/// Something done in the inspector's chrome or outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectorCommand {
    SetMode(PostView),
    /// `command` in the outline of the thread rooted at `root`.
    Outline { root: PostId, command: OutlineCommand },
    /// W4: show `month` in the calendar of the thread rooted at `root`.
    Month { root: PostId, month: YearMonth },
}

/// Sends an inspector command to the shell.
pub type InspectorCallback = Rc<dyn Fn(InspectorCommand, &mut Window, &mut App) + 'static>;
