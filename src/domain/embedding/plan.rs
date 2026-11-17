//! The index diff: which chunks must be (re-)embedded, which owners lost trailing chunks and
//! which owners no longer exist. Pure, so it can run on a background thread.

use super::chunk::{chunk_text, ChunkSpec};
use super::hash::content_hash;
use super::index::VectorIndex;
use super::owner::EmbeddingOwner;
use std::collections::HashSet;

/// A post or document as text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceText {
    pub owner: EmbeddingOwner,
    /// Document title (blank for posts).
    pub title: String,
    pub text: String,
}

/// What is being indexed with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanSpec {
    pub chunk: ChunkSpec,
    pub model: String,
    /// Everything besides the text that changes a vector (provider, dimensions, task
    /// encoding); it is part of the content hash.
    pub signature: String,
    /// Re-embed everything (Settings › Rebuild the index).
    pub force: bool,
}

/// One chunk to embed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkJob {
    pub owner: EmbeddingOwner,
    pub chunk_index: usize,
    pub title: String,
    pub text: String,
    pub content_hash: String,
}

impl ChunkJob {
    /// Identity of the job's target row.
    pub fn key(&self) -> (EmbeddingOwner, usize) {
        (self.owner.clone(), self.chunk_index)
    }
}

/// The result of [`plan_index`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndexPlan {
    pub jobs: Vec<ChunkJob>,
    /// Owners whose stored chunks at or past the count must go: `(owner, chunk count)`.
    pub trims: Vec<(EmbeddingOwner, usize)>,
    /// Indexed owners that are no longer among the sources.
    pub removals: Vec<EmbeddingOwner>,
}

/// Diffs `sources` against `index`: unchanged chunks of the same model are skipped.
pub fn plan_index(sources: &[SourceText], index: &VectorIndex, spec: &PlanSpec) -> IndexPlan {
    let mut plan = IndexPlan::default();
    for source in sources {
        let chunks = chunk_text(&source.text, spec.chunk);
        let stored = index.chunks(&source.owner);
        if stored.iter().any(|chunk| chunk.index >= chunks.len()) {
            plan.trims.push((source.owner.clone(), chunks.len()));
        }
        let jobs = chunks.into_iter().map(|chunk| ChunkJob {
            owner: source.owner.clone(),
            chunk_index: chunk.index,
            content_hash: content_hash(&[&spec.signature, &source.title, &chunk.text]),
            title: source.title.clone(),
            text: chunk.text,
        });
        plan.jobs.extend(jobs.filter(|job| spec.force || is_stale(index, job, &spec.model)));
    }
    let live: HashSet<&EmbeddingOwner> = sources.iter().map(|source| &source.owner).collect();
    plan.removals = index.owners().filter(|owner| !live.contains(owner)).cloned().collect();
    plan
}

/// Whether `job`'s row is missing, from another model or from other input.
fn is_stale(index: &VectorIndex, job: &ChunkJob, model: &str) -> bool {
    index
        .chunk(&job.owner, job.chunk_index)
        .is_none_or(|stored| stored.model != model || stored.content_hash != job.content_hash)
}
