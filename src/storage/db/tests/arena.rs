use crate::domain::arena::debate::{Debate, DebateFormat, DebateSetup, Debater, Judge, Side};
use crate::domain::arena::lab::{Experiment, Scenario};
use crate::domain::arena::ArenaId;
use crate::domain::beliefs::{apply_proposals, apply_user_action, BeliefAction, BeliefBook, BeliefId, BeliefOrigin, BeliefSettings, BeliefSource, Credence, Proposal};
use crate::domain::ProfileId;
use crate::storage::db::Database;
use jiff::Timestamp;

fn debate(id: &str, minute: i8) -> Debate {
    let setup = DebateSetup {
        motion: "Ban cars".into(),
        source_post: None,
        debaters: vec![Debater { agent: ProfileId::new("a"), side: Side::For }, Debater { agent: ProfileId::new("b"), side: Side::Against }],
        format: DebateFormat::Oxford,
        rounds: 1,
        moderator: None,
        judge: Judge::Agent(ProfileId::new("j")),
    };
    let at = crate::time::utc(2026, 11, 18, 9, minute, 0);
    Debate::new(ArenaId::new(id), setup, at).with_turn("Opening.", at)
}

#[test]
fn debates_and_experiments_round_trip_newest_first() {
    let db = Database::in_memory().unwrap();
    db.save_arena_record(&debate("d1", 1)).unwrap();
    db.save_arena_record(&debate("d2", 5)).unwrap();
    db.save_arena_record(&debate("d1", 9)).unwrap();
    let loaded: Vec<Debate> = db.load_arena_records().unwrap();
    assert_eq!(loaded.iter().map(|d| d.id.as_str()).collect::<Vec<_>>(), ["d1", "d2"]);
    assert_eq!(loaded[0], debate("d1", 9));
    db.delete_arena_record::<Debate>(&ArenaId::new("d1")).unwrap();
    assert_eq!(db.load_arena_records::<Debate>().unwrap().len(), 1);
    let experiment = Experiment::new(ArenaId::new("x"), Scenario::default(), vec![ProfileId::new("a")], Some("trolley".into()), Timestamp::now());
    db.save_arena_record(&experiment).unwrap();
    assert_eq!(db.load_arena_records::<Experiment>().unwrap(), vec![experiment]);
}

#[test]
fn belief_changes_and_settings_persist() {
    let db = Database::in_memory().unwrap();
    let agent = ProfileId::new("ada");
    let source = BeliefSource::new(BeliefOrigin::Post, "post_1", "A post");
    let proposals = [Proposal { proposition: "Cities should ban private cars".into(), credence: Credence::from_percent(70), reason: "Air".into() }];
    let change = apply_proposals(&BeliefBook::default(), &agent, &proposals, &source, Timestamp::now(), || BeliefId::new("b1"));
    db.save_belief_change(&change).unwrap();
    let book = db.load_belief_book().unwrap();
    assert_eq!((book.beliefs.len(), book.revisions.len()), (1, 1));
    let forget = apply_user_action(&book, &BeliefAction::Forget(BeliefId::new("b1")), Timestamp::now());
    db.save_belief_change(&forget).unwrap();
    let after = db.load_belief_book().unwrap();
    assert!(after.beliefs.is_empty());
    assert_eq!(after.revisions.len(), 2, "the log keeps the forgotten belief's history");
    db.save_belief_settings(&agent, &BeliefSettings::default().switched(false)).unwrap();
    assert!(!db.load_belief_settings().unwrap()[&agent].updates_enabled);
}
