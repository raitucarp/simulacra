//! N1: an embed lays out as its own item and shows the pinned version, the placeholder of a
//! locked post, or that the post is gone.

use super::super::embed::{embed_card, embed_view, EmbedView};
use super::super::layout::{layout, Item};
use super::super::BodyLinks;
use crate::domain::data::history::SemVer;
use crate::domain::post::lock::LOCKED_PLACEHOLDER;
use crate::domain::post::tokenize_post_body;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};
use crate::theme::ThemeTokens;

#[test]
fn an_embed_is_an_item_of_its_own() {
    let tokens = tokenize_post_body("see ((p1@1.0.0))");
    assert!(layout(&tokens).contains(&Item::Embed { target: "p1", version: Some(SemVer::ORIGINAL), trailing: "" }));
}

#[test]
fn an_embed_shows_the_version_it_is_pinned_to() {
    let mut state = AppState::new_in_memory();
    let target = state.submit_post("Version one of a thought.").expect("target");
    state.edit_post(&target, "Version two of a thought, rewritten from the start with more words.").expect("edited");
    let now = state.get_post(&target).and_then(|post| post.extras.head.clone()).expect("head").version;
    let EmbedView::Shown { pinned, now: shown, text, kept, .. } = embed_view(target.as_str(), Some(SemVer::ORIGINAL), &state) else { panic!("shown") };
    assert_eq!((pinned, shown, &*text, kept), (SemVer::ORIGINAL, now, "Version one of a thought.", true));
    let EmbedView::Shown { text, .. } = embed_view(target.as_str(), None, &state) else { panic!("shown") };
    assert!(text.starts_with("Version two"));
    let EmbedView::Shown { kept, text, .. } = embed_view(target.as_str(), Some(SemVer::new(9, 0, 0)), &state) else { panic!("shown") };
    assert!(!kept && text.starts_with("Version two"), "a version not kept falls back to the text shown");
    assert_eq!(embed_view("post_gone", None, &state), EmbedView::Missing);
}

#[test]
fn an_embedded_locked_post_shows_only_its_placeholder() {
    let (password, fast) = (crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 });
    let mut state = AppState::new_in_memory();
    let ids = state.submit_locked_thread(&[ThreadDraft::text("Quetzal orchard secret")], &ThreadAnchor::default(), &password, fast).expect("locked");
    assert!(matches!(embed_view(ids[0].as_str(), None, &state), EmbedView::Locked { .. }));
    assert_eq!(state.version_text(ids[0].as_str(), None).as_deref(), Some(LOCKED_PLACEHOLDER));
    let _card = embed_card(0, ids[0].as_str(), None, &state, &ThemeTokens::default(), &BodyLinks::default());
    for key in ["post.embed.unavailable", "post.embed.now", "post.embed.not_kept"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}
