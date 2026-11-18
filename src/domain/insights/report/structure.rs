//! The structural views: thread symmetry and turn-taking (group theory), structurally
//! equivalent posts (colour refinement), and the category of typed relations.

use crate::domain::graph::{GraphModel, NodeRef};
use crate::domain::insights::category::{compose, morphisms_of, schema_arrows, CompositionReport, ObjectKind, SchemaArrow};
use crate::domain::insights::facts::PostFacts;
use crate::domain::insights::graph::Graph;
use crate::domain::insights::symmetry::{colour_refinement, equivalence_classes, rotation, tree_symmetry, Rotation, TreeSymmetry};
use crate::domain::Post;
use std::collections::HashMap;

/// Threads analysed at most (the largest).
pub const MAX_THREADS: usize = 12;
/// Smallest thread worth analysing.
pub const MIN_THREAD_POSTS: usize = 3;
/// Equivalence classes kept.
pub const MAX_CLASSES: usize = 12;

/// One reply thread: its posts (global indices, breadth-first), symmetry and rotation.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThreadSymmetry {
    pub posts: Vec<usize>,
    /// Parent of each thread post, as a position in `posts`.
    pub parents: Vec<Option<usize>>,
    pub symmetry: TreeSymmetry,
    /// Over author indices of the report.
    pub rotation: Rotation,
}

/// Group-theoretic view of the analysed posts.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SymmetryReport {
    pub threads: Vec<ThreadSymmetry>,
    /// Structurally equivalent connected posts (global indices), largest class first.
    pub classes: Vec<Vec<usize>>,
    /// Connected posts that were refined.
    pub connected: usize,
}

/// The category of typed relations: objects per kind, the schema and composition.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CategoryReport {
    pub objects: [usize; ObjectKind::ALL.len()],
    pub arrows: Vec<SchemaArrow>,
    pub composition: CompositionReport,
    pub nodes: Vec<NodeRef>,
    pub labels: Vec<String>,
}

pub(super) fn category(model: &GraphModel) -> CategoryReport {
    let (objects, morphisms) = morphisms_of(model);
    let mut counts = [0usize; ObjectKind::ALL.len()];
    objects.iter().for_each(|o| counts[ObjectKind::ALL.iter().position(|k| k == o).unwrap_or_default()] += 1);
    CategoryReport {
        objects: counts,
        arrows: schema_arrows(&objects, &morphisms),
        composition: compose(objects.len(), &morphisms),
        nodes: model.nodes().iter().map(|n| n.node.clone()).collect(),
        labels: model.nodes().iter().map(|n| n.label.clone()).collect(),
    }
}

/// Parent of each post among `posts` (reply target when it is analysed too).
fn parents(posts: &[Post]) -> Vec<Option<usize>> {
    let index: HashMap<&str, usize> = posts.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect();
    posts.iter().map(|p| p.reply_to_id.as_ref().and_then(|id| index.get(id.as_str()).copied())).collect()
}

/// Root of every post, following parents (cycles end at the post where they close).
fn roots(parent: &[Option<usize>]) -> Vec<usize> {
    let mut root: Vec<Option<usize>> = vec![None; parent.len()];
    for start in 0..parent.len() {
        let (mut path, mut at) = (vec![start], start);
        while let Some(p) = parent[at].filter(|p| root[*p].is_none() && !path.contains(p)) {
            path.push(p);
            at = p;
        }
        let top = parent[at].and_then(|p| root[p]).unwrap_or(at);
        path.into_iter().for_each(|v| root[v] = Some(top));
    }
    root.into_iter().enumerate().map(|(v, r)| r.unwrap_or(v)).collect()
}

pub(super) fn symmetry(posts: &[Post], facts: &[PostFacts]) -> SymmetryReport {
    let parent = parents(posts);
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    roots(&parent).into_iter().enumerate().for_each(|(v, r)| groups.entry(r).or_default().push(v));
    let mut threads: Vec<Vec<usize>> = groups.into_values().filter(|g| g.len() >= MIN_THREAD_POSTS).collect();
    threads.sort_by_key(|g| (std::cmp::Reverse(g.len()), g.iter().min().copied()));
    threads.truncate(MAX_THREADS);
    let threads = threads.into_iter().map(|members| thread(members, &parent, facts)).collect();
    let arcs = parent.iter().enumerate().filter_map(|(v, p)| p.map(|p| (v, p)));
    let quotes = posts.iter().enumerate().filter_map(|(v, p)| p.quote_id.as_ref().and_then(|q| posts.iter().position(|o| &o.id == q)).map(|q| (v, q)));
    let graph = Graph::new(posts.len(), arcs.chain(quotes));
    let kinds: Vec<u32> = facts.iter().map(|f| f.kind.index() as u32).collect();
    let colours = colour_refinement(&graph, &kinds);
    let connected: Vec<usize> = (0..posts.len()).filter(|&v| graph.degree(v) > 0).collect();
    let mut classes: Vec<Vec<usize>> = equivalence_classes(&connected.iter().map(|&v| colours[v]).collect::<Vec<_>>())
        .into_iter()
        .map(|class| class.into_iter().map(|i| connected[i]).collect())
        .collect();
    classes.truncate(MAX_CLASSES);
    SymmetryReport { threads, classes, connected: connected.len() }
}

/// One thread's symmetry: members ordered root first by depth.
fn thread(mut members: Vec<usize>, parent: &[Option<usize>], facts: &[PostFacts]) -> ThreadSymmetry {
    members.sort_unstable();
    let position: HashMap<usize, usize> = members.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let local: Vec<Option<usize>> = members.iter().map(|&v| parent[v].and_then(|p| position.get(&p).copied())).collect();
    let replies: Vec<(usize, usize)> = members.iter().filter_map(|&v| parent[v].map(|p| (facts[v].author, facts[p].author))).collect();
    ThreadSymmetry { symmetry: tree_symmetry(&local), rotation: rotation(&replies), parents: local, posts: members }
}
