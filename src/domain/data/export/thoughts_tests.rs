//! N16–N20: lens replies, definitions, questions, changes of mind and translations travel with
//! the post; never those of a locked post.

use super::*;
use crate::domain::post::glossary::Definition;
use crate::domain::post::lens::Lens;
use crate::domain::post::mind::MindChange;
use crate::domain::post::open_question::Answered;
use crate::domain::post::translation::Translation;
use crate::domain::post::PostKind;

fn marked(post: &mut Post, tag: &str) {
    let at = jiff::Timestamp::UNIX_EPOCH;
    post.kind = PostKind::Question;
    post.extras.lens = Some(Lens::History);
    post.extras.defines = Some(Box::new(Definition { term: format!("{tag} term"), text: format!("{tag} meaning"), at }));
    post.extras.mind_change = Some(Box::new(MindChange { revises: None, was: format!("{tag} old view"), at }));
    post.extras.translation = Some(Box::new(Translation { language: "es".into(), text: format!("{tag} traducido"), at }));
}

#[test]
fn thought_marks_travel_but_never_past_a_lock() {
    let mut open = Post::create("p1", "n1", "u1", "Open").expect("post");
    marked(&mut open, "Open");
    let block = thoughts::thought_block(&open);
    for expected in ["*Through the lens of history*", "> **Defines “Open term”:** Open meaning", "**Open question**", "> **Changed my mind.** Before: Open old view", "> **Translation (es):** Open traducido"] {
        assert!(block.contains(expected), "{expected} in {block}");
    }
    open.extras.answered = Some(Box::new(Answered { at: jiff::Timestamp::UNIX_EPOCH, answer: None }));
    assert!(thoughts::thought_block(&open).contains("**Answered question**"));
    let mut locked = Post::create("p2", "n1", "u1", "Tamarind").expect("post");
    locked.lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    marked(&mut locked, "Quetzal");
    assert_eq!(thoughts::thought_block(&locked), "", "a locked post travels as its seal only");
}
