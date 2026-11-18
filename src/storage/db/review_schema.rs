//! The spaced-repetition tables (sb C1), created idempotently on every open:
//! - `review_cards`: one row per card; the card kind, source and schedule as JSON, with the
//!   due day and network as columns for queries;
//! - `review_log`: one row per answer, append-only.

/// Table and index definitions.
pub(super) const CREATE_REVIEW: &str = r#"
                CREATE TABLE IF NOT EXISTS review_cards (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    kind_json TEXT NOT NULL,
                    source_json TEXT NOT NULL,
                    tags_json TEXT NOT NULL DEFAULT '[]',
                    created_at TEXT NOT NULL,
                    schedule_json TEXT NOT NULL,
                    due TEXT NOT NULL,
                    suspended INTEGER NOT NULL DEFAULT 0
                );

                CREATE INDEX IF NOT EXISTS idx_review_cards_network_due ON review_cards(network_id, due);

                CREATE TABLE IF NOT EXISTS review_log (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    card_id TEXT NOT NULL,
                    network_id TEXT NOT NULL,
                    reviewed_at TEXT NOT NULL,
                    review_day TEXT NOT NULL,
                    grade TEXT NOT NULL,
                    mode TEXT NOT NULL,
                    entry_json TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_review_log_day ON review_log(network_id, review_day);
                CREATE INDEX IF NOT EXISTS idx_review_log_card ON review_log(card_id);
                "#;
