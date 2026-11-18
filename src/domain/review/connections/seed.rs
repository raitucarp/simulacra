//! Maps seeded from structure that already exists, so the learner starts from the real
//! connections and only edits them:
//! - a thread: its posts, joined by replies;
//! - a selection of posts: joined by replies, quotes, wikilinks and cited posts;
//! - both: hashtags shared by two posts or more become concept points linked to them;
//! - a vault document: its related entries and the posts that link to it.

use super::{pair, ConnectionMap, MapEdge, MapPoint, MAX_POINTS};
use crate::domain::post::SourceKind;
use crate::domain::{Post, PostId};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Most concept points made from shared hashtags.
pub const SHARED_CONCEPTS: usize = 3;
/// Most linking posts added around a document.
pub const DOCUMENT_POSTS: usize = 3;
const TAG_SIGIL: char = '#';

/// The thread `start` belongs to: from its first post, replies breadth first (oldest
/// first at each level), at most [`MAX_POINTS`] posts.
pub fn thread_posts<'a>(posts: &'a [Post], start: &PostId) -> Vec<&'a Post> {
    let by_id: HashMap<&PostId, &Post> = posts.iter().map(|post| (&post.id, post)).collect();
    let mut root = by_id.get(start).copied();
    let mut seen: HashSet<&PostId> = HashSet::new();
    while let Some(parent) = root.and_then(|p| p.reply_to_id.as_ref()).and_then(|id| by_id.get(id).copied()).filter(|p| seen.insert(&p.id)) {
        root = Some(parent);
    }
    let mut line: Vec<&Post> = root.into_iter().collect();
    let mut cursor = 0;
    while cursor < line.len() && line.len() < MAX_POINTS {
        let parent = &line[cursor].id;
        let mut replies: Vec<&Post> = posts.iter().filter(|p| p.reply_to_id.as_ref() == Some(parent) && !line.iter().any(|seen| seen.id == p.id)).collect();
        replies.sort_by_key(|p| p.created_at);
        line.extend(replies.into_iter().take(MAX_POINTS - line.len()));
        cursor += 1;
    }
    line
}

/// Posts `post` points at: its parent, its quote, its wikilinks and cited posts.
fn linked_ids(post: &Post) -> Vec<String> {
    let relations = [&post.reply_to_id, &post.quote_id].into_iter().flatten().map(|id| id.as_str().to_string());
    let wikilinks = post.linked_posts.iter().chain(Post::parse_linked_posts(&post.content).iter()).map(|id| id.as_str().trim().to_string()).collect::<Vec<_>>();
    let cited = post.extras.sources.iter().filter(|s| s.kind == SourceKind::Post).map(|s| s.id.clone());
    relations.chain(wikilinks).chain(cited).collect()
}

/// Adds the connection `a`–`b` unless the pair is already joined.
fn push_edge(edges: &mut Vec<MapEdge>, a: usize, b: usize) {
    if a != b && !edges.iter().any(|edge| edge.pair() == pair(a, b)) {
        edges.push(MapEdge::new(a, b));
    }
}

/// Lower-case hashtags of `post` without their sigil, each once.
fn tags_of(post: &Post) -> Vec<String> {
    let mut tags: Vec<String> = Post::parse_hashtags(&post.content).into_iter().map(|t| t.trim_start_matches(TAG_SIGIL).to_lowercase()).filter(|t| !t.is_empty()).collect();
    tags.sort();
    tags.dedup();
    tags
}

/// Concept points for hashtags shared by two posts or more, joined to those posts.
fn add_shared_concepts(map: ConnectionMap, posts: &[&Post]) -> ConnectionMap {
    let mut holders: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    posts.iter().enumerate().for_each(|(i, post)| tags_of(post).into_iter().for_each(|tag| holders.entry(tag).or_default().push(i)));
    let mut shared: Vec<(String, Vec<usize>)> = holders.into_iter().filter(|(_, at)| at.len() > 1).collect();
    shared.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));
    let room = MAX_POINTS.saturating_sub(map.points.len()).min(SHARED_CONCEPTS);
    shared.into_iter().take(room).fold(map, |mut map, (tag, at)| {
        let concept = map.points.len();
        map.points.push(MapPoint::Concept { label: format!("{TAG_SIGIL}{tag}") });
        at.into_iter().for_each(|i| push_edge(&mut map.edges, i, concept));
        map
    })
}

/// A map of `posts` (the first [`MAX_POINTS`], each once) and the connections among them.
pub fn map_from_posts(title: String, posts: &[&Post]) -> ConnectionMap {
    let unique: Vec<&Post> = posts.iter().fold(Vec::new(), |mut unique: Vec<&Post>, post| {
        if unique.len() < MAX_POINTS && !unique.iter().any(|u| u.id == post.id) {
            unique.push(post);
        }
        unique
    });
    let index: HashMap<&str, usize> = unique.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect();
    let mut edges = Vec::new();
    for (i, post) in unique.iter().enumerate() {
        linked_ids(post).iter().filter_map(|id| index.get(id.as_str())).for_each(|&j| push_edge(&mut edges, i, j));
    }
    let points = unique.iter().map(|p| MapPoint::Post { id: p.id.as_str().to_string() }).collect();
    add_shared_concepts(ConnectionMap { title, points, edges }, &unique)
}

/// A document and what surrounds it in the vault.
pub struct DocumentNeighbourhood<'a> {
    pub document_id: &'a str,
    /// Related entries, best first.
    pub related: Vec<&'a str>,
    /// Posts that link to the document, newest first.
    pub linking_posts: Vec<&'a Post>,
}

/// A map of a document, its related entries (joined to it, and to each other where
/// `related_pair` says so) and up to [`DOCUMENT_POSTS`] posts that link to it.
pub fn map_from_document(title: String, around: &DocumentNeighbourhood<'_>, related_pair: &dyn Fn(&str, &str) -> bool) -> ConnectionMap {
    let room = MAX_POINTS - 1;
    let posts: Vec<&Post> = around.linking_posts.iter().take(DOCUMENT_POSTS.min(room)).copied().collect();
    let documents: Vec<&str> = std::iter::once(around.document_id).chain(around.related.iter().copied().take(room - posts.len())).collect();
    let mut edges = Vec::new();
    (1..documents.len()).for_each(|i| push_edge(&mut edges, 0, i));
    for (i, a) in documents.iter().enumerate().skip(1) {
        documents.iter().enumerate().skip(i + 1).filter(|(_, b)| related_pair(a, b)).for_each(|(j, _)| push_edge(&mut edges, i, j));
    }
    (0..posts.len()).for_each(|k| push_edge(&mut edges, 0, documents.len() + k));
    let passage = |id: &&str| MapPoint::Passage { document_id: id.to_string(), locator: None, text: String::new() };
    let points = documents.iter().map(passage).chain(posts.iter().map(|p| MapPoint::Post { id: p.id.as_str().to_string() })).collect();
    ConnectionMap { title, points, edges }
}
