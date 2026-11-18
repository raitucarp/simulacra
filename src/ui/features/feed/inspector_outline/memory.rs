//! What the outline remembers per thread during a session (folded branches and the keyboard
//! cursor), and how each command changes it. Pure state transitions.

use super::tree::{visible_rows, OutlineNode, OutlineRow};
use crate::domain::PostId;
use std::collections::{BTreeSet, HashMap};

/// A key the outline handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutlineKey {
    Up,
    Down,
    /// Collapse, or move to the parent.
    Left,
    /// Expand, or move to the first reply.
    Right,
    /// Open the post under the cursor.
    Enter,
    Home,
    End,
}

/// Something the user did in the outline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutlineCommand {
    /// Chevron click: fold or unfold one branch.
    Toggle(PostId),
    /// Row click: put the cursor there.
    Focus(PostId),
    /// Row double click: open the post.
    Open(PostId),
    Key(OutlineKey),
    CollapseAll,
    ExpandAll,
}

/// One thread's folded branches and cursor.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutlineThread {
    pub collapsed: BTreeSet<PostId>,
    pub cursor: Option<PostId>,
}

/// Every thread's outline state, keyed by the thread's root post.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutlineMemory {
    threads: HashMap<PostId, OutlineThread>,
}

impl OutlineMemory {
    pub fn thread(&self, root: &PostId) -> OutlineThread {
        self.threads.get(root).cloned().unwrap_or_default()
    }

    pub fn with_thread(mut self, root: PostId, thread: OutlineThread) -> Self {
        self.threads.insert(root, thread);
        self
    }
}

/// `thread` after `command` on `tree`, and the post to open (Enter).
pub fn apply_command(thread: OutlineThread, tree: &OutlineNode, command: &OutlineCommand) -> (OutlineThread, Option<PostId>) {
    match command {
        OutlineCommand::Toggle(id) => (toggled(thread, id), None),
        OutlineCommand::Focus(id) => (OutlineThread { cursor: Some(id.clone()), ..thread }, None),
        OutlineCommand::Open(id) => (OutlineThread { cursor: Some(id.clone()), ..thread }, Some(id.clone())),
        OutlineCommand::CollapseAll => (OutlineThread { collapsed: tree.branch_ids(), cursor: Some(tree.id.clone()) }, None),
        OutlineCommand::ExpandAll => (OutlineThread { collapsed: BTreeSet::new(), ..thread }, None),
        OutlineCommand::Key(key) => apply_key(thread, tree, *key),
    }
}

fn toggled(mut thread: OutlineThread, id: &PostId) -> OutlineThread {
    if !thread.collapsed.remove(id) {
        thread.collapsed.insert(id.clone());
    }
    thread
}

fn apply_key(thread: OutlineThread, tree: &OutlineNode, key: OutlineKey) -> (OutlineThread, Option<PostId>) {
    let rows = visible_rows(tree, &thread.collapsed);
    let at = thread.cursor.as_ref().and_then(|id| rows.iter().position(|row| &row.id == id)).unwrap_or(0);
    let Some(row) = rows.get(at) else { return (thread, None) };
    let move_to = |index: usize, thread: OutlineThread| OutlineThread { cursor: rows.get(index).map(|row| row.id.clone()), ..thread };
    match key {
        OutlineKey::Up => (move_to(at.saturating_sub(1), thread), None),
        OutlineKey::Down => (move_to((at + 1).min(rows.len() - 1), thread), None),
        OutlineKey::Home => (move_to(0, thread), None),
        OutlineKey::End => (move_to(rows.len() - 1, thread), None),
        OutlineKey::Enter => (OutlineThread { cursor: Some(row.id.clone()), ..thread }, Some(row.id.clone())),
        OutlineKey::Left => (left(thread, row, &rows), None),
        OutlineKey::Right => (right(thread, &rows, at), None),
    }
}

/// An open branch folds; otherwise the cursor goes up to the parent.
fn left(thread: OutlineThread, row: &OutlineRow, rows: &[OutlineRow]) -> OutlineThread {
    if row.has_children && !row.collapsed {
        return toggled(thread, &row.id);
    }
    let parent = row.parent.as_ref().filter(|parent| rows.iter().any(|r| &r.id == *parent));
    OutlineThread { cursor: parent.cloned().or(Some(row.id.clone())), ..thread }
}

/// A folded branch opens; an open one moves the cursor to its first reply (the next row).
fn right(thread: OutlineThread, rows: &[OutlineRow], at: usize) -> OutlineThread {
    let row = &rows[at];
    match (row.has_children, row.collapsed) {
        (true, true) => toggled(thread, &row.id),
        (true, false) => OutlineThread { cursor: rows.get(at + 1).map(|next| next.id.clone()), ..thread },
        (false, _) => OutlineThread { cursor: Some(row.id.clone()), ..thread },
    }
}
