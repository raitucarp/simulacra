//! The private tables of the Journal page (sb E3), created idempotently on every open. Each
//! row holds one record as JSON, with its id, journal day and a sort time as columns:
//! - `journal_entries`: daily journal entries;
//! - `mood_checkins`: mood and energy check-ins;
//! - `focus_sessions`: finished focus sessions;
//! - `time_capsules`: notes to the future self, with the day they open.
//!
//! These tables are read only by the Journal page: no query of posts, documents, the
//! embedding index or agents' context touches them.

/// Table and index definitions.
pub(super) const CREATE_JOURNAL: &str = r#"
                CREATE TABLE IF NOT EXISTS journal_entries (
                    id TEXT PRIMARY KEY,
                    day TEXT NOT NULL,
                    sort_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_journal_entries_day ON journal_entries(day);

                CREATE TABLE IF NOT EXISTS mood_checkins (
                    id TEXT PRIMARY KEY,
                    day TEXT NOT NULL,
                    sort_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_mood_checkins_day ON mood_checkins(day);

                CREATE TABLE IF NOT EXISTS focus_sessions (
                    id TEXT PRIMARY KEY,
                    day TEXT NOT NULL,
                    sort_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_focus_sessions_day ON focus_sessions(day);

                CREATE TABLE IF NOT EXISTS time_capsules (
                    id TEXT PRIMARY KEY,
                    day TEXT NOT NULL,
                    sort_at TEXT NOT NULL,
                    record_json TEXT NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_time_capsules_day ON time_capsules(day);
                "#;
