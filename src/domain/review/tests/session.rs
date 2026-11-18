use crate::domain::review::card::CardId;
use crate::domain::review::session::{Session, Stage, MAX_REQUEUES, REQUEUE_GAP};
use crate::domain::review::Grade;

fn session(n: usize) -> Session {
    Session::new((0..n).map(|i| CardId::new(format!("c{i}"))).collect())
}

#[test]
fn reveal_then_grade_moves_to_the_next_card() {
    let s = session(3).reveal();
    assert_eq!(s.stage, Stage::Revealed);
    let s = s.grade(Grade::Good);
    assert_eq!((s.stage, s.current().map(CardId::as_str)), (Stage::Prompt, Some("c1")));
    let s = s.grade(Grade::Easy).grade(Grade::Hard);
    assert!(s.is_finished());
    assert_eq!((s.done(), s.planned, s.grade_counts()), (3, 3, [0, 1, 1, 1]));
    assert_eq!(s.first_try_recall(), Some(1.0));
}

#[test]
fn a_forgotten_card_returns_after_a_gap_a_limited_number_of_times() {
    let s = session(6).grade(Grade::Again);
    assert_eq!(s.queue[REQUEUE_GAP].as_str(), "c0", "back after {REQUEUE_GAP} cards");
    let mut s = Session::new(vec![CardId::new("only")]);
    for _ in 0..MAX_REQUEUES {
        s = s.grade(Grade::Again);
        assert_eq!(s.current().map(CardId::as_str), Some("only"));
    }
    let s = s.grade(Grade::Again);
    assert!(s.is_finished(), "gives up after {MAX_REQUEUES} requeues");
    assert_eq!((s.done(), s.answers.len()), (1, MAX_REQUEUES + 1));
    assert_eq!(s.first_try_recall(), Some(0.0));
}

#[test]
fn grading_an_empty_session_changes_nothing() {
    let s = Session::default();
    assert_eq!(s.clone().grade(Grade::Good), s);
    assert_eq!(s.first_try_recall(), None);
}
