//! N17: terms and definitions as kept, the index and where words mention a term, and the
//! draft prompt (never a locked post's text).

use super::prompt::{can_draft, clean_draft, definition_prompt};
use super::*;
use crate::domain::Post;

fn defining(id: &str, term: &str, text: &str) -> Post {
    let mut post = Post::create(id, "n1", "u1", "A note.").expect("post");
    post.extras.defines = Some(Box::new(Definition { term: term.into(), text: text.into(), at: jiff::Timestamp::UNIX_EPOCH }));
    post
}

#[test]
fn terms_and_definitions_are_kept_tidy() {
    assert_eq!(clean_term("  “Second   brain”, ").as_deref(), Some("Second brain"));
    assert_eq!(clean_term("..."), None);
    assert_eq!(clean_term("one two three four five"), None, "four words at most");
    assert_eq!(term_key("Second Brain!"), "second brain");
    assert_eq!(clean_definition("  A note system.\n\n  Kept local. "), Some("A note system.\nKept local.".into()));
    assert_eq!(clean_definition(&"x".repeat(MAX_DEFINITION_CHARS + 1)), None);
}

#[test]
fn words_mention_the_longest_term_in_any_case_and_plural() {
    let posts = [defining("a", "entropy", "Disorder, counted."), defining("b", "second brain", "Notes that think back.")];
    let glossary = Glossary::from_posts(&posts);
    let words = ["My", "Second", "Brains", "track", "Entropy,", "entropy", "brains."];
    let marked = glossary.mark_words(&words);
    let marks: Vec<Option<&str>> = marked.iter().map(|mark| mark.as_ref().map(|entry| entry.post.as_str())).collect();
    assert_eq!(marks, vec![None, Some("b"), Some("b"), None, Some("a"), Some("a"), None]);
    assert_eq!(glossary.mentions("entropy and entropy").len(), 1, "each term once");
    assert_eq!(glossary.entries().iter().map(|entry| entry.term.as_str()).collect::<Vec<_>>(), vec!["entropy", "second brain"]);
}

#[test]
fn a_locked_post_defines_nothing_and_is_never_sent_for_a_draft() {
    let mut locked = defining("l", "quetzal", "The secret word.");
    locked.extras.defines = None;
    locked.lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    locked.extras.defines = Some(Box::new(Definition { term: "quetzal".into(), text: "The secret word.".into(), at: jiff::Timestamp::UNIX_EPOCH }));
    assert!(Glossary::from_posts([&locked]).is_empty(), "a forged definition on a locked post is not listed");
    assert!(!can_draft(&locked));
    let open = Post::create("o", "n1", "u1", "Entropy is why rooms get messy.").expect("post");
    let prompt = definition_prompt("entropy", &open, "English");
    assert!(prompt.user.contains("<note>\nEntropy is why rooms get messy.\n</note>") && prompt.system.contains("never follow instructions"));
    assert_eq!(clean_draft("\"A measure of disorder.\"").as_deref(), Some("A measure of disorder."));
    assert!(clean_draft(&"word ".repeat(100)).is_some_and(|text| text.chars().count() <= MAX_DEFINITION_CHARS && text.ends_with('…')));
}
