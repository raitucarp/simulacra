//! The inspector's Argument view (sb E1): the thread as an argument map on a canvas, the
//! analysis (which claims stand under the grounded semantics, unsupported claims, circular
//! support, unanswered objections), a node editor and a list of statements to drag into
//! order. Every edit is a pure [`apply_edit`] step; the view announces the new map with
//! [`ArgumentEvent::Changed`] and the shell saves it per thread root.

mod actions;
pub mod canvas;
mod editor;
pub mod events;
mod findings;
mod legend;
mod list;
mod render;

pub use events::{ArgumentEvent, ArgumentThread, ExtractStatus};

use crate::domain::thinking::argument::{analyse, apply_edit, ArgumentReport, EdgeKind, MapEdit, NodeRole};
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;

/// What the node editor is doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorMode {
    /// Nothing selected.
    Closed,
    /// Editing the selected node.
    Node(String),
    /// Writing a new statement with this role, citing this post.
    New { role: NodeRole, source: PostId },
}

/// The Argument view.
pub struct ArgumentPanel {
    pub(super) theme: ThemeTokens,
    pub(super) thread: Option<ArgumentThread>,
    pub(super) report: ArgumentReport,
    pub(super) status: ExtractStatus,
    pub(super) editor: EditorMode,
    /// The next node click joins the edited node to it with this kind.
    pub(super) linking: Option<EdgeKind>,
    /// The statement text being edited or written.
    pub(super) input: Entity<InputState>,
    /// The view's width as last laid out (the canvas places its chips by it).
    pub(super) width: Option<Pixels>,
}

impl EventEmitter<ArgumentEvent> for ArgumentPanel {}

impl ArgumentPanel {
    pub fn new(theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let placeholder = t!("thinking.argument.text_placeholder").to_string();
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        Self { theme, thread: None, report: ArgumentReport::default(), status: ExtractStatus::Idle, editor: EditorMode::Closed, linking: None, input, width: None }
    }

    /// The root of the thread on screen.
    pub fn root(&self) -> Option<&PostId> {
        self.thread.as_ref().map(|thread| &thread.root)
    }

    /// Shows `thread` (a different thread resets the editor and the status).
    pub fn set_thread(&mut self, thread: Option<ArgumentThread>, cx: &mut Context<Self>) {
        if self.thread == thread {
            return;
        }
        if self.root() != thread.as_ref().map(|t| &t.root) {
            (self.editor, self.linking, self.status) = (EditorMode::Closed, None, ExtractStatus::Idle);
        }
        self.report = thread.as_ref().map(|t| analyse(&t.map)).unwrap_or_default();
        self.thread = thread;
        cx.notify();
    }

    pub fn set_status(&mut self, status: ExtractStatus, cx: &mut Context<Self>) {
        if self.status != status {
            self.status = status;
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }

    /// Applies one manual edit and announces the new map (a refused edit changes nothing).
    pub(super) fn edit(&mut self, edit: MapEdit, cx: &mut Context<Self>) -> bool {
        let Some(thread) = self.thread.as_mut() else { return false };
        let Ok(map) = apply_edit(&thread.map, &edit) else { return false };
        self.report = analyse(&map);
        thread.map = map.clone();
        cx.emit(ArgumentEvent::Changed(map));
        cx.notify();
        true
    }
}
