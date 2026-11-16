//! Lookup tables resolving post ids, wikilink targets and `@handles` within one network.

use super::NodeRef;
use crate::domain::post::HASHTAG_SIGIL;
use crate::domain::profile::handle_key;
use crate::domain::{GroundingDocument, Post, PostId, Profile, ProfileId};
use std::collections::{HashMap, HashSet};

/// Lower-cased handle without its `@` prefix.
pub(super) fn normalize_handle(handle: &str) -> String {
    handle_key(handle)
}

/// Lower-cased tag without its `#` sigil.
pub(super) fn normalize_tag(tag: &str) -> String {
    tag.trim().trim_start_matches(HASHTAG_SIGIL).to_lowercase()
}

/// Resolution tables built from the network's posts, documents and profiles.
pub(super) struct Lookup<'a> {
    post_ids: HashSet<&'a str>,
    doc_ids: HashSet<&'a str>,
    docs_by_title: HashMap<String, &'a str>,
    profiles_by_handle: HashMap<String, &'a Profile>,
    authors_by_id: HashMap<&'a ProfileId, &'a Profile>,
}

impl<'a> Lookup<'a> {
    pub(super) fn new(
        posts: &[&'a Post],
        docs: &[&'a GroundingDocument],
        profiles: &[&'a Profile],
        authors: &'a [Profile],
    ) -> Self {
        let mut profiles_by_handle = HashMap::new();
        for profile in profiles {
            profiles_by_handle.entry(normalize_handle(&profile.handle)).or_insert(*profile);
        }
        Self {
            post_ids: posts.iter().map(|p| p.id.as_str()).collect(),
            doc_ids: docs.iter().map(|d| d.id.as_str()).collect(),
            docs_by_title: docs.iter().map(|d| (d.title.trim().to_lowercase(), d.id.as_str())).collect(),
            profiles_by_handle,
            authors_by_id: authors.iter().map(|p| (&p.id, p)).collect(),
        }
    }

    /// Display name of the author `id` (the id itself when the profile is unknown).
    pub(super) fn author_name(&self, id: &ProfileId) -> String {
        self.authors_by_id.get(id).map_or_else(|| id.to_string(), |p| p.name.clone())
    }

    /// Node of a post of this network.
    pub(super) fn post(&self, id: &PostId) -> Option<NodeRef> {
        self.post_ids.contains(id.as_str()).then(|| NodeRef::Post(id.clone()))
    }

    /// Node named by a `[[target]]` wikilink: a post id, a document id or a document title.
    pub(super) fn wikilink(&self, target: &str) -> Option<NodeRef> {
        let target = target.trim();
        if self.post_ids.contains(target) {
            return Some(NodeRef::Post(PostId::new(target)));
        }
        if self.doc_ids.contains(target) {
            return Some(NodeRef::Document(target.to_string()));
        }
        let doc_id = self.docs_by_title.get(&target.to_lowercase())?;
        Some(NodeRef::Document((*doc_id).to_string()))
    }

    /// Profile mentioned by `@handle`.
    pub(super) fn mention(&self, handle: &str) -> Option<&'a Profile> {
        self.profiles_by_handle.get(&normalize_handle(handle)).copied()
    }
}
