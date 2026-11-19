//! The whole Insights report, computed in one pure pass from an owned snapshot of the store
//! (so it can run on a background thread) and the page's filter.

mod families;
mod structure;
#[cfg(test)]
mod tests;

pub use structure::{CategoryReport, SymmetryReport, ThreadSymmetry};

use super::activity::ActivityReport;
use super::embedding::EmbeddingMap;
use super::facts::{AuthorFacts, PostFacts, PostKind};
use super::filter::InsightsFilter;
use super::network::{GraphScope, NetworkReport};
use super::sets::{NamedSet, SetDimension};
use crate::domain::document::SubjectId;
use crate::domain::graph::{excerpt, GraphModel, GraphSource};
use crate::domain::{GroundingDocument, NetworkId, Post, PostId, Profile};
use jiff::Timestamp;
use jiff::tz::Offset;
use std::collections::HashMap;

/// Characters of a post excerpt in tooltips.
pub const EXCERPT_CHARS: usize = 96;

/// Everything the report is computed from.
#[derive(Debug, Clone)]
pub struct InsightsInput {
    pub network: NetworkId,
    pub posts: Vec<Post>,
    pub documents: Vec<GroundingDocument>,
    pub profiles: Vec<Profile>,
    pub subjects: Vec<(SubjectId, String)>,
    /// Post id and unit embedding vector (one model).
    pub vectors: Vec<(String, Vec<f32>)>,
    pub now: Timestamp,
    pub offset: Option<Offset>,
}

/// A post as the charts name it.
#[derive(Debug, Clone, PartialEq)]
pub struct PostSummary {
    pub id: PostId,
    pub author: usize,
    pub kind: PostKind,
    pub excerpt: String,
    pub created_at: Timestamp,
}

/// Every view's data.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InsightsReport {
    pub posts: Vec<PostSummary>,
    pub authors: Vec<AuthorFacts>,
    pub documents: usize,
    pub activity: ActivityReport,
    pub network: NetworkReport,
    pub families: Vec<(SetDimension, Vec<NamedSet>)>,
    pub symmetry: SymmetryReport,
    pub category: CategoryReport,
    pub embedding: EmbeddingMap,
    /// Post index of each embedding point.
    pub embedding_posts: Vec<usize>,
    /// The most common concept of each cluster (empty when none).
    pub cluster_labels: Vec<String>,
}

impl InsightsReport {
    /// Computes the report of `input` under `filter`, the Network view showing `scope`.
    pub fn compute(input: &InsightsInput, filter: &InsightsFilter, scope: GraphScope) -> Self {
        let offset = input.offset.unwrap_or_else(|| Offset::UTC);
        let profile_of: HashMap<&str, &Profile> = input.profiles.iter().map(|p| (p.id.as_str(), p)).collect();
        let posts: Vec<Post> = input
            .posts
            .iter()
            .filter(|p| filter.accepts(p, profile_of.get(p.author_id.as_str()).copied(), input.now, &input.network))
            .cloned()
            .collect();
        let authors = authors_of(&posts, &input.profiles);
        let author_index: HashMap<&str, usize> = authors.iter().enumerate().map(|(i, a)| (a.id.as_str(), i)).collect();
        let author_of = |p: &Post| author_index.get(p.author_id.as_str()).copied().unwrap_or_default();
        let facts: Vec<PostFacts> = posts.iter().map(|p| PostFacts::of(p, author_of(p), offset)).collect();
        let today = offset.to_datetime(input.now).date();
        let start = filter.range.days().map(|days| today - crate::time::days(days - 1));
        let summaries = posts.iter().zip(&facts).map(|(p, f)| summary(p, f)).collect();
        let model = GraphModel::build(&GraphSource { posts: &posts, documents: &input.documents, profiles: &input.profiles, network: &input.network });
        let mut report = Self {
            activity: ActivityReport::of(&facts, &authors, start, today),
            network: NetworkReport::of(&model, scope),
            families: families::families(&posts, &facts, &authors, &model, input),
            symmetry: structure::symmetry(&posts, &facts),
            category: structure::category(&model),
            documents: input.documents.iter().filter(|d| filter.all_networks || d.network_id == input.network).count(),
            posts: summaries,
            authors,
            ..Self::default()
        };
        families::attach_embedding(&mut report, input);
        report
    }

    /// Index of post `id` among the analysed posts.
    pub fn post_index(&self, id: &PostId) -> Option<usize> {
        self.posts.iter().position(|p| &p.id == id)
    }
}

fn summary(post: &Post, facts: &PostFacts) -> PostSummary {
    PostSummary { id: post.id.clone(), author: facts.author, kind: facts.kind, excerpt: excerpt(&post.content, EXCERPT_CHARS), created_at: post.created_at }
}

/// Authors of `posts` in order of first appearance (unknown authors keep their id as name).
fn authors_of(posts: &[Post], profiles: &[Profile]) -> Vec<AuthorFacts> {
    let mut seen: Vec<AuthorFacts> = Vec::new();
    for post in posts {
        if seen.iter().any(|a| a.id == post.author_id) {
            continue;
        }
        let facts = profiles.iter().find(|p| p.id == post.author_id).map(AuthorFacts::of);
        seen.push(facts.unwrap_or_else(|| AuthorFacts { id: post.author_id.clone(), name: post.author_id.as_str().to_string(), is_agent: false, traits: None }));
    }
    seen
}
