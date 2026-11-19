//! Lenses (sb F3): three more Insights views, computed in one pure pass from an owned
//! snapshot under the page's filter, so the shell runs them on the background executor.
//!
//! - [`timeline`]: concepts over time (posts, vault documents and review items): first
//!   appearance, peak, decline and "quiet since".
//! - [`diversity`]: how varied the analysed posts and the vault reading are: entropy and
//!   Simpson index over authors, stances and schools, spread in meaning, replies that agree or
//!   push back, the trend, and what was mostly read lately.
//! - [`readability`]: per-post scores of `domain::readability`, per author, over time, and
//!   the hardest and easiest posts.
//!
//! Only posts, vault documents and review items go in: journal, mood, focus and capsule
//! records are private and have no field here.

pub mod diversity;
pub mod periods;
pub mod readability;
pub mod sources;
pub mod timeline;
#[cfg(test)]
mod tests;

pub use periods::Periods;
pub use sources::{review_source, ReviewSource};

use super::facts::{AuthorFacts, PostKind};
use super::filter::InsightsFilter;
use super::report::{PostSummary, EXCERPT_CHARS};
use crate::domain::document::SubjectId;
use crate::domain::graph::excerpt;
use crate::domain::readability::ReadingLanguage;
use crate::domain::{GroundingDocument, NetworkId, Post, Profile};
use jiff::Timestamp;
use jiff::civil::Date;
use jiff::tz::Offset;
use std::collections::HashMap;

/// Everything the lenses are computed from.
#[derive(Debug, Clone)]
pub struct LensInput {
    pub network: NetworkId,
    /// Every post of the store's networks (replies find their parents here).
    pub posts: Vec<Post>,
    pub documents: Vec<GroundingDocument>,
    pub profiles: Vec<Profile>,
    pub subjects: Vec<(SubjectId, String)>,
    pub review: Vec<ReviewSource>,
    /// Post id and unit embedding vector (one model).
    pub vectors: Vec<(String, Vec<f32>)>,
    pub language: ReadingLanguage,
    pub now: Timestamp,
    pub offset: Option<Offset>,
}

impl LensInput {
    /// A snapshot of `network` at `now` with nothing in it yet.
    pub fn empty(network: NetworkId, now: Timestamp) -> Self {
        let (posts, documents, profiles, subjects, review, vectors) = Default::default();
        Self { network, posts, documents, profiles, subjects, review, vectors, language: ReadingLanguage::default(), now, offset: None }
    }
}

/// The three lenses.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LensReport {
    pub posts: Vec<PostSummary>,
    pub authors: Vec<AuthorFacts>,
    pub timeline: timeline::ConceptTimeline,
    pub diversity: diversity::DiversityReport,
    pub readability: readability::ReadabilityReport,
}

/// The analysed posts and what the lenses share about them.
pub struct Analysed<'a> {
    pub posts: Vec<&'a Post>,
    pub authors: Vec<AuthorFacts>,
    /// Author index of each analysed post.
    pub author_of: Vec<usize>,
    pub offset: Offset,
    pub today: Date,
}

impl Analysed<'_> {
    /// Local day of `at`.
    pub fn day(&self, at: Timestamp) -> Date {
        self.offset.to_datetime(at).date()
    }
}

impl LensReport {
    /// Computes every lens of `input` under `filter`.
    pub fn compute(input: &LensInput, filter: &InsightsFilter) -> Self {
        let analysed = analyse(input, filter);
        let posts = analysed.posts.iter().zip(&analysed.author_of).map(|(p, &author)| summary(p, author)).collect();
        Self {
            timeline: timeline::ConceptTimeline::of(sources::all(input, &analysed, filter), analysed.today),
            diversity: diversity::DiversityReport::of(input, &analysed),
            readability: readability::ReadabilityReport::of(&analysed, input.language),
            authors: analysed.authors.clone(),
            posts,
        }
    }
}

/// The posts `filter` lets through, their authors (first appearance order) and the clock.
pub fn analyse<'a>(input: &'a LensInput, filter: &InsightsFilter) -> Analysed<'a> {
    let offset = input.offset.unwrap_or_else(|| Offset::UTC);
    let profile_of: HashMap<&str, &Profile> = input.profiles.iter().map(|p| (p.id.as_str(), p)).collect();
    let posts: Vec<&Post> = input.posts.iter().filter(|p| filter.accepts(p, profile_of.get(p.author_id.as_str()).copied(), input.now, &input.network)).collect();
    let mut index: HashMap<&str, usize> = HashMap::new();
    let mut authors: Vec<AuthorFacts> = Vec::new();
    let author_of = posts
        .iter()
        .map(|post| {
            *index.entry(post.author_id.as_str()).or_insert_with(|| {
                let facts = profile_of.get(post.author_id.as_str()).map(|p| AuthorFacts::of(p));
                authors.push(facts.unwrap_or_else(|| AuthorFacts { id: post.author_id.clone(), name: post.author_id.as_str().to_string(), is_agent: false, traits: None }));
                authors.len() - 1
            })
        })
        .collect();
    Analysed { posts, authors, author_of, offset, today: offset.to_datetime(input.now).date() }
}

fn summary(post: &Post, author: usize) -> PostSummary {
    PostSummary { id: post.id.clone(), author, kind: PostKind::of(post), excerpt: excerpt(&post.content, EXCERPT_CHARS), created_at: post.created_at }
}
