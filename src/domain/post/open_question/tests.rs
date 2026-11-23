//! N18: where a question stands, and how an agent reads it.

use super::*;

#[test]
fn a_question_is_open_until_answered_and_other_posts_are_no_question() {
    let mut post = Post::create("q", "n1", "u1", "Why is the sky blue?").expect("post");
    assert_eq!(question_state(&post), None);
    post.kind = PostKind::Question;
    assert!(is_open(&post));
    assert!(agent_note(&post).is_some_and(|note| note.contains("open question")));
    post.extras.answered = Some(Box::new(Answered { at: Timestamp::UNIX_EPOCH, answer: Some(PostId::new("a")) }));
    assert_eq!(question_state(&post), Some(QuestionState::Answered));
    let json = serde_json::to_string(&post.extras).expect("json");
    let back: crate::domain::post::PostExtras = serde_json::from_str(&json).expect("extras");
    assert_eq!(back.answered, post.extras.answered);
}
