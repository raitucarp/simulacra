use crate::domain::journal::capsule::{seal, SealUntil};
use crate::domain::journal::focus::{FocusLink, FocusSession, Outcome};
use crate::domain::journal::mood::{Affect, CheckinDraft};
use crate::domain::journal::{JournalEntry, MoodCheckin, RecordId, TimeCapsule};
use crate::storage::db::Database;
use chrono::{NaiveDate, TimeZone, Utc};

const SECRET: &str = "a private worry nobody should read";

fn day() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 3, 2).unwrap()
}

fn book_records() -> (JournalEntry, MoodCheckin, FocusSession, TimeCapsule) {
    let now = Utc.with_ymd_and_hms(2026, 3, 2, 9, 0, 0).unwrap();
    let entry = JournalEntry::new(RecordId::new("e1"), day(), now, format!("## Evening\n{SECRET} [[post_1]]"));
    let draft = CheckinDraft { affect: Some(Affect::new(-0.4, 0.6)), energy: 2, note: SECRET.into(), tags: vec!["work".into()], ..CheckinDraft::default() };
    let checkin = draft.into_checkin(RecordId::new("m1"), now.fixed_offset(), day()).unwrap();
    let session = FocusSession {
        id: RecordId::new("f1"), day: day(), started_at: now, ended_at: now, intention: SECRET.into(), planned_minutes: 25,
        focused_minutes: 24, link: Some(FocusLink::Post("post_1".into())), distractions: 2, outcome: Some(Outcome::Partly), reflection: "ok".into(),
    };
    let capsule = seal(RecordId::new("c1"), SECRET, vec![], SealUntil::Days(30), now, day()).unwrap();
    (entry, checkin, session, capsule)
}

fn saved_db() -> Database {
    let db = Database::in_memory().unwrap();
    let (entry, checkin, session, capsule) = book_records();
    db.save_journal_record(&entry).unwrap();
    db.save_journal_record(&checkin).unwrap();
    db.save_journal_record(&session).unwrap();
    db.save_journal_record(&capsule).unwrap();
    db
}

#[test]
fn every_kind_round_trips_and_upserts() {
    let db = saved_db();
    let (entry, checkin, session, capsule) = book_records();
    let book = db.load_journal_book().unwrap();
    assert_eq!((book.entries, book.checkins, book.sessions, book.capsules), (vec![entry.clone()], vec![checkin], vec![session], vec![capsule]));
    db.save_journal_record(&entry.edited("rewritten", Utc::now())).unwrap();
    let entries: Vec<JournalEntry> = db.load_journal_records().unwrap();
    assert_eq!((entries.len(), entries[0].body.as_str()), (1, "rewritten"));
    db.delete_journal_record::<JournalEntry>(&RecordId::new("e1")).unwrap();
    assert!(db.load_journal_records::<JournalEntry>().unwrap().is_empty());
    assert_eq!(db.load_journal_book().unwrap().checkins.len(), 1, "deleting an entry leaves the other kinds");
}

#[test]
fn journal_rows_never_reach_posts_documents_or_the_index() {
    let db = saved_db();
    let posts = db.load_posts().unwrap();
    let documents = db.load_all_documents().unwrap();
    let vectors = db.load_embeddings().unwrap();
    assert!(posts.iter().all(|p| !p.content.contains(SECRET)), "no journal text among posts (the stream, Insights and agents read these)");
    assert!(documents.iter().all(|d| !d.content.contains(SECRET)), "no journal text among vault documents");
    assert!(vectors.is_empty(), "nothing private is queued for embedding");
}

#[test]
fn unparseable_rows_are_skipped() {
    let db = saved_db();
    db.conn.execute("INSERT INTO mood_checkins (id, day, sort_at, record_json) VALUES ('bad', '2026-03-02', 'z', '{oops')", []).unwrap();
    assert_eq!(db.load_journal_book().unwrap().checkins.len(), 1);
}

#[test]
fn tables_are_created_idempotently() {
    let path = std::env::temp_dir().join(format!("simulacra-journal-{}.db", uuid::Uuid::new_v4()));
    Database::open(&path).unwrap().save_journal_record(&book_records().0).unwrap();
    let reopened = Database::open(&path).unwrap();
    assert_eq!(reopened.load_journal_book().unwrap().entries.len(), 1, "reopening keeps the rows");
    drop(reopened);
    let _ = std::fs::remove_file(&path);
}
