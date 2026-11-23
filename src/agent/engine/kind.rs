//! What an engine job generates: a reply to a post, a new top-level post from a seed, a
//! progress reply on a to-do the post's author delegated to the agent (D12), or a reply through a lens (N16).

use super::reply::{build_reply, build_root_post};
use super::DialecticEngine;
use crate::domain::post::lens::Lens;
use crate::domain::post::{PostExtras, ResolvedSeed, SourceKind, SourceRef};
use crate::domain::{AuthoredPost, NetworkId, Post, Profile};

/// The kind of post an [`super::EngineJob`] produces.
#[derive(Debug, Clone, PartialEq)]
pub enum JobKind {
    /// Reply to (and mention the author of) this post.
    Reply(AuthoredPost),
    /// Start a new conversation, seeded by an optional topic and/or vault document.
    NewPost(ResolvedSeed),
    /// Reply to `parent` with progress on its to-do `item`, handed to the agent (D12).
    Progress { parent: AuthoredPost, item: String },
    /// N16: reply to `parent` through `lens`; the reply keeps its lens.
    Lens { parent: AuthoredPost, lens: Lens },
}

impl JobKind {
    /// The post the job answers (`None` for a new post).
    pub fn parent(&self) -> Option<&AuthoredPost> {
        match self {
            Self::Reply(parent) | Self::Progress { parent, .. } | Self::Lens { parent, .. } => Some(parent),
            Self::NewPost(_) => None,
        }
    }

    /// The user turn for `agent`.
    pub(crate) fn user_prompt(&self, agent: &Profile) -> String {
        match self {
            Self::Reply(parent) => DialecticEngine::persona_user_prompt(parent, agent),
            Self::NewPost(seed) => DialecticEngine::root_user_prompt(agent, seed),
            Self::Progress { parent, item } => DialecticEngine::progress_user_prompt(parent, agent, item),
            Self::Lens { parent, lens } => format!("{}\n\n{}", DialecticEngine::persona_user_prompt(parent, agent), lens.prompt_line()),
        }
    }

    /// The offline text used when no model could be reached.
    pub(super) fn heuristic_text(&self, agent: &Profile) -> String {
        match self {
            Self::Reply(parent) | Self::Lens { parent, .. } => DialecticEngine::generate_heuristic_text(parent, agent),
            Self::NewPost(seed) => DialecticEngine::generate_heuristic_root_text(agent, seed),
            Self::Progress { item, .. } => DialecticEngine::generate_progress_text(agent, item),
        }
    }

    /// The post carrying `content` and `extras`; a new post also cites its seed document.
    pub(super) fn post(&self, agent: Profile, network: NetworkId, content: String, extras: PostExtras) -> Post {
        let mut post = match self {
            Self::Reply(parent) | Self::Progress { parent, .. } | Self::Lens { parent, .. } => build_reply(parent, agent, network, content),
            Self::NewPost(_) => build_root_post(agent, network, content),
        };
        post.extras = extras;
        if let Self::Lens { lens, .. } = self {
            post.extras.lens = Some(*lens);
        }
        if let Self::NewPost(ResolvedSeed { document: Some(doc), .. }) = self {
            let sources = &mut post.extras.sources;
            if !sources.iter().any(|s| s.kind == SourceKind::VaultDoc && s.id == doc.id) {
                sources.push(SourceRef::new(SourceKind::VaultDoc, &doc.id, &doc.title));
            }
        }
        post
    }
}
