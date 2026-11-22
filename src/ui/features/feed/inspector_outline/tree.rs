//! The thread as a tree (root post → replies → their replies) and its visible rows once
//! collapsed branches are folded away. Pure: posts come in through lookup closures.

use crate::domain::post::Tint;
use crate::domain::symbols::SymbolGlyph;
use crate::domain::{Post, PostId};
use std::collections::{BTreeSet, HashSet};

/// Deepest reply level kept (a guard against pathological chains).
pub const MAX_OUTLINE_DEPTH: usize = 64;

/// One post of the outline with its replies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlineNode {
    pub id: PostId,
    pub author: String,
    pub text: String,
    /// The post's container tint (sb C2), shown on its bullet.
    pub tint: Option<Tint>,
    /// The post's primary symbol (S4), shown before its author.
    pub symbol: Option<SymbolGlyph>,
    pub children: Vec<OutlineNode>,
}

/// One visible bullet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlineRow {
    pub id: PostId,
    pub parent: Option<PostId>,
    pub depth: usize,
    pub author: String,
    pub text: String,
    /// The post's container tint (sb C2), shown on its bullet.
    pub tint: Option<Tint>,
    /// The post's primary symbol (S4), shown before its author.
    pub symbol: Option<SymbolGlyph>,
    pub has_children: bool,
    pub collapsed: bool,
    /// Replies folded under a collapsed bullet (all levels).
    pub hidden: usize,
}

/// The top of `post`'s reply chain (stops at missing parents and cycles).
pub fn thread_root<'a>(post: &'a Post, parent_of: impl Fn(&PostId) -> Option<&'a Post>) -> &'a Post {
    let mut seen: HashSet<&PostId> = HashSet::from([&post.id]);
    let mut root = post;
    while let Some(parent) = root.reply_to_id.as_ref().and_then(&parent_of) {
        if !seen.insert(&parent.id) {
            break;
        }
        root = parent;
    }
    root
}

/// The tree under `root`; `replies_of` lists direct replies in reading order.
pub fn build_outline<'a>(root: &'a Post, replies_of: &impl Fn(&PostId) -> Vec<&'a Post>, author_of: &impl Fn(&Post) -> String) -> OutlineNode {
    let mut seen = HashSet::from([root.id.clone()]);
    node(root, 0, replies_of, author_of, &mut seen)
}

fn node<'a>(post: &'a Post, depth: usize, replies_of: &impl Fn(&PostId) -> Vec<&'a Post>, author_of: &impl Fn(&Post) -> String, seen: &mut HashSet<PostId>) -> OutlineNode {
    let replies = if depth < MAX_OUTLINE_DEPTH { replies_of(&post.id) } else { Vec::new() };
    let fresh: Vec<&Post> = replies.into_iter().filter(|reply| seen.insert(reply.id.clone())).collect();
    let children = fresh.into_iter().map(|reply| node(reply, depth + 1, replies_of, author_of, seen)).collect();
    let text = post.content.split_whitespace().collect::<Vec<_>>().join(" ");
    OutlineNode { id: post.id.clone(), author: author_of(post), text, tint: post.extras.tint, symbol: post.extras.symbols.face().map(|face| face.primary), children }
}

impl OutlineNode {
    /// Every reply below this node, all levels.
    pub fn descendants(&self) -> usize {
        self.children.iter().map(|child| 1 + child.descendants()).sum()
    }

    /// Ids of every node that has replies (what "collapse all" folds).
    pub fn branch_ids(&self) -> BTreeSet<PostId> {
        let own = (!self.children.is_empty()).then(|| self.id.clone());
        own.into_iter().chain(self.children.iter().flat_map(OutlineNode::branch_ids)).collect()
    }

    /// Whether `id` is in this tree.
    pub fn contains(&self, id: &PostId) -> bool {
        self.id == *id || self.children.iter().any(|child| child.contains(id))
    }
}

/// The bullets shown with `collapsed` branches folded, in reading order.
pub fn visible_rows(root: &OutlineNode, collapsed: &BTreeSet<PostId>) -> Vec<OutlineRow> {
    let mut rows = Vec::new();
    push_rows(root, None, 0, collapsed, &mut rows);
    rows
}

fn push_rows(node: &OutlineNode, parent: Option<&PostId>, depth: usize, collapsed: &BTreeSet<PostId>, rows: &mut Vec<OutlineRow>) {
    let is_collapsed = !node.children.is_empty() && collapsed.contains(&node.id);
    rows.push(OutlineRow {
        id: node.id.clone(),
        parent: parent.cloned(),
        depth,
        author: node.author.clone(),
        text: node.text.clone(),
        tint: node.tint,
        symbol: node.symbol.clone(),
        has_children: !node.children.is_empty(),
        collapsed: is_collapsed,
        hidden: if is_collapsed { node.descendants() } else { 0 },
    });
    if !is_collapsed {
        node.children.iter().for_each(|child| push_rows(child, Some(&node.id), depth + 1, collapsed, rows));
    }
}
