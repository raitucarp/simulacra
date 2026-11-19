//! Rows written before the move from `chrono` to `jiff` still load, and rows written now keep
//! the same timestamp text, so old and new rows sort together.

use crate::domain::journal::MoodCheckin;
use crate::domain::Post;
use crate::storage::db::Database;

/// A mood check-in as the chrono-based app stored it: local offset, millisecond fraction.
const LEGACY_CHECKIN: &str = r#"{"id":"m1","at":"2026-03-02T21:30:00.250+07:00","day":"2026-03-02","affect":{"valence":0.5,"arousal":-0.25},"energy":3,"sleep_hours":null,"tags":["calm"],"note":"","wants_support":false}"#;
/// Its ordering column, in the same form.
const LEGACY_SORT_AT: &str = "2026-03-02T21:30:00.250+07:00";
/// A post time as chrono's `to_rfc3339` wrote it (100 ns clock resolution on Windows).
const LEGACY_POST_AT: &str = "2025-05-01T10:00:00.123456700+00:00";

#[test]
fn checkins_written_by_earlier_versions_load_and_save_unchanged() {
    let db = Database::in_memory().unwrap();
    let insert = "INSERT INTO mood_checkins (id, day, sort_at, record_json) VALUES ('m1', '2026-03-02', ?1, ?2)";
    db.conn.execute(insert, [LEGACY_SORT_AT, LEGACY_CHECKIN]).unwrap();
    let checkins: Vec<MoodCheckin> = db.load_journal_records().unwrap();
    let at = &checkins[0].at;
    assert_eq!((at.hour(), at.minute(), at.offset().seconds()), (21, 30, 7 * 3_600), "the local offset survives");
    assert_eq!(checkins[0].day, jiff::civil::date(2026, 3, 2));
    db.save_journal_record(&checkins[0]).unwrap();
    let select = "SELECT sort_at, record_json FROM mood_checkins";
    let row: (String, String) = db.conn.query_row(select, [], |r| Ok((r.get(0)?, r.get(1)?))).unwrap();
    assert_eq!(row, (LEGACY_SORT_AT.to_owned(), LEGACY_CHECKIN.to_owned()), "re-saved byte for byte");
}

#[test]
fn post_times_written_by_earlier_versions_load_and_save_unchanged() {
    let db = Database::in_memory().unwrap();
    db.save_post(&Post::create("p1", "net", "u1", "An old thought").unwrap()).unwrap();
    db.conn.execute("UPDATE posts SET created_at = ?1", [LEGACY_POST_AT]).unwrap();
    let post = db.load_posts().unwrap().pop().unwrap();
    let expected = crate::time::utc(2025, 5, 1, 10, 0, 0) + jiff::SignedDuration::from_nanos(123_456_700);
    assert_eq!(post.created_at, expected);
    db.save_post(&post).unwrap();
    let stored: String = db.conn.query_row("SELECT created_at FROM posts", [], |r| r.get(0)).unwrap();
    assert_eq!(stored, LEGACY_POST_AT);
}
