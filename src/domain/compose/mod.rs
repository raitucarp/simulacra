//! Free compose: one block of text typed freely becomes a tree of post drafts.
//!
//! - Every non-empty line is one post; blank lines are ignored.
//! - A line starting with `-` or `*` followed by whitespace is a bullet: the marker is
//!   stripped. A bare marker is an empty bullet and is ignored. Numbered lines (`1. ...`)
//!   and markers not followed by whitespace (`-5`, `**bold**`) are kept as text.
//! - Indentation nests: each tab, or each two spaces, is one level. A line becomes a reply to
//!   the closest line above it that sits at a lower level, so jumping several levels in at
//!   once still makes a direct reply, and dedenting to a level between two earlier ones
//!   attaches to the nearest shallower line.
//! - `\r\n` line endings are handled; text is trimmed of surrounding whitespace.

mod lines;
mod outline; // T2: the preview of the split
#[cfg(test)]
mod tests;

pub use lines::{indent_level, strip_bullet};
pub use outline::{compose_lines, ComposeLine};

/// One post draft and the drafts that reply to it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComposeNode {
    pub text: String,
    pub children: Vec<ComposeNode>,
}

impl ComposeNode {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), children: Vec::new() }
    }

    /// The node with `children` as its replies.
    #[must_use]
    pub fn with_children(self, children: Vec<ComposeNode>) -> Self {
        Self { children, ..self }
    }

    /// This draft plus every reply below it.
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(Self::count).sum::<usize>()
    }
}

/// Splits freely composed `text` into a tree of drafts (see the module rules).
pub fn split_compose(text: &str) -> Vec<ComposeNode> {
    let mut roots = Vec::new();
    let mut open: Vec<(usize, ComposeNode)> = Vec::new();
    for line in text.split('\n') {
        let Some((level, body)) = lines::item(line) else { continue };
        close_to(level, &mut open, &mut roots);
        open.push((level, ComposeNode::new(body)));
    }
    close_to(0, &mut open, &mut roots);
    roots
}

/// Total number of drafts in a tree.
pub fn draft_count(nodes: &[ComposeNode]) -> usize {
    nodes.iter().map(ComposeNode::count).sum()
}

/// Closes every open node at `level` or deeper, attaching each to its parent (or the roots).
fn close_to(level: usize, open: &mut Vec<(usize, ComposeNode)>, roots: &mut Vec<ComposeNode>) {
    while open.last().is_some_and(|(open_level, _)| *open_level >= level) {
        let Some((_, node)) = open.pop() else { break };
        match open.last_mut() {
            Some((_, parent)) => parent.children.push(node),
            None => roots.push(node),
        }
    }
}
