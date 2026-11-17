//! The broader/narrower hierarchy of the subject vocabulary.
//!
//! A subject without a known broader term is a top term. A broader chain that loops back
//! (a data error) is cut: every subject on the loop becomes a top term, so the tree is
//! always finite.

use super::Subject;
use crate::domain::document::ids::SubjectId;
use std::collections::{HashMap, HashSet};

/// One term and its narrower terms, sorted by label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectNode {
    pub subject: Subject,
    pub children: Vec<SubjectNode>,
}

/// A visible row of the tree: the term, its depth and whether it has narrower terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubjectTreeRow<'a> {
    pub subject: &'a Subject,
    pub depth: usize,
    pub has_children: bool,
}

/// The subject vocabulary as a forest of top terms.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SubjectTree {
    pub roots: Vec<SubjectNode>,
}

/// Whether `id` lies on a loop of broader links (its chain comes back to it).
fn on_loop(id: &SubjectId, parents: &HashMap<&SubjectId, Option<&SubjectId>>) -> bool {
    let mut seen = HashSet::new();
    let mut current = parents.get(id).copied().flatten();
    while let Some(node) = current {
        if node == id {
            return true;
        }
        if !seen.insert(node) {
            return false;
        }
        current = parents.get(node).copied().flatten();
    }
    false
}

fn sorted_by_label(mut nodes: Vec<SubjectNode>) -> Vec<SubjectNode> {
    nodes.sort_by_key(|node| node.subject.label.to_lowercase());
    nodes
}

impl SubjectTree {
    pub fn build(subjects: &[Subject]) -> Self {
        let parents: HashMap<&SubjectId, Option<&SubjectId>> =
            subjects.iter().map(|s| (&s.id, s.broader.as_ref())).collect();
        let is_root = |s: &Subject| match s.broader.as_ref() {
            Some(parent) if parents.contains_key(parent) => on_loop(&s.id, &parents),
            _ => true,
        };
        let roots: HashSet<&SubjectId> = subjects.iter().filter(|s| is_root(s)).map(|s| &s.id).collect();
        let node = |s: &Subject| Self::node(s, subjects, &roots);
        Self { roots: sorted_by_label(subjects.iter().filter(|s| roots.contains(&s.id)).map(node).collect()) }
    }

    fn node(subject: &Subject, all: &[Subject], roots: &HashSet<&SubjectId>) -> SubjectNode {
        let children = all
            .iter()
            .filter(|s| s.broader.as_ref() == Some(&subject.id) && !roots.contains(&s.id))
            .map(|s| Self::node(s, all, roots))
            .collect();
        SubjectNode { subject: subject.clone(), children: sorted_by_label(children) }
    }

    /// `id` and every term below it.
    pub fn descendants(&self, id: &SubjectId) -> Vec<SubjectId> {
        fn collect(node: &SubjectNode, out: &mut Vec<SubjectId>) {
            out.push(node.subject.id.clone());
            node.children.iter().for_each(|child| collect(child, out));
        }
        let mut out = Vec::new();
        if let Some(node) = self.find(id) {
            collect(node, &mut out);
        }
        out
    }

    /// The node of `id`.
    pub fn find(&self, id: &SubjectId) -> Option<&SubjectNode> {
        fn search<'a>(nodes: &'a [SubjectNode], id: &SubjectId) -> Option<&'a SubjectNode> {
            nodes.iter().find_map(|node| if &node.subject.id == id { Some(node) } else { search(&node.children, id) })
        }
        search(&self.roots, id)
    }

    /// Labels from the top term down to `id` ("Philosophy › Metaphysics › Substance").
    pub fn path(&self, id: &SubjectId) -> Vec<&Subject> {
        fn walk<'a>(nodes: &'a [SubjectNode], id: &SubjectId, trail: &mut Vec<&'a Subject>) -> bool {
            nodes.iter().any(|node| {
                trail.push(&node.subject);
                let found = &node.subject.id == id || walk(&node.children, id, trail);
                if !found {
                    trail.pop();
                }
                found
            })
        }
        let mut trail = Vec::new();
        walk(&self.roots, id, &mut trail);
        trail
    }

    /// Rows shown when the terms in `expanded` are open (top terms are always shown).
    pub fn visible_rows(&self, expanded: &HashSet<SubjectId>) -> Vec<SubjectTreeRow<'_>> {
        fn push<'a>(node: &'a SubjectNode, depth: usize, open: &HashSet<SubjectId>, out: &mut Vec<SubjectTreeRow<'a>>) {
            out.push(SubjectTreeRow { subject: &node.subject, depth, has_children: !node.children.is_empty() });
            if open.contains(&node.subject.id) {
                node.children.iter().for_each(|child| push(child, depth + 1, open, out));
            }
        }
        let mut out = Vec::new();
        self.roots.iter().for_each(|root| push(root, 0, expanded, &mut out));
        out
    }
}

