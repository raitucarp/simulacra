use crate::domain::learning::goals::edit::{new_goal, with_link, with_tasks, GoalDraft};
use crate::domain::learning::palace::edit::{new_palace, placed};
use crate::domain::learning::tutor::machine::SessionStart;
use crate::domain::learning::{GoalId, GoalLink, PalaceId, PlacedTarget, RoomKind, SessionId, TutorSession, TutorSource};
use crate::domain::review::ItemId;
use crate::domain::NetworkId;
use crate::storage::db::Database;
use chrono::{NaiveDate, Utc};

fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 11, 18).unwrap()
}

#[test]
fn goals_sessions_and_palaces_round_trip_and_upsert() {
    let db = Database::in_memory().unwrap();
    let network = NetworkId::new("net");
    let draft = GoalDraft { title: "Spinoza".into(), why: "Reading group".into(), target: today() + chrono::Duration::days(30) };
    let goal = new_goal(&draft, network.clone(), GoalId("goal_1".into()), today(), Utc::now()).unwrap();
    let goal = with_tasks(with_link(goal, GoalLink::concept("Necessity")), &["Write a post".into()], today());
    let start = SessionStart { network_id: network.clone(), source: TutorSource::Post { id: "p1".into() }, title: "Freedom".into(), material: "text".into() };
    let session = TutorSession::start(SessionId("tutor_1".into()), start, Utc::now()).with_opening("What is it?", Utc::now());
    let palace = new_palace(PalaceId("palace_1".into()), network, "Home", (RoomKind::Garden, "Garden"), Utc::now());
    let room = palace.rooms[0].id.clone();
    let palace = placed(palace, &room, 1, PlacedTarget::Item { id: ItemId::new("i1") });
    db.save_goal(&goal).unwrap();
    db.save_tutor_session(&session).unwrap();
    db.save_palace(&palace).unwrap();
    let loaded = db.load_learning().unwrap();
    assert_eq!(loaded.goals, vec![goal.clone()]);
    assert_eq!(loaded.sessions, vec![session.clone()]);
    assert_eq!(loaded.palaces, vec![palace.clone()]);
    let renamed = crate::domain::learning::Goal { title: "Ethics".into(), ..goal };
    db.save_goal(&renamed).unwrap();
    assert_eq!(db.load_learning().unwrap().goals, vec![renamed], "saving again replaces the row");
    db.delete_goal("goal_1").unwrap();
    db.delete_tutor_session("tutor_1").unwrap();
    db.delete_palace("palace_1").unwrap();
    assert_eq!(db.load_learning().unwrap(), Default::default());
}

#[test]
fn the_learning_tables_are_created_idempotently() {
    let db = Database::in_memory().unwrap();
    db.conn.execute_batch(super::super::learning_schema::CREATE_LEARNING).expect("running the schema again is harmless");
    db.conn.execute("INSERT INTO learning_goals (id, network_id, updated_at, record_json) VALUES ('bad', 'net', 'x', 'not json')", []).unwrap();
    assert!(db.load_learning().unwrap().goals.is_empty(), "an unreadable row is skipped");
}
