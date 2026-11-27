//! Deterministic assembly of a [`GraphModel`] from store collections.

use super::edges::RawGraph;
use super::label::{excerpt, LABEL_MAX_CHARS};
use super::resolve::Lookup;
use super::{GraphEdge, GraphModel, GraphNode, NodeRef};
use crate::domain::post::{readable_line, HASHTAG_SIGIL};
use crate::domain::{GroundingDocument, NetworkId, Post, Profile};
use std::collections::HashMap;

/// Store collections and the network the graph is scoped to.
pub struct GraphSource<'a> {
    pub posts: &'a [Post],
    pub documents: &'a [GroundingDocument],
    pub profiles: &'a [Profile],
    pub network: &'a NetworkId,
}

impl GraphSource<'_> {
    /// Posts of the network, oldest first (ties by id).
    fn posts(&self) -> Vec<&Post> {
        let mut posts: Vec<&Post> = self.posts.iter().filter(|p| &p.network_id == self.network).collect();
        posts.sort_by(|a, b| (a.created_at, &a.id).cmp(&(b.created_at, &b.id)));
        posts.dedup_by(|a, b| a.id == b.id);
        posts
    }

    /// Documents of the network, by id.
    fn documents(&self) -> Vec<&GroundingDocument> {
        let mut docs: Vec<&GroundingDocument> =
            self.documents.iter().filter(|d| &d.network_id == self.network).collect();
        docs.sort_by(|a, b| a.id.cmp(&b.id));
        docs.dedup_by(|a, b| a.id == b.id);
        docs
    }

    /// Profiles mentionable in the network: members first, then post authors.
    fn mentionable<'a>(&'a self, posts: &[&'a Post]) -> Vec<&'a Profile> {
        let members = self.profiles.iter().filter(|p| p.belongs_to_network(self.network));
        let authors = posts.iter().filter_map(|post| self.profiles.iter().find(|p| p.id == post.author_id));
        members.chain(authors).collect()
    }
}

/// Label of a post node: its excerpt read as text, or the author's name for text-less reposts.
fn post_label(post: &Post, lookup: &Lookup<'_>) -> String {
    let text = excerpt(&readable_line(&post.content), LABEL_MAX_CHARS);
    if text.is_empty() {
        excerpt(&lookup.author_name(&post.author_id), LABEL_MAX_CHARS)
    } else {
        text
    }
}

fn node(node: NodeRef, label: String) -> GraphNode {
    GraphNode { node, label }
}

impl GraphModel {
    /// Builds the graph of `source.network` with stable node and edge order:
    /// posts (oldest first), documents, hashtags, then profiles.
    pub fn build(source: &GraphSource<'_>) -> Self {
        let posts = source.posts();
        let docs = source.documents();
        let lookup = Lookup::new(&posts, &docs, &source.mentionable(&posts), source.profiles);
        let mut raw = RawGraph::default();
        let agents = source.profiles.iter().filter(|p| p.is_agent() && p.belongs_to_network(source.network));
        agents.for_each(|agent| raw.add_profile(agent));
        posts.iter().for_each(|post| raw.add_post(post, &lookup));
        Self::assemble(&posts, &docs, raw, &lookup)
    }

    fn assemble(posts: &[&Post], docs: &[&GroundingDocument], raw: RawGraph, lookup: &Lookup<'_>) -> Self {
        let mut nodes: Vec<GraphNode> = Vec::new();
        nodes.extend(posts.iter().map(|p| node(NodeRef::Post(p.id.clone()), post_label(p, lookup))));
        nodes.extend(docs.iter().map(|d| node(NodeRef::Document(d.id.clone()), excerpt(&d.title, LABEL_MAX_CHARS))));
        nodes.extend(raw.tags.iter().map(|t| node(NodeRef::Hashtag(t.clone()), format!("{HASHTAG_SIGIL}{t}"))));
        nodes.extend(
            raw.profiles.iter().map(|(id, name)| node(NodeRef::Profile(id.clone()), excerpt(name, LABEL_MAX_CHARS))),
        );
        let index: HashMap<NodeRef, usize> =
            nodes.iter().enumerate().map(|(i, n)| (n.node.clone(), i)).collect();
        let mut edges: Vec<GraphEdge> = raw
            .edges
            .iter()
            .filter_map(|(from, to, kind)| {
                Some(GraphEdge { source: *index.get(from)?, target: *index.get(to)?, kind: *kind })
            })
            .collect();
        edges.sort_unstable();
        edges.dedup();
        Self { nodes, edges, index }
    }
}
