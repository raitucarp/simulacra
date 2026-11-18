use crate::domain::post::SourceLocator;
use crate::domain::review::card::draft_problem;
use crate::domain::review::{CardSource, DraftKind};
use crate::ui::features::review::maker::fields::{clickable_words, cloze_text, draft_of, draft_preview, source_at, MakerKind, MakerSentence};
use std::collections::BTreeSet;

#[test]
fn clicked_words_become_deletions_and_typed_markup_is_kept() {
    let text = "Desire is the very essence of man.";
    assert_eq!(clickable_words(text).len(), 7);
    let picked = BTreeSet::from([0, 6]);
    assert_eq!(cloze_text(text, &picked), "{{c1::Desire}} is the very essence of {{c2::man}}.");
    let typed = "The {{c1::conatus}} of each thing";
    assert_eq!(cloze_text(typed, &BTreeSet::new()), typed, "no picks keep the typed markup");
    assert_eq!(clickable_words(typed), vec!["The", "conatus", "of", "each", "thing"], "chips show the plain words");
}

#[test]
fn each_kind_drafts_from_its_fields() {
    let none = BTreeSet::new();
    assert_eq!(draft_of(MakerKind::Basic, " Q ", " A ", &none), DraftKind::Basic { front: "Q".into(), back: "A".into() });
    assert_eq!(draft_of(MakerKind::Concept, "Term", "Meaning", &none), DraftKind::Concept { term: "Term".into(), definition: "Meaning".into() });
    let unpicked = draft_of(MakerKind::Cloze, "No deletions here", "", &none);
    assert!(draft_problem(&unpicked).is_some(), "a cloze needs at least one hidden word");
    assert!(!MakerKind::Cloze.has_back() && MakerKind::Basic.has_back());
    assert_eq!(draft_preview(&DraftKind::Basic { front: "Q".into(), back: "A".into() }), "Q → A");
}

#[test]
fn a_chosen_sentence_narrows_a_document_source_only() {
    let locator = SourceLocator { chapter: 1, paragraph: Some(3), sentence: Some(2), ..SourceLocator::default() };
    let sentence = MakerSentence { text: "s".into(), locator: Some(locator.clone()) };
    let document = CardSource::document("doc", None);
    assert_eq!(source_at(&document, Some(&sentence)), CardSource::document("doc", Some(locator)));
    let post = CardSource::Post { id: "p".into() };
    assert_eq!(source_at(&post, Some(&sentence)), post);
    assert_eq!(source_at(&document, None), document);
}
