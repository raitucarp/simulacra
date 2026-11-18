use super::{basic, day, no_posts};
use crate::domain::post::SourceLocator;
use crate::domain::review::item::{
    draft_problem, edited_item, new_item, parse_tags, retire_orphans, retired_now, DraftProblem, Face, ItemDraft, NewItemContext,
};
use crate::domain::review::{Anchor, AnchorSide, Deck, Grade, ItemId, ReviewItem, Subject};
use crate::domain::{NetworkId, PostId};
use chrono::Utc;

fn context() -> NewItemContext {
    NewItemContext { network_id: NetworkId::new("net"), created_at: Utc::now(), today: day(0) }
}

fn on_post(side: AnchorSide, written: &str) -> ItemDraft {
    ItemDraft { anchor: Anchor::post(&PostId::new("p1")), side, written: written.into(), tags: vec!["ethics".into()] }
}

fn post_text(id: &str) -> Option<String> {
    (id == "p1").then(|| "Desire is the very essence of man.".to_string())
}

#[test]
fn add_question_makes_the_post_the_answer_and_add_answer_makes_it_the_prompt() {
    let question = new_item(&on_post(AnchorSide::Back, " What is the essence of man? "), &context(), ItemId::new("q")).unwrap();
    assert_eq!(question.faces().prompt, Face::Written("What is the essence of man?".into()));
    assert_eq!(question.faces().answer, Face::Source(Anchor::post(&PostId::new("p1"))));
    assert_eq!(question.answer_text(&post_text), "Desire is the very essence of man.");
    assert_eq!(question.subject.key(), "question");
    let answer = new_item(&on_post(AnchorSide::Front, "Desire"), &context(), ItemId::new("a")).unwrap();
    assert_eq!(answer.prompt_text(&post_text), "Desire is the very essence of man.");
    assert_eq!(answer.answer_text(&post_text), "Desire");
    assert!(answer.schedule.is_new() && answer.is_on_post("p1") && answer.subject.key() == "answer");
}

#[test]
fn an_edited_post_is_reviewed_as_edited() {
    let item = new_item(&on_post(AnchorSide::Back, "Q?"), &context(), ItemId::new("q")).unwrap();
    let edited = |id: &str| (id == "p1").then(|| "Desire, once edited.".to_string());
    assert_eq!(item.answer_text(&edited), "Desire, once edited.");
    assert_eq!(item.answer_text(&no_posts), "", "a deleted post has no text");
}

#[test]
fn drafts_report_the_missing_face() {
    assert_eq!(draft_problem(&on_post(AnchorSide::Back, "  ")), Some(DraftProblem::MissingQuestion));
    assert_eq!(draft_problem(&on_post(AnchorSide::Front, "")), Some(DraftProblem::MissingAnswer));
    let empty_passage = ItemDraft { anchor: Anchor::passage("doc", None, " "), ..on_post(AnchorSide::Back, "Q?") };
    assert_eq!(draft_problem(&empty_passage), Some(DraftProblem::MissingPassage));
    assert!(new_item(&on_post(AnchorSide::Back, ""), &context(), ItemId::new("x")).is_none());
}

#[test]
fn a_passage_keeps_its_place_and_quoted_text() {
    let locator = SourceLocator { chapter: 3, paragraph: Some(2), sentence: Some(1), ..SourceLocator::default() };
    let draft = ItemDraft { anchor: Anchor::passage("doc_ethics", Some(locator.clone()), "God is substance."), side: AnchorSide::Back, written: "What is God?".into(), tags: vec![] };
    let item = new_item(&draft, &context(), ItemId::new("p")).unwrap();
    assert_eq!(item.answer_text(&no_posts), "God is substance.");
    assert!(matches!(item.subject.anchor(), Some(Anchor::Passage { locator: Some(l), .. }) if *l == locator));
    assert_eq!(item.group_key(), "document:doc_ethics");
}

#[test]
fn editing_keeps_the_schedule_and_can_switch_sides() {
    let mut item = new_item(&on_post(AnchorSide::Back, "Q?"), &context(), ItemId::new("q")).unwrap();
    item.schedule.reps = 4;
    let edited = edited_item(&item, &ItemDraft { tags: vec![], ..on_post(AnchorSide::Front, "Desire") }).unwrap();
    assert_eq!((edited.schedule.reps, edited.id.as_str(), edited.subject.key()), (4, "q", "answer"));
    assert!(edited_item(&item, &on_post(AnchorSide::Front, " ")).is_none());
}

#[test]
fn deleted_posts_retire_their_items_and_keep_the_log() {
    let item = new_item(&on_post(AnchorSide::Back, "Q?"), &context(), ItemId::new("q")).unwrap();
    let passage = basic("b", "q", "a");
    let gone = |_: &str| false;
    assert!(retired_now(&item, &gone) && !retired_now(&passage, &gone));
    let retired = retire_orphans(&[item.clone(), passage], &gone);
    assert_eq!(retired.len(), 1);
    assert!(retired[0].retired && !retired[0].is_active());
    assert!(retire_orphans(&retired, &gone).is_empty(), "retiring happens once");
    let answer = crate::domain::review::Answer { item_id: item.id.clone(), grade: Grade::Good, mode: crate::domain::review::ReviewMode::Review, confidence: None, correct: None };
    let (_, entry) = crate::domain::review::apply_answer(&item, &answer, Utc::now(), day(0), &crate::domain::review::Policy::with_retention_percent(90));
    let deck = Deck { items: retired, log: vec![entry] };
    assert!(deck.on_post(&NetworkId::new("net"), "p1").is_none(), "a retired item no longer marks its post");
    assert_eq!(deck.without_item(&item.id).log.len(), 1);
}

#[test]
fn the_deck_finds_the_item_on_a_post_and_tags_are_cleaned() {
    let item = new_item(&on_post(AnchorSide::Back, "Q?"), &context(), ItemId::new("q")).unwrap();
    let deck = Deck::default().with_item(item.clone()).with_item(ReviewItem { tags: vec![], ..item });
    assert_eq!(deck.items.len(), 1, "saving an item again replaces it");
    assert!(deck.on_post(&NetworkId::new("net"), "p1").is_some());
    assert!(deck.on_post(&NetworkId::new("other"), "p1").is_none());
    assert!(matches!(deck.items[0].subject, Subject::Anchored { .. }));
    assert_eq!(parse_tags("#ethics, Ethics, ,greek "), vec!["ethics".to_string(), "greek".to_string()]);
}

#[test]
fn review_modes_keep_their_wire_keys() {
    for mode in crate::domain::review::ReviewMode::ALL {
        assert_eq!(serde_json::to_string(&mode).unwrap(), format!("\"{}\"", mode.key()), "the log stores the key");
    }
}
