//! N1: the reverse of every post's references, kept by the post index: which posts embed,
//! quote or link to a post, so each card can list its backlinks without scanning the store.

use crate::domain::post::refs::{backlinks, Backlink, PostRef};
use crate::domain::PostId;
use std::collections::HashMap;

/// The posts referring to each post, with their references to it.
#[derive(Debug, Default)]
pub struct Backrefs {
    by_target: HashMap<PostId, Vec<(PostId, Vec<PostRef>)>>,
}

impl Backrefs {
    /// Records the references `refs` of the post `from`.
    pub fn add(&mut self, from: &PostId, refs: &[PostRef]) {
        for target in distinct_targets(refs) {
            let mine: Vec<PostRef> = refs.iter().filter(|r| &r.target == target).cloned().collect();
            self.by_target.entry(target.clone()).or_default().push((from.clone(), mine));
        }
    }

    /// Forgets the references `refs` of the post `from`.
    pub fn remove(&mut self, from: &PostId, refs: &[PostRef]) {
        for target in distinct_targets(refs) {
            if let Some(list) = self.by_target.get_mut(target) {
                list.retain(|(referrer, _)| referrer != from);
                if list.is_empty() {
                    self.by_target.remove(target);
                }
            }
        }
    }

    /// The backlinks of `target`, one per referring post, in no particular order.
    pub fn of(&self, target: &PostId) -> Vec<Backlink> {
        let list = self.by_target.get(target).map(Vec::as_slice).unwrap_or_default();
        backlinks(target, list.iter().map(|(from, refs)| (from, refs.as_slice())))
    }
}

fn distinct_targets(refs: &[PostRef]) -> Vec<&PostId> {
    let mut targets: Vec<&PostId> = refs.iter().map(|r| &r.target).collect();
    targets.sort();
    targets.dedup();
    targets
}

impl super::PostIndex {
    /// Re-derives the body data of `post`, whose text was edited (sb F2), and its references.
    pub fn content_changed(&mut self, post: &crate::domain::Post) {
        let meta = super::PostMeta::of(post);
        if let Some(old) = self.meta.insert(post.id.clone(), meta.clone()) {
            self.backrefs.remove(&post.id, &old.refs);
        }
        self.backrefs.add(&post.id, &meta.refs);
        self.versions.forget(&post.id);
        self.revision += 1;
    }

    /// N1: the posts referring to `id` (unordered).
    pub fn backlinks(&self, id: &PostId) -> Vec<Backlink> {
        self.backrefs.of(id)
    }
}

impl super::AppState {
    /// N1: the posts of the active network referring to `id`, newest first.
    pub fn backlinks(&self, id: &PostId) -> Vec<Backlink> {
        let mut links: Vec<(usize, Backlink)> = self
            .index
            .backlinks(id)
            .into_iter()
            .filter_map(|link| self.post_position(&link.from).filter(|&at| self.posts[at].network_id == self.active_network_id).map(|at| (at, link)))
            .collect();
        links.sort_by_key(|(at, _)| *at);
        links.into_iter().map(|(_, link)| link).collect()
    }
}
