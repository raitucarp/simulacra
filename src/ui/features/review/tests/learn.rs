use crate::domain::post::SourceLocator;
use crate::domain::review::{Anchor, AnchorSide};
use crate::domain::{NetworkId, Post, PostId, Profile, ProfileId};
use crate::ui::features::review::learn::request::{anchor_of, draft_of, suggestion_text, Granularity, LearnRequest, LearnSource, Passage};
use crate::ui::features::review::panel::matches_query;
use crate::ui::features::review::PostView;

fn view() -> PostView {
    let post = Post::create(PostId::new("p1"), NetworkId::new("n"), ProfileId::new("a"), "Desire is the essence of man.").unwrap();
    let author = Profile { name: "Spinoza Node".into(), handle: "@spinoza".into(), ..Profile::unknown().clone() };
    PostView { post, author }
}

fn document() -> LearnSource {
    let passage = |text: &str, sentence| Passage { text: text.into(), locator: Some(SourceLocator { chapter: 1, paragraph: Some(1), sentence, ..SourceLocator::default() }) };
    LearnSource::Document { id: "doc".into(), title: "Ethics".into(), sentences: vec![passage("God is.", Some(1)), passage("God acts.", Some(2))], paragraphs: vec![passage("God is. God acts.", None)] }
}

#[test]
fn a_post_is_its_own_anchor_and_a_document_needs_a_passage() {
    let post = LearnSource::Post(view());
    assert_eq!(anchor_of(&post, None), Some(Anchor::post(&PostId::new("p1"))));
    let doc = document();
    assert_eq!(anchor_of(&doc, None), None);
    let request = LearnRequest::new(doc.clone(), AnchorSide::Back);
    assert_eq!((request.passages(Granularity::Sentence).len(), request.passages(Granularity::Paragraph).len()), (2, 1));
    let chosen = &request.passages(Granularity::Sentence)[1];
    let Some(Anchor::Passage { document_id, locator, text }) = anchor_of(&doc, Some(chosen)) else { panic!("a passage anchor") };
    assert_eq!((document_id.as_str(), text.as_str(), locator.and_then(|l| l.sentence)), ("doc", "God acts.", Some(2)));
}

#[test]
fn the_fields_make_a_draft_on_the_chosen_side() {
    let draft = draft_of(&LearnSource::Post(view()), None, AnchorSide::Front, "  Desire  ", vec!["ethics".into()]).unwrap();
    assert_eq!((draft.side, draft.written.as_str(), draft.tags.len()), (AnchorSide::Front, "Desire", 1));
    assert!(draft_of(&document(), None, AnchorSide::Back, "Q?", vec![]).is_none());
    assert_eq!(suggestion_text(&LearnSource::Post(view()), None).as_deref(), Some("Desire is the essence of man."));
    assert_eq!(suggestion_text(&document(), None), None);
}

#[test]
fn the_picker_matches_every_word_in_text_or_author() {
    let view = view();
    assert!(matches_query(&view, "essence SPINOZA"));
    assert!(matches_query(&view, ""));
    assert!(!matches_query(&view, "essence kant"));
}
