//! Related entries: documents that share subjects (broader terms give partial credit), the
//! same discipline, or, once embeddings exist, similar meaning.
//!
//! The semantic part is a hook: callers pass precomputed [`ScoredDocument`]s (cosine
//! similarities from an embedding index) and they are merged with the subject scores.

use crate::domain::document::facets::{document_values, Facet};
use crate::domain::document::ids::DocumentId;
use crate::domain::document::vocabulary::SubjectTree;
use crate::domain::document::GroundingDocument;
use std::collections::BTreeSet;

/// A document and a similarity in `0.0..=1.0`.
#[derive(Debug, Clone, PartialEq)]
pub struct ScoredDocument {
    pub id: DocumentId,
    pub score: f32,
}

/// Why an entry is related.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelatedReason {
    /// Shares this many subjects (counting broader terms).
    SharedSubjects(usize),
    /// Same discipline, no shared subject.
    SameDiscipline,
    /// Close in meaning (from the embedding hook).
    Semantic,
}

/// One related entry.
#[derive(Debug, Clone, PartialEq)]
pub struct RelatedEntry {
    pub id: DocumentId,
    pub score: f32,
    pub reason: RelatedReason,
}

/// Score given to a shared discipline alone.
const DISCIPLINE_SCORE: f32 = 0.15;

fn subject_entry(doc: &GroundingDocument, other: &GroundingDocument, tree: &SubjectTree) -> Option<RelatedEntry> {
    let mine: BTreeSet<String> = document_values(doc, Facet::Subject, tree).into_iter().collect();
    let theirs: BTreeSet<String> = document_values(other, Facet::Subject, tree).into_iter().collect();
    let shared = mine.intersection(&theirs).count();
    let union = mine.union(&theirs).count();
    let id = other.document_id();
    if shared > 0 {
        return Some(RelatedEntry { id, score: shared as f32 / union as f32, reason: RelatedReason::SharedSubjects(shared) });
    }
    let discipline = doc.record.classification.discipline;
    (discipline.is_some() && discipline == other.record.classification.discipline)
        .then_some(RelatedEntry { id, score: DISCIPLINE_SCORE, reason: RelatedReason::SameDiscipline })
}

/// Up to `limit` entries related to `doc` among `docs` of its network, best first.
/// `semantic` scores win where they are higher than the subject score.
pub fn related_entries(
    doc: &GroundingDocument,
    docs: &[GroundingDocument],
    tree: &SubjectTree,
    semantic: &[ScoredDocument],
    limit: usize,
) -> Vec<RelatedEntry> {
    let candidates = docs.iter().filter(|other| other.id != doc.id && other.network_id == doc.network_id);
    let mut entries: Vec<RelatedEntry> = candidates
        .filter_map(|other| {
            let by_subject = subject_entry(doc, other, tree);
            let by_meaning = semantic
                .iter()
                .find(|s| s.id.as_str() == other.id)
                .map(|s| RelatedEntry { id: s.id.clone(), score: s.score, reason: RelatedReason::Semantic });
            match (by_subject, by_meaning) {
                (Some(a), Some(b)) => Some(if b.score > a.score { b } else { a }),
                (a, b) => a.or(b),
            }
        })
        .collect();
    entries.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    entries.truncate(limit);
    entries
}
