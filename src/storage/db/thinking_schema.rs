//! The thinking tools' tables (sb E1), created idempotently on every open:
//!
//! - `argument_maps`: one argument map per thread root, as JSON (nodes, edges, order).
//! - `credences`: one stated credence per post, its due date, and its resolution.
//! - `tension_pairs`: pairs the contradiction finder showed, by content hash: the AI's
//!   verdict and reason (a cache, so a pair is never classified twice) and the user's status.

pub(super) const CREATE_THINKING_TABLES: &str = r#"
                CREATE TABLE IF NOT EXISTS argument_maps (
                    root_id TEXT PRIMARY KEY,
                    map_json TEXT NOT NULL,
                    updated_at INTEGER NOT NULL
                );

                CREATE TABLE IF NOT EXISTS credences (
                    post_id TEXT PRIMARY KEY,
                    percent INTEGER NOT NULL,
                    sentence TEXT,
                    resolves_by TEXT,
                    note TEXT NOT NULL DEFAULT '',
                    outcome TEXT,
                    created_at TEXT NOT NULL,
                    resolved_at TEXT
                );

                CREATE INDEX IF NOT EXISTS idx_credences_due ON credences(resolves_by);

                CREATE TABLE IF NOT EXISTS tension_pairs (
                    pair_key TEXT PRIMARY KEY,
                    left_key TEXT NOT NULL,
                    right_key TEXT NOT NULL,
                    verdict TEXT,
                    reason TEXT NOT NULL DEFAULT '',
                    status TEXT NOT NULL DEFAULT 'open',
                    updated_at INTEGER NOT NULL
                );
                "#;
