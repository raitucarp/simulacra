//! The spaced-repetition tables (sb C1, reshaped in sb C1b), created idempotently on every
//! open:
//! - `review_items`: one row per item anchored on a post or a vault passage, or one per
//!   connection map; the subject and schedule as JSON, with the network, the anchor key
//!   (`post:<id>` / `document:<id>`, empty for a map) and the due day as columns;
//! - `review_log`: one row per answer, append-only (its `card_id` column holds item ids).
//!
//! The retired `review_cards` table of standalone flashcards is carried over and dropped by
//! the user_version 5 migration (`versions.rs`).

/// Table and index definitions.
pub(super) const CREATE_REVIEW: &str = r#"
                CREATE TABLE IF NOT EXISTS review_items (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    kind TEXT NOT NULL,
                    anchor_key TEXT NOT NULL DEFAULT '',
                    subject_json TEXT NOT NULL,
                    tags_json TEXT NOT NULL DEFAULT '[]',
                    created_at TEXT NOT NULL,
                    schedule_json TEXT NOT NULL,
                    due TEXT NOT NULL,
                    suspended INTEGER NOT NULL DEFAULT 0,
                    retired INTEGER NOT NULL DEFAULT 0
                );

                CREATE INDEX IF NOT EXISTS idx_review_items_network_due ON review_items(network_id, due);
                CREATE INDEX IF NOT EXISTS idx_review_items_anchor ON review_items(network_id, anchor_key);

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
