//! Objects, typed morphisms and the schema diagram (morphisms grouped by kinds).

use crate::domain::graph::{EdgeKind, GraphModel, NodeKind};
use std::collections::BTreeMap;

/// The kind of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectKind {
    Thought,
    Document,
    Concept,
    Person,
}

impl ObjectKind {
    pub const ALL: [Self; 4] = [Self::Thought, Self::Document, Self::Concept, Self::Person];

    pub const fn of(kind: NodeKind) -> Self {
        match kind {
            NodeKind::Post => Self::Thought,
            NodeKind::Document => Self::Document,
            NodeKind::Hashtag => Self::Concept,
            NodeKind::Profile => Self::Person,
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Thought => "thought",
            Self::Document => "document",
            Self::Concept => "concept",
            Self::Person => "person",
        }
    }
}

/// The type of a relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MorphismKind {
    Writes,
    RepliesTo,
    Quotes,
    Reposts,
    Links,
    Cites,
    IsAbout,
    Mentions,
}

impl MorphismKind {
    pub const ALL: [Self; 8] =
        [Self::Writes, Self::RepliesTo, Self::Quotes, Self::Reposts, Self::Links, Self::Cites, Self::IsAbout, Self::Mentions];

    /// The relation of a graph edge (a wikilink cites a document and links anything else).
    pub const fn of(edge: EdgeKind, target: ObjectKind) -> Self {
        match (edge, target) {
            (EdgeKind::Authored, _) => Self::Writes,
            (EdgeKind::Reply, _) => Self::RepliesTo,
            (EdgeKind::Quote, _) => Self::Quotes,
            (EdgeKind::Repost, _) => Self::Reposts,
            (EdgeKind::Wikilink, ObjectKind::Document) => Self::Cites,
            (EdgeKind::Wikilink, _) => Self::Links,
            (EdgeKind::Hashtag, _) => Self::IsAbout,
            (EdgeKind::Mention, _) => Self::Mentions,
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Writes => "writes",
            Self::RepliesTo => "replies_to",
            Self::Quotes => "quotes",
            Self::Reposts => "reposts",
            Self::Links => "links",
            Self::Cites => "cites",
            Self::IsAbout => "is_about",
            Self::Mentions => "mentions",
        }
    }
}

/// One morphism between object indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Morphism {
    pub source: u32,
    pub target: u32,
    pub kind: MorphismKind,
}

/// The objects' kinds and the morphisms of a graph model.
pub fn morphisms_of(model: &GraphModel) -> (Vec<ObjectKind>, Vec<Morphism>) {
    let objects: Vec<ObjectKind> = model.nodes().iter().map(|n| ObjectKind::of(n.node.kind())).collect();
    let arrows = model
        .edges()
        .iter()
        .map(|e| Morphism { source: e.source as u32, target: e.target as u32, kind: MorphismKind::of(e.kind, objects[e.target]) })
        .collect();
    (objects, arrows)
}

/// A morphism type between object kinds and how many morphisms it has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaArrow {
    pub source: ObjectKind,
    pub kind: MorphismKind,
    pub target: ObjectKind,
    pub count: usize,
}

/// The schema diagram: morphisms grouped by `(source kind, relation, target kind)`.
pub fn schema_arrows(objects: &[ObjectKind], morphisms: &[Morphism]) -> Vec<SchemaArrow> {
    let mut counts: BTreeMap<(ObjectKind, MorphismKind, ObjectKind), usize> = BTreeMap::new();
    for m in morphisms {
        *counts.entry((objects[m.source as usize], m.kind, objects[m.target as usize])).or_default() += 1;
    }
    counts.into_iter().map(|((source, kind, target), count)| SchemaArrow { source, kind, target, count }).collect()
}
