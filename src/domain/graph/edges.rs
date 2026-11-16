//! Edge extraction from a single post, collecting hashtag and profile nodes on the way.

use super::resolve::{normalize_tag, Lookup};
use super::{EdgeKind, NodeRef};
use crate::domain::{Post, Profile, ProfileId};
use std::collections::{BTreeMap, BTreeSet};

/// Edges by node identity plus the derived (hashtag, profile) node sets.
#[derive(Default)]
pub(super) struct RawGraph {
    pub edges: Vec<(NodeRef, NodeRef, EdgeKind)>,
    /// Normalized hashtags, sorted.
    pub tags: BTreeSet<String>,
    /// Profile id to display name, sorted by id.
    pub profiles: BTreeMap<ProfileId, String>,
}

impl RawGraph {
    /// Registers `profile` as a node.
    pub(super) fn add_profile(&mut self, profile: &Profile) {
        self.profiles.entry(profile.id.clone()).or_insert_with(|| profile.name.clone());
    }

    /// Adds every edge originating from `post` (and its authorship edge).
    pub(super) fn add_post(&mut self, post: &Post, lookup: &Lookup<'_>) {
        let source = NodeRef::Post(post.id.clone());
        self.add_relations(&source, post, lookup);
        self.add_wikilinks(&source, post, lookup);
        self.add_tags(&source, post);
        self.add_mentions(&source, post, lookup);
        self.add_profile(&post.author);
        self.push(NodeRef::Profile(post.author.id.clone()), source, EdgeKind::Authored);
    }

    fn add_relations(&mut self, source: &NodeRef, post: &Post, lookup: &Lookup<'_>) {
        let relations = [
            (&post.reply_to_id, EdgeKind::Reply),
            (&post.quote_id, EdgeKind::Quote),
            (&post.repost_id, EdgeKind::Repost),
        ];
        for (target_id, kind) in relations {
            if let Some(target) = target_id.as_ref().and_then(|id| lookup.post(id)) {
                self.push(source.clone(), target, kind);
            }
        }
    }

    fn add_wikilinks(&mut self, source: &NodeRef, post: &Post, lookup: &Lookup<'_>) {
        for target in Post::parse_linked_posts(&post.content) {
            if let Some(node) = lookup.wikilink(target.as_str()) {
                self.push(source.clone(), node, EdgeKind::Wikilink);
            }
        }
    }

    fn add_tags(&mut self, source: &NodeRef, post: &Post) {
        for tag in Post::parse_hashtags(&post.content) {
            let key = normalize_tag(&tag);
            if key.is_empty() {
                continue;
            }
            self.tags.insert(key.clone());
            self.push(source.clone(), NodeRef::Hashtag(key), EdgeKind::Hashtag);
        }
    }

    fn add_mentions(&mut self, source: &NodeRef, post: &Post, lookup: &Lookup<'_>) {
        for handle in &post.mentions {
            if let Some(profile) = lookup.mention(handle) {
                self.add_profile(profile);
                self.push(source.clone(), NodeRef::Profile(profile.id.clone()), EdgeKind::Mention);
            }
        }
    }

    /// Records an edge unless it is a self-loop.
    fn push(&mut self, from: NodeRef, to: NodeRef, kind: EdgeKind) {
        if from != to {
            self.edges.push((from, to, kind));
        }
    }
}
