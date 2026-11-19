//! What an export holds: the scope picks posts and documents; review items follow what was
//! picked; agents are the network's profiles (everything) or the authors picked; settings
//! and the journal come along as the options say. The counts are the dialog's preview.

use super::threads::group_threads;
use super::{ExportOptions, ExportScope, ExportSource};
use crate::domain::data::saved::ItemRef;
use crate::domain::review::ReviewItem;
use crate::domain::{GroundingDocument, Post, PostId, Profile};
use jiff::Timestamp;
use jiff::civil::Date;
use std::collections::HashSet;

/// One thread of the export (only its selected posts).
#[derive(Debug, Clone, PartialEq)]
pub struct ThreadExport {
    pub root: PostId,
    pub posts: Vec<Post>,
}

/// How much of each kind goes in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Counts {
    pub threads: usize,
    pub posts: usize,
    pub documents: usize,
    pub originals: usize,
    pub images: usize,
    pub review_items: usize,
    pub agents: usize,
    pub settings: usize,
    pub journal_records: usize,
}

/// The selection, ready to be written.
#[derive(Debug, Clone)]
pub struct ExportPlan {
    pub options: ExportOptions,
    pub threads: Vec<ThreadExport>,
    pub documents: Vec<GroundingDocument>,
    pub review: Vec<ReviewItem>,
    pub agents: Vec<Profile>,
    pub counts: Counts,
}

fn local_day(at: Timestamp) -> Date {
    crate::time::local_date(at)
}

fn in_range(at: Timestamp, from: Date, to: Date) -> bool {
    (from..=to).contains(&local_day(at))
}

fn selected_posts<'a>(source: &'a ExportSource, scope: &ExportScope) -> Vec<&'a Post> {
    let collected: HashSet<&str> = match scope {
        ExportScope::Collection(id) => source.entries.iter().filter(|e| &e.collection == id).filter_map(|e| match &e.item { ItemRef::Post(post) => Some(post.as_str()), ItemRef::Document(_) => None }).collect(),
        _ => HashSet::new(),
    };
    let keep = |post: &&Post| match scope {
        ExportScope::Everything => true,
        ExportScope::Collection(_) => collected.contains(post.id.as_str()),
        ExportScope::DateRange { from, to } => in_range(post.created_at, *from, *to),
        ExportScope::Persona(persona) => &post.author_id == persona,
    };
    source.posts.iter().filter(|p| !p.is_repost() || !p.content.is_empty()).filter(keep).collect()
}

fn selected_documents(source: &ExportSource, scope: &ExportScope) -> Vec<GroundingDocument> {
    let persona_name = match scope {
        ExportScope::Persona(id) => source.authors.get(id).map(|p| p.name.clone()),
        _ => None,
    };
    let keep = |doc: &&GroundingDocument| match scope {
        ExportScope::Everything => true,
        ExportScope::Collection(id) => doc.record.collections.contains(id),
        ExportScope::DateRange { from, to } => in_range(doc.created_at, *from, *to),
        ExportScope::Persona(_) => persona_name.as_deref().is_some_and(|name| doc.record.provenance.added_by == name),
    };
    source.documents.iter().filter(keep).cloned().collect()
}

/// Review items anchored to what was picked (every item for "everything").
fn selected_review(source: &ExportSource, scope: &ExportScope, posts: &HashSet<&str>, documents: &HashSet<&str>) -> Vec<ReviewItem> {
    let anchored = |item: &&ReviewItem| match item.subject.anchor() {
        Some(crate::domain::review::Anchor::Post { id }) => posts.contains(id.as_str()),
        Some(crate::domain::review::Anchor::Passage { document_id, .. }) => documents.contains(document_id.as_str()),
        None => false,
    };
    source.review.iter().filter(|item| *scope == ExportScope::Everything || anchored(item)).cloned().collect()
}

fn selected_agents(source: &ExportSource, scope: &ExportScope, posts: &[&Post]) -> Vec<Profile> {
    if *scope == ExportScope::Everything {
        return source.profiles.clone();
    }
    let authors: HashSet<&str> = posts.iter().map(|p| p.author_id.as_str()).collect();
    let mut agents: Vec<Profile> = authors.into_iter().filter_map(|id| source.authors.get(id).cloned()).collect();
    agents.sort_by(|a, b| a.handle.cmp(&b.handle));
    agents
}

/// The export of `source` under `options`.
pub fn plan_export(source: &ExportSource, options: &ExportOptions) -> ExportPlan {
    let posts = selected_posts(source, &options.scope);
    let documents = selected_documents(source, &options.scope);
    let post_ids: HashSet<&str> = posts.iter().map(|p| p.id.as_str()).collect();
    let doc_ids: HashSet<&str> = documents.iter().map(|d| d.id.as_str()).collect();
    let review = selected_review(source, &options.scope, &post_ids, &doc_ids);
    let agents = selected_agents(source, &options.scope, &posts);
    let threads: Vec<ThreadExport> = group_threads(&source.posts, &posts).into_iter().map(|(root, posts)| ThreadExport { root, posts }).collect();
    let images = threads.iter().flat_map(|t| &t.posts).map(|p| super::bundle::post_images(p).len()).sum();
    let counts = Counts {
        threads: threads.len(),
        posts: posts.len(),
        documents: documents.len(),
        originals: if options.include_originals { documents.iter().filter(|d| !d.record.provenance.original.is_empty()).count() } else { 0 },
        images,
        review_items: review.len(),
        agents: agents.len(),
        settings: if options.scope == ExportScope::Everything { source.settings.len() } else { 0 },
        journal_records: source.journal.as_ref().filter(|_| options.include_journal).map_or(0, |book| journal_in_scope(book, &options.scope).1),
    };
    ExportPlan { options: options.clone(), threads, documents, review, agents, counts }
}

/// The journal's records in scope, as a JSON object of four lists, and how many there are.
pub(super) fn journal_in_scope(book: &crate::domain::journal::JournalBook, scope: &ExportScope) -> (serde_json::Value, usize) {
    let keep = |day: Date| match scope {
        ExportScope::DateRange { from, to } => (*from..=*to).contains(&day),
        _ => true,
    };
    let values = |items: Vec<serde_json::Value>| serde_json::Value::Array(items);
    let entries: Vec<_> = book.entries.iter().filter(|e| keep(e.day)).filter_map(|e| serde_json::to_value(e).ok()).collect();
    let checkins: Vec<_> = book.checkins.iter().filter(|c| keep(c.day)).filter_map(|c| serde_json::to_value(c).ok()).collect();
    let sessions: Vec<_> = book.sessions.iter().filter(|s| keep(s.day)).filter_map(|s| serde_json::to_value(s).ok()).collect();
    let capsules: Vec<_> = book.capsules.iter().filter(|c| keep(c.opens_on)).filter_map(|c| serde_json::to_value(c).ok()).collect();
    let count = entries.len() + checkins.len() + sessions.len() + capsules.len();
    let json = serde_json::json!({ "entries": values(entries), "checkins": values(checkins), "sessions": values(sessions), "capsules": values(capsules) });
    (json, count)
}
