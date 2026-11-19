//! The library's controlled vocabulary (subjects with broader, narrower and related terms,
//! in the manner of a thesaurus) and its user-defined collections.

mod index;
mod tree;
#[cfg(test)]
mod tests;

pub use index::{subject_index, IndexEntry, IndexLetter};
pub use tree::{SubjectNode, SubjectTree, SubjectTreeRow};

use super::ids::{CollectionId, SubjectId};
use crate::domain::NetworkId;
use serde::{Deserialize, Serialize};

/// One preferred term of the subject vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Subject {
    pub id: SubjectId,
    /// Preferred label ("Metaphysics").
    pub label: String,
    /// What the term covers, for consistent use.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub scope_note: String,
    /// The broader term this one narrows (none for a top term).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broader: Option<SubjectId>,
    /// Associated terms that are neither broader nor narrower.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<SubjectId>,
}

impl Subject {
    pub fn new(id: impl Into<SubjectId>, label: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into(), ..Self::default() }
    }

    pub fn under(mut self, broader: impl Into<SubjectId>) -> Self {
        self.broader = Some(broader.into());
        self
    }
}

/// A user-defined collection (a shelf) of documents in one network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Collection {
    pub id: CollectionId,
    pub network_id: NetworkId,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
}

/// The vocabulary and the collections, as held in memory.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Library {
    pub subjects: Vec<Subject>,
    pub collections: Vec<Collection>,
}

impl Library {
    pub fn subject(&self, id: &SubjectId) -> Option<&Subject> {
        self.subjects.iter().find(|s| &s.id == id)
    }

    pub fn collection(&self, id: &CollectionId) -> Option<&Collection> {
        self.collections.iter().find(|c| &c.id == id)
    }

    /// The subject whose label is `label` (case-insensitive), so typed terms reuse the
    /// vocabulary instead of duplicating it.
    pub fn subject_by_label(&self, label: &str) -> Option<&Subject> {
        let wanted = label.trim().to_lowercase();
        self.subjects.iter().find(|s| s.label.trim().to_lowercase() == wanted)
    }

    pub fn collection_by_name(&self, network: &NetworkId, name: &str) -> Option<&Collection> {
        let wanted = name.trim().to_lowercase();
        self.collections.iter().find(|c| &c.network_id == network && c.name.trim().to_lowercase() == wanted)
    }

    /// Collections of `network`, by name.
    pub fn collections_of(&self, network: &NetworkId) -> Vec<&Collection> {
        let mut found: Vec<&Collection> = self.collections.iter().filter(|c| &c.network_id == network).collect();
        found.sort_by_key(|c| c.name.to_lowercase());
        found
    }

    /// The subject tree of the whole vocabulary.
    pub fn tree(&self) -> SubjectTree {
        SubjectTree::build(&self.subjects)
    }

    /// This library with `subject` added or replaced (by id).
    pub fn with_subject(mut self, subject: Subject) -> Self {
        self.subjects.retain(|s| s.id != subject.id);
        self.subjects.push(subject);
        self
    }

    /// This library with `collection` added or replaced (by id).
    pub fn with_collection(mut self, collection: Collection) -> Self {
        self.collections.retain(|c| c.id != collection.id);
        self.collections.push(collection);
        self
    }
}
