//! What clicks in the Argument view do: picking a node (or the far end of a new edge),
//! writing a statement, changing a role, linking, unlinking and removing.

use super::{ArgumentEvent, ArgumentPanel, EditorMode};
use crate::domain::thinking::argument::{EdgeKind, MapEdit, NodeRole};
use crate::domain::PostId;
use gpui_kit::*;

impl ArgumentPanel {
    /// The id of the node being edited.
    pub(super) fn edited_id(&self) -> Option<&str> {
        match &self.editor {
            EditorMode::Node(id) => Some(id.as_str()),
            _ => None,
        }
    }

    fn set_text(&self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value(text, window, cx));
    }

    /// A click on node `index`: completes a pending link, otherwise opens it in the editor.
    pub(super) fn pick_node(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(node) = self.thread.as_ref().and_then(|t| t.map.nodes.get(index)).cloned() else { return };
        if let (Some(kind), Some(from)) = (self.linking, self.edited_id().map(str::to_string)) {
            self.linking = None;
            if from != node.id {
                self.link(&from, &node.id, kind, cx);
            }
            return cx.notify();
        }
        self.editor = EditorMode::Node(node.id.clone());
        self.set_text(node.text, window, cx);
        cx.notify();
    }

    /// Joins `from` to `to` (re-kinding an existing edge between them).
    fn link(&mut self, from: &str, to: &str, kind: EdgeKind, cx: &mut Context<Self>) {
        let exists = self.thread.as_ref().is_some_and(|t| t.map.has_edge(from, to));
        let edit = if exists { MapEdit::SetEdgeKind { from: from.into(), to: to.into(), kind } } else { MapEdit::AddEdge { from: from.into(), to: to.into(), kind } };
        self.edit(edit, cx);
    }

    /// Starts (or cancels) picking the target of a new edge from the edited node.
    pub(super) fn toggle_linking(&mut self, kind: EdgeKind, cx: &mut Context<Self>) {
        self.linking = if self.linking == Some(kind) { None } else { Some(kind) };
        cx.notify();
    }

    /// Opens the editor on a new statement citing the thread's root.
    pub(super) fn compose(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(root) = self.root().cloned() else { return };
        (self.editor, self.linking) = (EditorMode::New { role: NodeRole::Claim, source: root }, None);
        self.set_text(String::new(), window, cx);
        cx.notify();
    }

    pub(super) fn close_editor(&mut self, cx: &mut Context<Self>) {
        (self.editor, self.linking) = (EditorMode::Closed, None);
        cx.notify();
    }

    /// Saves the typed text: a new statement, or the edited node's new wording.
    pub(super) fn save_text(&mut self, cx: &mut Context<Self>) {
        let text = self.input.read(cx).value().to_string();
        match self.editor.clone() {
            EditorMode::New { role, source } => {
                if self.edit(MapEdit::AddNode { role, text, source }, cx) {
                    let id = self.thread.as_ref().and_then(|t| t.map.nodes.last()).map(|n| n.id.clone());
                    self.editor = id.map_or(EditorMode::Closed, EditorMode::Node);
                }
            }
            EditorMode::Node(id) => {
                self.edit(MapEdit::Relabel { id, text }, cx);
            }
            EditorMode::Closed => {}
        }
    }

    /// Changes the role of the edited node, or of the statement being written.
    pub(super) fn set_role(&mut self, role: NodeRole, cx: &mut Context<Self>) {
        match self.editor.clone() {
            EditorMode::New { source, .. } => {
                self.editor = EditorMode::New { role, source };
                cx.notify();
            }
            EditorMode::Node(id) => {
                self.edit(MapEdit::SetRole { id, role }, cx);
            }
            EditorMode::Closed => {}
        }
    }

    /// Picks the post a new statement cites.
    pub(super) fn set_source(&mut self, source: PostId, cx: &mut Context<Self>) {
        if let EditorMode::New { role, .. } = self.editor {
            self.editor = EditorMode::New { role, source };
            cx.notify();
        }
    }

    pub(super) fn remove_edited(&mut self, cx: &mut Context<Self>) {
        if let Some(id) = self.edited_id().map(str::to_string) {
            if self.edit(MapEdit::RemoveNode { id }, cx) {
                self.close_editor(cx);
            }
        }
    }

    pub(super) fn flip_edge(&mut self, from: String, to: String, kind: EdgeKind, cx: &mut Context<Self>) {
        let flipped = match kind {
            EdgeKind::Support => EdgeKind::Attack,
            EdgeKind::Attack => EdgeKind::Support,
        };
        self.edit(MapEdit::SetEdgeKind { from, to, kind: flipped }, cx);
    }

    pub(super) fn remove_edge(&mut self, from: String, to: String, cx: &mut Context<Self>) {
        self.edit(MapEdit::RemoveEdge { from, to }, cx);
    }

    pub(super) fn move_node(&mut self, id: String, to: usize, cx: &mut Context<Self>) {
        self.edit(MapEdit::MoveNode { id, to }, cx);
    }

    pub(super) fn request(&mut self, event: ArgumentEvent, cx: &mut Context<Self>) {
        cx.emit(event);
    }
}
