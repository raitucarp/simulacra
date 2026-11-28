//! Unit tests of the post controls' pure logic (permalinks, share text, references).

use super::{collect_references, post_permalink, share_text, ReferenceKind};
use crate::domain::{Post, PostId, Profile};

fn post(id: &str, content: &str) -> Post {
    let author = Profile::new_primary("u1", "Ada Lovelace", "@ada");
    Post::create(id, "net_agora", author, content).expect("valid post")
}

#[test]
fn permalink_names_network_and_post() {
    assert_eq!(post_permalink(&post("post_1", "Hello")), "simulacra://net_agora/post/post_1");
}

#[test]
fn share_text_carries_thought_author_and_permalink() {
    let author = Profile::new_primary("u1", "Ada Lovelace", "@ada");
    let shared = share_text(&post("post_9", "  Know thyself.  "), &author);
    assert_eq!(shared, "Know thyself.\n\n— Ada Lovelace @ada\nsimulacra://net_agora/post/post_9");
}

#[test]
fn references_list_quote_first_then_links_without_duplicates() {
    let quoted = post("post_q", "Quoted thought about virtue ethics");
    let linked = post("post_l", "Linked idea on monism");
    let mut root = post("post_r", "See [[post_l]] and [[post_q]] and [[post_r]]");
    root.quote_id = Some(PostId::new("post_q"));
    let all = vec![quoted, linked, root.clone()];

    let refs = collect_references(&root, &all);

    let ids: Vec<(&str, ReferenceKind)> = refs.iter().map(|r| (r.id.as_str(), r.kind)).collect();
    assert_eq!(ids, vec![("post_q", ReferenceKind::Quote), ("post_l", ReferenceKind::Link)]);
    assert_eq!(refs[1].excerpt, "Linked idea on...");
}

#[test]
fn a_thought_without_quote_or_links_has_no_references() {
    let lonely = post("post_x", "Standalone musing");
    let all = [lonely.clone()];
    assert!(collect_references(&lonely, &all[..]).is_empty());
}

#[test]
fn copy_embed_pins_the_version_the_post_shows() {
    let mut post = crate::domain::Post::create("post_1", "n1", "u1", "An idea worth keeping.").expect("valid");
    assert_eq!(super::menu::embed_code(&post), "((post_1@1.0.0))");
    post.extras.head = Some(crate::domain::data::history::VersionHead { version: crate::domain::data::history::SemVer::new(2, 1, 0), ..Default::default() });
    assert_eq!(super::menu::embed_code(&post), "((post_1@2.1.0))");
    for key in ["post.menu_copy_embed", "post.copied_embed"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}

#[test]
fn references_list_embeds_then_every_thought_referring_back_without_leaking_locked_text() {
    use crate::domain::post::refs::RefKind;
    use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};
    let mut state = AppState::new_in_memory();
    let target = state.submit_post("A thought others build on.").expect("target");
    let embeds = state.submit_post(&format!("Building on (({target}))")).expect("embeds");
    let (password, fast) = (crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 });
    let anchor = ThreadAnchor { quote_of: Some(target.clone()), ..ThreadAnchor::default() };
    let locked = state.submit_locked_thread(&[ThreadDraft::text("Quetzal orchard secret")], &anchor, &password, fast).expect("locked")[0].clone();
    let refs = collect_references(state.get_post(&target).expect("target"), &state);
    let kinds: Vec<(&PostId, ReferenceKind)> = refs.iter().map(|r| (&r.id, r.kind)).collect();
    assert_eq!(kinds, vec![(&locked, ReferenceKind::ReferredBy(RefKind::Quote)), (&embeds, ReferenceKind::ReferredBy(RefKind::Embed))]);
    assert!(refs.iter().all(|r| !r.excerpt.contains("Quetzal")), "a locked thought shows its placeholder only");
    let back = collect_references(state.get_post(&embeds).expect("embeds"), &state);
    assert_eq!(back.iter().map(|r| r.kind).collect::<Vec<_>>(), vec![ReferenceKind::Embed]);
    for key in ["post.refs_embed", "post.refs_backlinks_one", "post.refs_by.embed", "post.refs_by.quote", "post.refs_by.link", "feed.backlinks.title_other"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}

#[test]
fn the_references_menu_sections_what_it_refers_to_and_what_refers_back() {
    use super::{refs_menu_spec, OpenPostAction, PostReference};
    let refs = vec![
        PostReference { id: PostId::new("q"), kind: ReferenceKind::Quote, excerpt: "Quoted".into() },
        PostReference { id: PostId::new("b"), kind: ReferenceKind::ReferredBy(crate::domain::post::refs::RefKind::Link), excerpt: "Back".into() },
    ];
    let on_open: OpenPostAction = std::rc::Rc::new(|_, _, _| {});
    let spec = refs_menu_spec(&refs, 2, &PostId::new("p"), &on_open);
    assert_eq!(crate::ui::menus::rules::violations(&spec), Vec::<String>::new());
    assert!(spec.scrollable, "a long list scrolls instead of running off the window");
    assert_eq!(spec.sections.len(), 3, "outgoing, incoming, the conversation");
    assert!(spec.sections[..2].iter().all(|section| section.label.is_some()));
    assert!(spec.reaches("refs.outgoing") && spec.reaches("refs.incoming") && spec.reaches("refs.thread"));
    let lonely = refs_menu_spec(&[], 0, &PostId::new("p"), &on_open);
    assert!(lonely.item("refs.none").is_some_and(|item| item.disabled.is_some()), "says there is nothing, greyed out");
}
