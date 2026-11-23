//! N1: how one post refers to another (an embed, a quote or a wikilink) and the backlinks of
//! a post: every post that refers to it, each once with its strongest reference. A locked
//! post's text is only its placeholder, so its sealed embeds and wikilinks are never read;
//! only its quote (which is not text) still refers.

use super::tokens::{embed_targets, wikilink_targets, BodyToken};
use super::Post;
use crate::domain::data::history::SemVer;
use crate::domain::document::keyed::keyed_enum;
use crate::domain::PostId;

keyed_enum! {
    /// How a post refers to another, strongest first. The key names its locale strings.
    RefKind {
        /// N1: `((target))` shows the target inside the post.
        Embed => "embed",
        /// The target as a quote card.
        Quote => "quote",
        /// `[[target]]` inline.
        Link => "link",
    }
}

/// One reference from a post.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostRef {
    pub target: PostId,
    pub kind: RefKind,
    /// The version an embed shows (`None`: the version the target shows).
    pub version: Option<SemVer>,
}

/// One post referring to another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backlink {
    pub from: PostId,
    pub kind: RefKind,
}

/// The references of `post` (body `tokens`): its embeds in order, its quote, then its
/// wikilinks; never itself, each target once per kind.
pub fn outgoing_refs(post: &Post, tokens: &[BodyToken]) -> Vec<PostRef> {
    let embeds = embed_targets(tokens).into_iter().map(|(target, version)| (target, RefKind::Embed, version));
    let quote = post.quote_id.iter().map(|id| (id.as_str().to_string(), RefKind::Quote, None));
    let links = wikilink_targets(tokens).into_iter().map(|target| (target, RefKind::Link, None));
    let mut refs: Vec<PostRef> = Vec::new();
    for (target, kind, version) in embeds.chain(quote).chain(links) {
        let target = PostId::new(target.trim());
        if target != post.id && !refs.iter().any(|r| r.target == target && r.kind == kind) {
            refs.push(PostRef { target, kind, version });
        }
    }
    refs
}

/// The backlinks of `target` among `referrers` (each post with its references, in the order
/// to list them): one per referring post, with its strongest reference.
pub fn backlinks<'a>(target: &PostId, referrers: impl IntoIterator<Item = (&'a PostId, &'a [PostRef])>) -> Vec<Backlink> {
    referrers
        .into_iter()
        .filter(|(from, _)| *from != target)
        .filter_map(|(from, refs)| {
            let kind = refs.iter().filter(|r| &r.target == target).map(|r| r.kind).min()?; // declared strongest first
            Some(Backlink { from: from.clone(), kind })
        })
        .collect()
}

#[cfg(test)]
mod tests;
