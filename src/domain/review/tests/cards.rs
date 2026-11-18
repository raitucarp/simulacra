use super::day;
use crate::domain::review::card::cloze::{cloze_from_words, cloze_ordinals, cloze_words, deleted_answer, parse_cloze, plain_text, ClozePart};
use crate::domain::review::card::{
    card_faces, draft_problem, expand_draft, parse_tags, CardDraft, CardId, CardKind, CardSource, ConceptDirection, DraftKind,
    DraftProblem, FacePart, NewCardContext,
};
use crate::domain::NetworkId;
use chrono::Utc;
use std::collections::BTreeSet;

const SPINOZA: &str = "{{c1::Spinoza}} wrote the {{c2::Ethics::a book}} in {{c1::Latin}}.";

fn context() -> NewCardContext {
    NewCardContext { network_id: NetworkId::new("net"), created_at: Utc::now(), today: day(0) }
}

fn ids() -> impl FnMut() -> CardId {
    let mut n = 0;
    move || {
        n += 1;
        CardId::new(format!("c{n}"))
    }
}

#[test]
fn cloze_markup_parses_with_hints_and_keeps_bad_markup_plain() {
    let parts = parse_cloze(SPINOZA);
    assert_eq!(parts[0], ClozePart::Deletion { ordinal: 1, answer: "Spinoza".into(), hint: None });
    assert_eq!(parts[2], ClozePart::Deletion { ordinal: 2, answer: "Ethics".into(), hint: Some("a book".into()) });
    assert_eq!(cloze_ordinals(SPINOZA), vec![1, 2]);
    assert_eq!(plain_text(SPINOZA), "Spinoza wrote the Ethics in Latin.");
    assert_eq!(deleted_answer(SPINOZA, 1), "Spinoza … Latin");
    assert_eq!(plain_text("{{cx::nope}} and {{c1::}} stay"), "{{cx::nope}} and {{c1::}} stay");
    assert!(cloze_ordinals("no deletions {{c1::unclosed").is_empty());
}

#[test]
fn a_cloze_card_blanks_its_own_deletions_only() {
    let faces = card_faces(&CardKind::Cloze { text: SPINOZA.into(), ordinal: 2 });
    assert!(faces.prompt.contains(&FacePart::Blank(Some("a book".into()))));
    assert!(faces.prompt.contains(&FacePart::Text("Spinoza".into())), "other deletions are shown");
    assert!(faces.answer.contains(&FacePart::Answer("Ethics".into())));
    assert_eq!(faces.prompt_text(), "Spinoza wrote the … in Latin.");
}

#[test]
fn picked_words_become_numbered_deletions_without_their_punctuation() {
    let words = cloze_words("Memory decays like a power law.");
    let text = cloze_from_words(&words, &BTreeSet::from([0, 5]));
    assert_eq!(text, "{{c1::Memory}} decays like a power {{c2::law}}.");
    assert_eq!(cloze_ordinals(&text), vec![1, 2]);
}

#[test]
fn drafts_expand_to_one_card_per_deletion_and_two_per_concept() {
    let source = CardSource::document("doc_1", None);
    let cloze = CardDraft { kind: DraftKind::Cloze { text: SPINOZA.into() }, source: source.clone(), tags: vec!["philosophy".into()] };
    let cards = expand_draft(&cloze, &context(), ids());
    assert_eq!(cards.len(), 2);
    assert!(cards.iter().all(|c| c.source == source && c.tags == ["philosophy"] && c.schedule.is_new()));
    let concept = CardDraft { kind: DraftKind::Concept { term: "Conatus".into(), definition: "The striving to persist".into() }, source, tags: vec![] };
    let directions: Vec<ConceptDirection> = expand_draft(&concept, &context(), ids())
        .into_iter()
        .filter_map(|c| match c.kind {
            CardKind::Concept { direction, .. } => Some(direction),
            _ => None,
        })
        .collect();
    assert_eq!(directions, ConceptDirection::ALL.to_vec());
}

#[test]
fn incomplete_drafts_report_their_problem_and_make_no_cards() {
    assert_eq!(draft_problem(&DraftKind::Basic { front: " ".into(), back: "b".into() }), Some(DraftProblem::MissingPrompt));
    assert_eq!(draft_problem(&DraftKind::Concept { term: "t".into(), definition: "".into() }), Some(DraftProblem::MissingAnswer));
    assert_eq!(draft_problem(&DraftKind::Cloze { text: "plain".into() }), Some(DraftProblem::NoDeletion));
    let draft = CardDraft { kind: DraftKind::Cloze { text: "plain".into() }, source: CardSource::Sentence { text: "plain".into() }, tags: vec![] };
    assert!(expand_draft(&draft, &context(), ids()).is_empty());
}

#[test]
fn concept_answers_follow_the_direction_and_tags_are_cleaned() {
    let forward = CardKind::Concept { term: "Akrasia".into(), definition: "Weakness of will".into(), direction: ConceptDirection::TermToDefinition };
    let backward = CardKind::Concept { term: "Akrasia".into(), definition: "Weakness of will".into(), direction: ConceptDirection::DefinitionToTerm };
    assert_eq!((forward.answer_text(), backward.answer_text()), ("Weakness of will".into(), "Akrasia".into()));
    assert_eq!(parse_tags("#ethics, Ethics, ,greek "), vec!["ethics".to_string(), "greek".to_string()]);
}
