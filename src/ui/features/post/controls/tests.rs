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
    let shared = share_text(&post("post_9", "  Know thyself.  "));
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
