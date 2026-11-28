//! In-memory test data for live cases: an agent, a post to answer, vault documents, the tool
//! snapshot, a credential store holding the key, and engine plans. Nothing touches the
//! user's database.

use super::harness::{Live, GEMINI_OPENAI_URL};
use super::report::media_dir;
use simulacra::agent::credentials::{Credential, CredentialKind, CredentialStore};
use simulacra::agent::engine::{EngineJob, JobKind, PromptContext};
use simulacra::agent::runtime::EnginePlan;
use simulacra::agent::tools::{DocumentSnapshot, ToolContext};
use simulacra::domain::engine::EngineConfig;
use simulacra::domain::post::{PostSeed, ResolvedSeed};
use simulacra::domain::{AuthoredPost, GroundingDocument, NetworkId, Post, Profile};

/// Credential of kind Gemini (text, images, custom requests).
pub const GEMINI_CRED: &str = "cred_live_gemini";
/// Credential of kind Custom pointing at Gemini's OpenAI-compatible endpoint.
pub const OPENAI_COMPAT_CRED: &str = "cred_live_openai_compat";
pub const NETWORK: &str = "net_live";
pub const PARENT_ID: &str = "post_live_root";
pub const ETHICS_DOC: &str = "doc_ethics";
const ETHICS_TEXT: &str = "Spinoza's Ethics argues that there is one substance, God or Nature, and that mind and body are two attributes of it. \
He ground optical lenses for a living in The Hague.";
const BREAD_TEXT: &str = "Knead the dough for ten minutes, let it rise for an hour, then bake at 230 degrees.";

pub fn agent() -> Profile {
    Profile::new_agent("agent_spinoza", "Baruch", "@baruch", "Lens grinder and philosopher")
}

pub fn parent() -> AuthoredPost {
    let author = Profile::new_primary("user_live", "Arya", "@arya");
    let post = Post::create(PARENT_ID, NETWORK, author.id.clone(), "Is the mind just the body seen from inside?").expect("post");
    AuthoredPost::new(post, author)
}

/// A new-post seed: a topic plus the Ethics document.
pub fn seed() -> ResolvedSeed {
    let docs = vec![GroundingDocument::new(ETHICS_DOC, NETWORK, "Ethics", ETHICS_TEXT)];
    PostSeed { topic: "What does grinding lenses teach about seeing?".into(), ..PostSeed::document(ETHICS_DOC) }.resolve(&docs)
}

/// The tool snapshot: two vault documents.
pub fn tool_context() -> ToolContext {
    let doc = |id: &str, title: &str, content: &str| DocumentSnapshot { id: id.into(), title: title.into(), content: content.into() };
    ToolContext { documents: vec![doc(ETHICS_DOC, "Ethics", ETHICS_TEXT), doc("doc_bread", "Bread", BREAD_TEXT)], ..ToolContext::default() }
}

/// A store with the key twice: as a Gemini credential and as a Custom credential for the
/// OpenAI-compatible endpoint.
pub fn store(live: &Live) -> CredentialStore {
    let mut store = CredentialStore::default();
    let credential = |id: &str, kind, endpoint: &str| Credential { id: id.into(), kind, label: "Live".into(), secret: live.key().to_string(), endpoint: endpoint.into() };
    store.upsert(credential(GEMINI_CRED, CredentialKind::Gemini, ""));
    store.upsert(credential(OPENAI_COMPAT_CRED, CredentialKind::Custom, GEMINI_OPENAI_URL));
    store
}

/// `engine` resolved against the live settings and store (images go under `target/live-media`).
pub fn plan(live: &Live, engine: &EngineConfig) -> EnginePlan {
    EnginePlan::resolve(engine, &live.settings(), &store(live), media_dir())
}

/// An engine job of `kind` for the agent, with the tool snapshot.
pub fn job(live: &Live, kind: JobKind, engine: &EngineConfig) -> EngineJob {
    EngineJob {
        kind,
        agent: agent(),
        network: NetworkId::new(NETWORK),
        context: PromptContext { network_name: "Mind".into(), grounding_docs: vec!["Ethics".into()], ..PromptContext::default() },
        plan: plan(live, engine),
        tools: Some(tool_context()),
        roll: 0,
    }
}
