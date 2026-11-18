mod candidates;
mod classify;
mod lexical;

use super::*;

/// A post side by "Ada".
pub(super) fn post(id: &str, text: &str) -> TensionSide {
    TensionSide::post(id, "Ada", text)
}

/// An item to rank.
pub(super) fn item(id: &str, text: &str, vector: Option<Vec<f32>>) -> CandidateInput {
    CandidateInput { side: post(id, text), vector }
}

/// A candidate between two made-up posts, for the classifier.
pub(super) fn candidate(number: usize) -> Candidate {
    Candidate {
        left: post(&format!("a{number}"), &format!("Claim {number} holds.")),
        right: post(&format!("b{number}"), &format!("Claim {number} does not hold.")),
        similarity: None,
        lexical: LexicalSignals::default(),
        score: 0.0,
    }
}

/// Floats equal to rounding.
pub(super) fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

#[test]
fn side_keys_name_the_kind_id_and_chunk() {
    assert_eq!(post("p1", "x").key(), "post:p1");
    assert_eq!(TensionSide::passage("d1", 3, "Ethics", "x").key(), "passage:d1#3");
}

#[test]
fn statuses_round_trip_and_default_to_open() {
    assert_eq!(TensionStatus::default(), TensionStatus::Open);
    assert_eq!(TensionStatus::from_key("dismissed"), Some(TensionStatus::Dismissed));
    assert_eq!(SideKind::from_key(SideKind::Passage.key()), Some(SideKind::Passage));
}

#[test]
fn pair_keys_are_symmetric_stable_and_follow_the_texts() {
    let (a, b) = (post("a", "Habits shape us."), post("b", "Habits do not shape us."));
    assert_eq!(pair_key(&a, &b), pair_key(&b, &a));
    let expected = crate::domain::embedding::content_hash(&["post:a", "Habits shape us.", "post:b", "Habits do not shape us."]);
    assert_eq!(pair_key(&a, &b), expected);
    let edited = post("b", "Habits barely shape us.");
    assert_ne!(pair_key(&a, &b), pair_key(&a, &edited));
    assert_ne!(pair_key(&a, &b), pair_key(&a, &post("c", "Habits do not shape us.")));
}
