use super::*;
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::post::tokenize_post_body;

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

fn post(id: &str, text: &str) -> Post {
    Post::create(id, "n1", "u1", text).expect("valid")
}

fn refs_of(post: &Post) -> Vec<PostRef> {
    outgoing_refs(post, &tokenize_post_body(&post.content))
}

#[test]
fn embeds_come_first_then_the_quote_then_wikilinks_never_the_post_itself() {
    let mut quoting = post("a", "see [[c]] and ((b@1.2.0)) and ((a)) and [[a]]");
    quoting.quote_id = Some(PostId::new("d"));
    let refs = refs_of(&quoting);
    let kinds: Vec<(&str, RefKind)> = refs.iter().map(|r| (r.target.as_str(), r.kind)).collect();
    assert_eq!(kinds, vec![("b", RefKind::Embed), ("d", RefKind::Quote), ("c", RefKind::Link)]);
    assert_eq!(refs[0].version, Some(SemVer::new(1, 2, 0)));
}

#[test]
fn a_backlink_is_listed_once_with_its_strongest_reference() {
    let both = post("x", "((t)) again [[t]]");
    let link = post("y", "only [[t]]");
    let other = post("z", "nothing here");
    let (a, b, c) = (refs_of(&both), refs_of(&link), refs_of(&other));
    let target = PostId::new("t");
    let found = backlinks(&target, [(&both.id, a.as_slice()), (&link.id, b.as_slice()), (&other.id, c.as_slice())]);
    assert_eq!(found, vec![Backlink { from: PostId::new("x"), kind: RefKind::Embed }, Backlink { from: PostId::new("y"), kind: RefKind::Link }]);
}

#[test]
fn a_locked_post_refers_only_through_its_quote() {
    let mut locked = post("l", "secret ((t)) and [[u]]");
    locked.quote_id = Some(PostId::new("q"));
    locked.lock_with(&LockPassword::new("correct horse battery"), FAST).expect("locked");
    let refs = refs_of(&locked);
    assert_eq!(refs.iter().map(|r| (r.target.as_str(), r.kind)).collect::<Vec<_>>(), vec![("q", RefKind::Quote)]);
    assert!(!locked.linked_posts.contains(&PostId::new("t")), "the sealed embed is not kept as a link");
}

#[test]
fn embeds_are_linked_posts_after_the_wikilinks() {
    let written = post("p", "((b)) then [[c]] then ((c))");
    assert_eq!(written.linked_posts, vec![PostId::new("c"), PostId::new("b")]);
}
