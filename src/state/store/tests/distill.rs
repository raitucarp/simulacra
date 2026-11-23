//! N15: a thread distilled into the vault: every post in reading order with a link back to it,
//! the root remembering the document (and listed among its backlinks); a locked post and one
//! kept from some readers never reach the vault.

use crate::domain::crypto::KdfParams;
use crate::domain::document::links::backlinks;
use crate::domain::post::audience::Audience;
use crate::domain::post::distill::{post_link, DistillWords};
use crate::domain::post::lock::LockPassword;
use crate::state::store::AppState;

fn words(count: usize) -> DistillWords {
    DistillWords { origin: format!("From {count} posts."), summary_heading: "Summary".into(), open_post: "Open".into(), retracted: "Retracted".into(), corrected: "Corrected".into(), untitled: "A thread".into(), locked: "(locked)".into(), restricted: "(not shared)".into() }
}

#[test]
fn a_thread_becomes_a_vault_document_with_links_back_and_nothing_sealed() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("Why do bridges hum?\nA thread.").expect("root");
    let reply = state.submit_reply(&root, "Vortex shedding at the cables.", None, None).expect("reply");
    let sealed = state.submit_reply(&reply, "the Quetzalcoatl confession", None, None).expect("sealed");
    let scoped = state.submit_reply(&root, "My private hunch.", None, None).expect("scoped");
    let at = state.post_position(&sealed).expect("stored");
    state.posts[at].lock_with(&LockPassword::new("seven tidy lanterns"), KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locks");
    state.set_audience(&scoped, Some(Audience::OnlyMe)).expect("scoped");
    let order: Vec<_> = state.thread_of(&sealed).into_iter().map(|post| post.id.clone()).collect();
    assert_eq!(order, vec![root.clone(), reply.clone(), sealed.clone(), scoped.clone()], "root first, each reply under its parent");
    let distilled = state.distill_thread(&reply, words, None, |_| "2 Nov".into()).expect("distilled");
    assert_eq!(distilled.title, "Why do bridges hum?");
    let doc = state.documents.iter().find(|doc| doc.id == distilled.doc_id).expect("in the vault").clone();
    assert!(doc.content.starts_with("# Why do bridges hum?\n\n*From 4 posts.*"));
    assert!(doc.content.contains("Vortex shedding at the cables."));
    for id in [&root, &reply, &sealed, &scoped] {
        assert!(doc.content.contains(&post_link(state.active_network_id.as_str(), id.as_str())), "a link back to {id}");
    }
    assert!(!doc.content.contains("Quetzalcoatl") && doc.content.contains("(locked)"), "a locked post stays sealed");
    assert!(!doc.content.contains("private hunch") && doc.content.contains("(not shared)"), "N8: the vault is read by every agent");
    assert_eq!(state.get_post(&root).and_then(|p| p.extras.distilled.as_deref().map(|d| d.doc_id.clone())), Some(doc.id.clone()));
    assert!(backlinks(&doc, &state.posts).iter().any(|post| post.id == root), "the root is among the document's backlinks");
}
